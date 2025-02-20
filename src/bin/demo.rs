use std::sync::Arc;

use fp::{
    aux::{servers, Setup},
    client::Client,
    common::{Amount, Nonce, Receipt, Transfer},
    validator::Committee,
};
use futures::future::join_all;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let num_authrorities = 7;
    let num_accounts = 9;
    let setup = fp::aux::setup(num_authrorities, num_accounts);

    let (validators, shutdown, handle) = servers(&setup).await;
    let http = Arc::new(reqwest::Client::new());
    let validators = validators
        .into_iter()
        .map(|url| Client::new(url, http.clone()))
        .collect::<Vec<_>>();

    println!(
        "Valid transfer: {}",
        transfer(&setup, &validators, 0, 1, 42, 0).await
    );
    println!(
        "Reused nonce  : {}",
        transfer(&setup, &validators, 0, 1, 42, 0).await
    );
    println!(
        "Invalid nonce : {}",
        transfer(&setup, &validators, 2, 3, 42, 1).await
    );
    println!(
        "Wrong balance : {}",
        transfer(&setup, &validators, 4, 5, 1001, 0).await
    );

    // Only one of the two attempted simultaneous spends succeeds
    join_all([
        {
            let setup = setup.clone();
            let validators = validators.clone();
            tokio::spawn(async move {
                println!(
                    "Double spend 1: {}",
                    transfer(&setup, &validators, 6, 7, 100, 0).await
                );
            })
        },
        {
            let setup = setup.clone();
            let validators = validators.clone();
            tokio::spawn(async move {
                println!(
                    "Double spend 2: {}",
                    transfer(&setup, &validators, 6, 8, 100, 0).await
                );
            })
        },
    ])
    .await;

    shutdown();
    handle.await;
}

async fn transfer(
    setup: &Setup,
    validators: &[Client],
    src: usize,
    dst: usize,
    amt: u64,
    seq: u64,
) -> &'static str {
    let key = &setup.clients[src].0;

    let src = setup.clients[src].1.clone();
    let dst = setup.clients[dst].1.clone();

    let tx = Transfer::new(src.clone(), dst.clone(), Amount(amt), Nonce(seq))
        .sign(&key.0)
        .expect("tx");
    let mut acks = Vec::with_capacity(validators.len());
    for validator in validators {
        if let Ok(ack) = validator.accept(&tx).await {
            if tx.ack(&ack).is_ok() {
                acks.push(ack);
            }
        }
    }

    let rx = Receipt { tx, acks };

    let committee = Committee::new(setup.committee.clone());
    let _ = committee.check(&rx); // offline receipt verification against committee

    for validator in validators {
        let _ = validator.confirm(&rx).await;
    }

    for validator in validators {
        let confirmed = validator.lookup(&src).await.unwrap().confirmed;
        if let Some(receipt) = confirmed.iter().find(|r| r.tx.seq == rx.tx.seq) {
            if &rx != receipt {
                return "invalid receipt";
            }
        } else {
            return "missing receipt";
        }

        let src_balance = validator.lookup(&src).await.unwrap().bal.0;
        let dst_balance = validator.lookup(&dst).await.unwrap().bal.0;
        if dst_balance - src_balance != 2 * amt as i64 {
            return "invalid balance";
        }
    }

    "OK"
}

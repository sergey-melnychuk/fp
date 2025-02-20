use std::sync::Arc;

use fp::{
    aux::servers,
    client::Client,
    common::{Amount, Nonce, Receipt, Transfer},
    error::Result,
    validator::Committee,
};

#[tokio::main]
async fn main() -> Result<()> {
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

    let key = &setup.clients[0].0;

    let src = setup.clients[0].1.clone();
    let dst = setup.clients[1].1.clone();

    let tx = Transfer::new(src.clone(), dst.clone(), Amount(100), Nonce(0))
        .sign(&key.0)
        .expect("tx");

    println!("TX: {}\n---", serde_json::to_string_pretty(&tx).unwrap());

    let mut acks = Vec::with_capacity(validators.len());
    for validator in &validators {
        if let Ok(ack) = validator.accept(&tx).await {
            println!("ACK: {}", serde_json::to_string_pretty(&ack).unwrap());
            tx.ack(&ack)?;
            acks.push(ack);
        }
    }

    let rx = Receipt { tx, acks };
    println!("---\nRX: {}", serde_json::to_string_pretty(&rx).unwrap());

    let committee = Committee::new(setup.committee.clone());
    committee.check(&rx)?; // offline receipt verification against committee
    println!("---\nTX OK");

    for validator in &validators {
        let id = validator.id().await?;
        let id = hex::encode(id.0);

        validator.confirm(&rx).await?;
        println!("TX confirmed by {id}");
    }
    println!("---");

    let src_exp_amt: i64 = 200 - 100;
    let dst_exp_amt: i64 = 200 + 100;
    let src_exp_seq: u64 = 1;
    for validator in &validators {
        let id = validator.id().await?;
        let id = hex::encode(id.0);

        let src_state = validator.lookup(&src).await?;
        let src_amt = src_state.bal.0;
        let dst_amt = validator.lookup(&dst).await?.bal.0;
        assert_eq!(src_amt, src_exp_amt, "src balance must match");
        assert_eq!(dst_amt, dst_exp_amt, "dst balance must match");

        let src_seq = src_state.seq.0;
        assert_eq!(src_seq, src_exp_seq, "seq must match");

        let src_rx = src_state.confirmed.last().unwrap();
        assert_eq!(src_rx, &rx, "receipt must match");

        println!("seq:{src_exp_seq} src:{src_exp_amt} dst:{dst_exp_amt} | {id}");
    }

    shutdown();
    handle.await;
    Ok(())
}

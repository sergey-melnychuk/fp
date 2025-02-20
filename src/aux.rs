use std::future::Future;

use futures::future::join_all;
use rand::{rngs::OsRng, TryRngCore};
use serde::{Deserialize, Serialize};

use crate::{
    common::{Balance, PubKey, Secret},
    server::serve,
    validator::{Committee, Validator},
};

fn random<const N: usize>() -> [u8; N] {
    let mut ret = [0u8; N];
    OsRng.try_fill_bytes(&mut ret).expect("random");
    ret
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Setup {
    pub committee: Vec<PubKey>,
    pub validators: Vec<Secret>,
    pub clients: Vec<(Secret, PubKey)>,
}

pub fn setup(num_validators: usize, num_clients: usize) -> Setup {
    let (validators, committee): (Vec<_>, Vec<_>) = (0..num_validators)
        .map(|_| random())
        .filter_map(|sk| {
            crate::crypto::public(&sk)
                .ok()
                .map(|pk| (Secret(sk), PubKey(pk)))
        })
        .unzip();
    let clients = (0..num_clients)
        .map(|_| random())
        .filter_map(|sk| {
            crate::crypto::public(&sk)
                .ok()
                .map(|pk| (Secret(sk), PubKey(pk)))
        })
        .collect::<Vec<_>>();
    Setup {
        committee,
        validators,
        clients,
    }
}

pub fn input() -> String {
    use std::io::BufRead;
    let mut acc = String::new();
    let mut line = String::new();
    let mut stdin = std::io::stdin().lock();
    while stdin.read_line(&mut line).unwrap() != 0 {
        let line = std::mem::take(&mut line);
        acc.push_str(&line);
    }
    acc
}

pub async fn servers(setup: &Setup) -> (Vec<String>, impl FnOnce(), impl Future<Output = ()>) {
    let committee = Committee::new(setup.committee.clone());
    let genesis = setup
        .clients
        .iter()
        .map(|(_, pubkey)| (pubkey.clone(), Balance(200)))
        .collect::<Vec<_>>();

    let mut addresses = Vec::with_capacity(setup.validators.len());
    let mut triggers = Vec::with_capacity(setup.validators.len());
    let mut futures = Vec::with_capacity(setup.validators.len());
    for key in setup.validators.iter() {
        let validator = Validator::new(key.clone(), committee.clone(), genesis.clone());
        let (addr, f, h) = serve("127.0.0.1:0", validator).await;
        addresses.push(format!("http://{addr}"));
        triggers.push(f);
        futures.push(h);
    }

    let shutdown = move || {
        for f in triggers {
            f();
        }
    };

    let all = async {
        join_all(futures).await;
    };
    (addresses, shutdown, all)
}

use std::collections::{HashMap, HashSet};

use crate::{
    common::{Ack, Address, Genesis, Nonce, PubKey, Receipt, Secret, Signature, State, Transfer},
    error::{Error, Result},
};

#[derive(Clone)]
pub struct Committee {
    validators: HashSet<PubKey>,
}

impl Committee {
    pub fn new(validators: Vec<PubKey>) -> Self {
        Self {
            validators: validators.into_iter().collect(),
        }
    }

    pub fn check(&self, receipt: &Receipt) -> Result<()> {
        let hash = receipt.tx.hash()?;
        let acks = receipt
            .acks
            .iter()
            .filter(|ack| self.validators.contains(&ack.key))
            .filter(|ack| crate::crypto::check(&hash, &ack.key.0, &ack.sig.0).is_ok())
            .collect::<HashSet<_>>();

        let f = (self.validators.len() - 1) / 3;
        let threshold = self.validators.len() - f;
        if acks.len() < threshold {
            return Err(Error::Unconfirmed {
                expected: threshold,
                confirmed: acks.len(),
            });
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Validator {
    key: Secret,
    committee: Committee,
    state: HashMap<Address, State>,
}

impl Validator {
    pub fn new(key: Secret, committee: Committee, genesis: Genesis) -> Self {
        Self {
            key,
            committee,
            state: genesis
                .into_iter()
                .map(|(addr, bal)| {
                    (
                        addr,
                        State {
                            bal,
                            seq: Nonce(0),
                            pending: None,
                            confirmed: vec![],
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn id(&self) -> Result<PubKey> {
        crate::crypto::public(&self.key.0).map(PubKey)
    }

    pub fn lookup(&self, address: &Address) -> Option<State> {
        self.state.get(address).cloned()
    }

    pub fn accept(&mut self, tx: Transfer) -> Result<Ack> {
        tx.check()?;

        let Some(acc) = self.state.get_mut(&tx.src) else {
            return Err(Error::UnknownSender);
        };

        let pubkey = crate::crypto::public(&self.key.0)?;

        if let Some((pending, ack)) = acc.pending.as_ref() {
            if pending != &tx {
                return Err(Error::PendingMismatch);
            }
            return Ok(ack.clone());
        }

        if acc.seq.0 != tx.seq.0 {
            return Err(Error::NonceMismatch {
                expected: acc.seq.0,
                received: tx.seq.0,
            });
        }

        if acc.bal.0 < tx.amt.0 as i64 {
            return Err(Error::InsufficientFunds {
                requested: tx.amt.0 as i64,
                available: acc.bal.0,
            });
        }

        let hash = tx.hash()?;
        let sig = crate::crypto::sign(&hash, &self.key.0)?;
        let ack = Ack {
            key: PubKey(pubkey),
            sig: Signature(sig),
        };

        acc.pending = Some((tx, ack.clone()));
        Ok(ack)
    }

    pub fn confirm(&mut self, receipt: &Receipt) -> Result<()> {
        self.committee.check(receipt)?;

        {
            let Some(src) = self.state.get_mut(&receipt.tx.src) else {
                return Err(Error::UnknownSender);
            };
            src.bal.0 = src.bal.0.saturating_sub(receipt.tx.amt.0 as i64);

            src.confirmed.push(receipt.clone());
            src.pending = None;
            src.seq.0 += 1;
        }

        {
            let dst = self.state.entry(receipt.tx.dst.clone()).or_default();
            dst.bal.0 = dst.bal.0.saturating_add(receipt.tx.amt.0 as i64);
        }

        Ok(())
    }
}

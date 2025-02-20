use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::error::{Error, Result};

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Secret(#[serde_as(as = "serde_with::hex::Hex")] pub [u8; 32]);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hashed(#[serde_as(as = "serde_with::hex::Hex")] pub [u8; 32]);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PubKey(#[serde_as(as = "serde_with::hex::Hex")] pub [u8; 33]);

pub type Address = PubKey;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Balance(pub i64);

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Amount(pub u64);

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Nonce(pub u64);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature(#[serde_as(as = "serde_with::hex::Hex")] pub [u8; 64]);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transfer {
    pub src: Address,
    pub dst: Address,
    pub amt: Amount,
    pub seq: Nonce,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig: Option<Signature>,
}

impl Transfer {
    pub fn new(src: Address, dst: Address, amt: Amount, seq: Nonce) -> Self {
        Self {
            src,
            dst,
            amt,
            seq,
            sig: None,
        }
    }

    pub fn hash(&self) -> Result<[u8; 32]> {
        crate::crypto::hash(&[
            &self.src.0,
            &self.dst.0,
            &self.amt.0.to_be_bytes(),
            &self.seq.0.to_be_bytes(),
        ])
    }

    pub fn sign(self, sk: &[u8; 32]) -> Result<Self> {
        let pk = crate::crypto::public(sk)?;
        if self.src.0 != pk {
            return Err(Error::WrongSigningKey);
        }
        let h = self.hash()?;
        let sig = crate::crypto::sign(&h, sk)?;
        let signed = Self {
            sig: Some(Signature(sig)),
            ..self
        };
        signed.check()?;
        Ok(signed)
    }

    pub fn check(&self) -> Result<()> {
        let h = self.hash()?;
        if let Some(sig) = self.sig.as_ref() {
            crate::crypto::check(&h, &self.src.0, &sig.0)?;
        } else {
            return Err(Error::SignatureMissing);
        }
        Ok(())
    }

    pub fn ack(&self, ack: &Ack) -> Result<()> {
        let h = self.hash()?;
        crate::crypto::check(&h, &ack.key.0, &ack.sig.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ack {
    pub key: PubKey,
    pub sig: Signature,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Receipt {
    pub tx: Transfer,
    pub acks: Vec<Ack>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub bal: Balance,
    pub seq: Nonce,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<(Transfer, Ack)>,
    pub confirmed: Vec<Receipt>,
}

pub type Genesis = Vec<(Address, Balance)>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip() {
        let sk = [0xCCu8; 32];
        let pk = crate::crypto::public(&sk).expect("pubkey");

        let tx = Transfer::new(PubKey(pk), PubKey([0xBBu8; 33]), Amount(42), Nonce(12345));

        let json = serde_json::json!({
            "src": "02b95c249d84f417e3e395a127425428b540671cc15881eb828c17b722a53fc599",
            "dst": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "amt": 42,
            "seq": 12345
        });

        assert_eq!(serde_json::to_value(&tx).expect("json"), json);

        assert_eq!(serde_json::from_value::<Transfer>(json).expect("json"), tx);

        let tx = tx.sign(&sk).expect("signed");

        let json = serde_json::json!({
            "src": "02b95c249d84f417e3e395a127425428b540671cc15881eb828c17b722a53fc599",
            "dst": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "amt": 42,
            "seq": 12345,
            "sig": "15a7fcfded1bc011d4eb1dbdb01541853d8e431c42c22dcd062042f5a88db08c418d1a314fb44b584825962b8893d94d67aa7376c465d8d503f811d8bf3d0e46"
        });

        assert_eq!(serde_json::to_value(&tx).expect("json"), json);

        assert_eq!(serde_json::from_value::<Transfer>(json).expect("json"), tx);
    }
}

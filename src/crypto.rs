use std::{io::Write, sync::OnceLock};

use crypto_hash::{Algorithm, Hasher};
use secp256k1::{ecdsa::Signature, All, Message, PublicKey, Secp256k1, SecretKey};

use crate::error::Result;

static SECP: OnceLock<Secp256k1<All>> = OnceLock::new();

fn secp() -> &'static Secp256k1<All> {
    SECP.get_or_init(Secp256k1::new)
}

pub fn sign(h: &[u8; 32], sk: &[u8; 32]) -> Result<[u8; 64]> {
    let seckey = SecretKey::from_byte_array(sk)?;
    let msg = Message::from_digest(*h);
    let sig = secp().sign_ecdsa(&msg, &seckey);
    Ok(sig.serialize_compact())
}

pub fn check(h: &[u8; 32], pk: &[u8; 33], sig: &[u8; 64]) -> Result<()> {
    let pubkey = PublicKey::from_slice(pk)?;
    let sig = Signature::from_compact(sig)?;
    let msg = Message::from_digest(*h);
    secp().verify_ecdsa(&msg, &sig, &pubkey)?;
    Ok(())
}

pub fn public(sk: &[u8; 32]) -> Result<[u8; 33]> {
    let seckey = SecretKey::from_byte_array(sk)?;
    Ok(seckey.public_key(secp()).serialize())
}

pub fn hash(bytes: &[&[u8]]) -> Result<[u8; 32]> {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    for x in bytes {
        hasher.write_all(x)?;
    }
    let hash = hasher.finish();
    assert_eq!(hash.len(), 32);
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&hash);
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecdsa_sanity_check() {
        let h = [0xAAu8; 32];
        let sk = [0xBBu8; 32];
        let pk = public(&sk).expect("pubkey");

        let sig = sign(&h, &sk).expect("signed");
        assert!(check(&h, &pk, &sig).is_ok());
    }
}

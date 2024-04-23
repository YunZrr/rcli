use std::{fs, path::Path};

use crate::{get_buf, process_genpass, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub trait TextSign {
    fn sign(&self, data: String) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, data: String, sig: &[u8]) -> Result<bool>;
}

pub trait KeyGen {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

impl TextSign for Blake3 {
    fn sign(&self, data: String) -> Result<Vec<u8>> {
        let buf = blake3::keyed_hash(&self.key, data.as_bytes());
        Ok(buf.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, data: String, sig: &[u8]) -> Result<bool> {
        let buf = blake3::keyed_hash(&self.key, data.as_bytes());
        let hash = buf.as_bytes();
        Ok(hash == sig)
    }
}

impl KeyGen for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, 1, 1, 1, 1)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGen for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, data: String) -> Result<Vec<u8>> {
        let sig = self.key.sign(data.as_bytes());
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, data: String, sig: &[u8]) -> Result<bool> {
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(data.as_bytes(), &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[0..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Ed25519Signer { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Ed25519Verifier { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Ed25519Verifier::new(key))
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let buf = get_buf(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(buf)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(buf)?
        }
    };

    Ok(URL_SAFE_NO_PAD.encode(signed))
}

pub fn process_verify(input: &str, key: &str, sig: String, format: TextSignFormat) -> Result<bool> {
    let buf = get_buf(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(buf, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(buf, &sig)?
        }
    };

    Ok(verified)
}

pub fn process_keygen(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.key")?;
        let data = String::from("hello1");
        let sig = blake3.sign(data.clone())?;
        assert!(blake3.verify(data, &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = String::from("hello1");
        let sig = sk.sign(data.clone())?;
        println!("sign: {:?}", sig);
        assert!(pk.verify(data, &sig)?);
        Ok(())
    }
}

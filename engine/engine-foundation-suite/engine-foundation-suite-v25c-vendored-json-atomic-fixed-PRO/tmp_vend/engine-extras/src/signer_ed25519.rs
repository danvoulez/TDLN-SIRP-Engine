
use engine_core::providers::Signer;
use ed25519_dalek::{SigningKey, Signer as _};
use anyhow::Result;

pub struct Ed25519Signer {
    kid: String,
    sk: SigningKey,
}
impl Ed25519Signer {
    pub fn from_pkcs8_pem(kid:&str, pem:&str) -> Result<Self> {
        let sk = SigningKey::from_pkcs8_pem(pem)?;
        Ok(Self{ kid: kid.to_string(), sk })
    }
    pub fn from_bytes(kid:&str, bytes:&[u8]) -> Result<Self> {
        let sk = SigningKey::from_bytes(bytes.try_into().map_err(|_| anyhow::anyhow!("bad key len"))?);
        Ok(Self{ kid: kid.to_string(), sk })
    }
}
impl Signer for Ed25519Signer {
    fn sign(&self, msg:&[u8]) -> Option<Vec<u8>> { Some(self.sk.sign(msg).to_bytes().to_vec()) }
    fn kid(&self) -> Option<String> { Some(self.kid.clone()) }
}


use ed25519_dalek::{Signer, SigningKey};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fs;
use std::path::Path;

static SIGNER: Lazy<Mutex<Option<SigningKey>>> = Lazy::new(|| Mutex::new(None));

pub fn init_signer() {
    let mut guard = SIGNER.lock().unwrap();
    if guard.is_some() { return; }
    // Load from ENV seed (base64 32 bytes) or from file, else generate and persist.
    if let Ok(b64) = std::env::var("ENGINE_SIGNING_KEY_ED25519") {
        if let Ok(bytes) = base64::decode(b64) {
            if bytes.len()==32 {
                let seed: [u8;32] = bytes.clone().try_into().unwrap();
                *guard = Some(SigningKey::from_bytes(&seed));
                return;
            }
        }
    }
    let path = std::env::var("ENGINE_SIGNING_KEY_ED25519_FILE").unwrap_or_else(|_| "var/keys/ed25519.seed".to_string());
    if Path::new(&path).exists() {
        if let Ok(bytes) = fs::read(&path) {
            if bytes.len()==32 {
                let seed: [u8;32] = bytes.clone().try_into().unwrap();
                *guard = Some(SigningKey::from_bytes(&seed));
                return;
            }
        }
    }
    // generate new seed and persist
    let seed: [u8;32] = rand::random();
    fs::create_dir_all(Path::new(&path).parent().unwrap()).ok();
    let _ = fs::write(&path, &seed);
    *guard = Some(SigningKey::from_bytes(&seed));
}

pub fn sign_bytes(data: &[u8]) -> String {
    let guard = SIGNER.lock().unwrap();
    let sk = guard.as_ref().expect("Signer not initialized. Call init_signer().");
    let sig = sk.sign(data);
    format!("ed25519:{}", base64::encode(sig.to_bytes()))
}

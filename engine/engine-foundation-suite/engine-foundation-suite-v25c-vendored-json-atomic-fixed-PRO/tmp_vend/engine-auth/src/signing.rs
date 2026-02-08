use anyhow::{Result, Context};
use ed25519_dalek::{SigningKey, Signature, Signer};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;

use crate::grant::AccessGrant;

/// Sign an AccessGrant as per ADR-0001 flavor:
/// - set seal.sig = "" before hashing
/// - message = JSON✯Atomic(grant)
/// - digest = blake3(message)
/// - signature = ed25519(digest)
/// - seal.sig = base64(signature)
pub fn sign_grant(signing_key: &SigningKey, grant: &mut AccessGrant) -> Result<()> {
    // ensure we don't self-sign over an existing signature
    grant.seal.sig = String::new();

    let msg = engine_core::json_atomic::to_json_atomic_bytes(grant)
        .context("canonize grant")?;

    let digest = blake3::hash(&msg);
    let sig: Signature = signing_key.sign(digest.as_bytes());
    grant.seal.sig = B64.encode(sig.to_bytes());
    Ok(())
}

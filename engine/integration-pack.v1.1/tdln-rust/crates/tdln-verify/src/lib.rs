
use regex::Regex;
use tdln_receipt::Card;

#[derive(Debug)]
pub enum Verdict {
    Pass,
    Warn(&'static str),
    Fail(&'static str),
}

pub fn verify_rref_11(card: &Card) -> Verdict {
    // Basic fields
    if card.kind != "receipt.card.v1" { return Verdict::Fail("BAD_KIND"); }
    if card.realm != "trust" { return Verdict::Fail("BAD_REALM"); }
    match card.decision.as_str() {
        "ACK" | "ASK" | "NACK" | "RUNNING" => {},
        _ => return Verdict::Fail("BAD_DECISION"),
    }
    let re_handle = Regex::new(r"^https://cert\.tdln\.foundry/r/b3:[0-9a-f]{16,}$").unwrap();
    if !re_handle.is_match(&card.links.card_url) { return Verdict::Fail("BAD_LINK"); }

    // proof fields
    if card.proof.seal.alg != "ed25519-blake3" { return Verdict::Fail("BAD_SEAL"); }
    if card.proof.seal.kid.is_empty() || card.proof.seal.sig.is_empty() { return Verdict::Fail("BAD_SEAL"); }

    let re_cid = Regex::new(r"^cid:b3:[0-9a-f]{16,}$").unwrap();
    if !re_cid.is_match(&card.output_cid) { return Verdict::Fail("BAD_OUTPUT_CID"); }
    if card.proof.hash_chain.is_empty() { return Verdict::Fail("HASH_CHAIN_EMPTY"); }
    let has_output = card.proof.hash_chain.iter().any(|s| s.kind=="output" && s.cid==card.output_cid);
    if !has_output { return Verdict::Fail("HASH_CHAIN_INCOMPLETE"); }

    if card.decision=="ASK" || card.decision=="NACK" {
        if card.poi.as_ref().and_then(|p| p.get("present")).and_then(|v| v.as_bool()) != Some(true) {
            return Verdict::Fail("POI_MISSING"); }
    }

    // refs policy
    let re_canon = Regex::new(r"^https://registry\.tdln\.foundry/v1/objects/").unwrap();
    let re_tdln  = Regex::new(r"^tdln://objects/").unwrap();
    let mut warned = None;
    for r in &card.refs {
        if !re_cid.is_match(&r.cid) { return Verdict::Fail("REF_MISSING_CID"); }
        if r.hrefs.is_empty() { return Verdict::Fail("REF_NO_HREFS"); }
        let is_private = r.private.unwrap_or(false) || r.kind.to_lowercase().contains("private");
        let portable = r.hrefs.iter().any(|h| re_canon.is_match(h) || re_tdln.is_match(h));
        if is_private && !portable {
            warned.get_or_insert("PRIVATE_NO_PORTABLE"); }
        if !is_private && !portable {
            warned.get_or_insert("PUBLIC_NO_CANONICAL_OR_TDLN"); }
    }
    if let Some(code) = warned { Verdict::Warn(code) } else { Verdict::Pass }
}


use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use blake3::Hasher;

pub fn canonical_bytes_for_card(card: &tdln_receipt::Card) -> Vec<u8> {
    // Redact sig before canonicalization
    let mut c = card.clone();
    c.proof.seal.sig = String::new();
    serde_json::to_vec(&c).expect("serialize")
}

pub fn verify_seal(card: &tdln_receipt::Card, vk_b64: &str) -> bool {
    let bytes = canonical_bytes_for_card(card);
    let mut hasher = Hasher::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    let vk_bytes = base64::decode(vk_b64).ok();
    let sig_bytes = base64::decode(&card.proof.seal.sig).ok();
    if vk_bytes.is_none() || sig_bytes.is_none() { return false; }
    let vk = VerifyingKey::from_bytes(vk_bytes.unwrap().as_slice()).ok();
    let sig = Signature::from_bytes(&sig_bytes.unwrap());
    if vk.is_none().unwrap_or(true) { return false; }
    vk.unwrap().verify(digest.as_bytes(), &sig).is_ok()
}

pub fn sign_card(card: &mut tdln_receipt::Card, sk_b64: &str, kid: &str) {
    let bytes = canonical_bytes_for_card(card);
    let mut hasher = Hasher::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    let sk_bytes = base64::decode(sk_b64).expect("sk");
    let sk = SigningKey::from_bytes(&sk_bytes.try_into().expect("len32"));
    let sig = sk.sign(digest.as_bytes());
    card.proof.seal.alg = "ed25519-blake3".into();
    card.proof.seal.kid = kid.into();
    card.proof.seal.sig = base64::encode(sig.to_bytes());
}

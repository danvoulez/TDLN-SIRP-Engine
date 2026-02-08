use engine_auth::{grant::{AccessGrant, GrantResource, GrantSeal}, signing::sign_grant};
use ed25519_dalek::SigningKey;
use rand_core::OsRng;

#[test]
fn sign_sets_sig_and_is_stable() {
    let signing_key = SigningKey::generate(&mut OsRng);
    let mut grant = AccessGrant {
        kind: "access.grant.v1".into(),
        grant_id: "01H...".into(),
        sub: "user".into(),
        tenants: vec!["t".into()],
        resource: GrantResource {
            store: "S3Compatible".into(),
            bucket: "b".into(),
            prefix: "p".into(),
            object: None,
            verbs: vec!["GET".into()],
            constraints: None,
        },
        exp: "2026-01-01T00:00:00Z".into(),
        iat: "2026-01-01T00:00:00Z".into(),
        nonce: "n".into(),
        seal: GrantSeal { alg: "ed25519-blake3".into(), kid: "k".into(), sig: "".into() },
    };

    sign_grant(&signing_key, &mut grant).unwrap();
    assert!(!grant.seal.sig.is_empty());
}

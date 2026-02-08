#!/usr/bin/env python3
import argparse, json, hashlib, uuid, datetime, zipfile, os, sys

def canon(obj): 
    return json.dumps(obj, ensure_ascii=False, sort_keys=True, separators=(",",":")).encode("utf-8")
def b3demo(b: bytes) -> str:
    return "b3:" + hashlib.sha256(b).hexdigest()

def cmd_run(args):
    now = datetime.datetime.utcnow().replace(microsecond=0).isoformat()+"Z"
    policy = {"kind":"tdln","cid": args.policy_cid or "cid:b3:" + hashlib.sha256(b'policy').hexdigest()}
    inputs = {"policy": policy, "data":[]}
    manifest = {"realm":"trust","intent":"certify","inputs":inputs,"options":{"no_hitl":True,"offline_bundle":True},"ts":now}
    cid = b3demo(canon(manifest))
    did = "did:tdln:"+uuid.uuid4().hex[:16]
    card_url = f"https://cert.tdln.foundry/r/{cid}"
    print("DID:", did)
    print("RUN_CID:", cid)
    print("CARD_URL:", card_url)
    if args.bundle:
        os.makedirs(args.out, exist_ok=True)
        zp = os.path.join(args.out, "bundle.zip")
        card = {
            "kind":"receipt.card.v1",
            "realm":"trust",
            "decision":"RUNNING",
            "output_cid":"cid:b3:" + hashlib.sha256(b'out').hexdigest(),
            "proof":{"seal":{"alg":"ed25519-blake3","kid":"demo","sig":"DEMO"},"hash_chain":[{"kind":"input","cid":"cid:b3:"+hashlib.sha256(b'in').hexdigest()}]},
            "links":{"card_url": card_url},
            "refs":[]
        }
        with zipfile.ZipFile(zp,"w",zipfile.ZIP_DEFLATED) as z:
            z.writestr("run.manifest.json", json.dumps(manifest,ensure_ascii=False,sort_keys=True,indent=2))
            z.writestr("card.json", json.dumps(card,ensure_ascii=False,sort_keys=True,indent=2))
        print("Bundle:", zp)

def main():
    ap = argparse.ArgumentParser(prog="tdln-certify", description="demo CLI: give -> card_url -> bundle")
    sp = ap.add_subparsers(dest="cmd", required=True)

    run = sp.add_parser("run", help="produce card_url + optional bundle.zip")
    run.add_argument("--policy-cid", help="existing policy CID", default=None)
    run.add_argument("--bundle", action="store_true", help="emit bundle.zip")
    run.add_argument("--out", default="./runs/last", help="output dir")
    run.set_defaults(func=cmd_run)

    args = ap.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()

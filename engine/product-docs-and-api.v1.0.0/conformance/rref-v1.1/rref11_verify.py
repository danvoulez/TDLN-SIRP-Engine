#!/usr/bin/env python3
import sys, json, re

CID_RE = re.compile(r"^cid:b3:[0-9a-f]{16,}$")
RUN_HANDLE_RE = re.compile(r"^https://cert\.tdln\.foundry/r/b3:[0-9a-f]{16,}$")
CANON_PREFIX = "https://registry.tdln.foundry/v1/objects/"
TDLN_PREFIX = "tdln://objects/"
PRESIGN_PREFIX = "https://registry.tdln.foundry/v1/presign/"

def fail(code, msg): 
    return {"result":"FAIL","code":code,"msg":msg}

def warn(code, msg):
    return {"result":"WARN","code":code,"msg":msg}

def pass_ok(): 
    return {"result":"PASS"}

def _href_has_portable(hrefs):
    return any(h.startswith(CANON_PREFIX) or h.startswith(TDLN_PREFIX) for h in hrefs)

def verify_card(card):
    # Basic fields
    if card.get("kind") != "receipt.card.v1":
        return fail("BAD_KIND","kind must be receipt.card.v1")
    if card.get("realm") not in ("trust",):
        return fail("BAD_REALM","realm must be trust")
    if card.get("decision") not in ("ACK","ASK","NACK"):
        return fail("BAD_DECISION","decision must be ACK|ASK|NACK")
    links = card.get("links") or {}
    if not RUN_HANDLE_RE.match(links.get("card_url","")):
        return fail("BAD_LINK","links.card_url invalid")

    # proof
    proof = card.get("proof") or {}
    seal = (proof.get("seal") or {})
    if seal.get("alg") != "ed25519-blake3" or not seal.get("kid") or not seal.get("sig"):
        return fail("BAD_SEAL","seal invalid or missing fields")
    output_cid = card.get("output_cid")
    if not output_cid or not CID_RE.match(output_cid):
        return fail("BAD_OUTPUT_CID","output_cid missing/invalid")
    chain = proof.get("hash_chain") or []
    if not chain:
        return fail("HASH_CHAIN_EMPTY","hash_chain empty")
    if not any(step.get("cid")==output_cid and step.get("kind")=="output" for step in chain):
        return fail("HASH_CHAIN_INCOMPLETE","output_cid not present as output in hash_chain")

    # ASK/NACK needs PoI.present
    if card["decision"] in ("ASK","NACK"):
        poi = card.get("poi") or {}
        if not poi.get("present", False):
            return fail("POI_MISSING","ASK/NACK must include PoI.present=true")

    # Refs: CID-first + resolver policy
    refs = card.get("refs") or []
    warns = []
    for ref in refs:
        if not CID_RE.match(ref.get("cid","")):
            return fail("REF_MISSING_CID","ref without valid cid")
        hrefs = ref.get("hrefs") or []
        if not hrefs:
            return fail("REF_NO_HREFS","ref without hrefs")
        is_private = bool(ref.get("private")) or "private" in (ref.get("kind") or "").lower()
        if is_private:
            # Private: may start presigned, but MUST include a portable resolver somewhere
            if not _href_has_portable(hrefs):
                warns.append("PRIVATE_NO_PORTABLE")
        else:
            # Public: MUST include canonical OR tdln portable
            if not _href_has_portable(hrefs):
                warns.append("PUBLIC_NO_CANONICAL_OR_TDLN")
    if warns:
        return warn("|".join(sorted(set(warns))), "one or more refs missing portable resolvers")
    return pass_ok()

def main():
    if len(sys.argv) < 2:
        print("usage: rref11_verify.py <card.json>"); sys.exit(2)
    with open(sys.argv[1], "r", encoding="utf-8") as f:
        card = json.load(f)
    print(json.dumps(verify_card(card), ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()

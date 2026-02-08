from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse
from pydantic import BaseModel
import json, hashlib, uuid, datetime

app = FastAPI(title="TDLN Certify API (demo)")

def canon(obj): 
    return json.dumps(obj, ensure_ascii=False, sort_keys=True, separators=(",",":")).encode("utf-8")

def b3demo(b: bytes) -> str:
    return "b3:" + hashlib.sha256(b).hexdigest()

class RunRequest(BaseModel):
    realm: str
    intent: str
    inputs: dict
    options: dict | None = None
    metadata: dict | None = None

@app.post("/v1/run")
def run(req: RunRequest):
    if req.realm not in ("trust","chip"):
        raise HTTPException(400, "realm must be 'trust' or 'chip' in this demo")
    did = ("did:tdln:" if req.realm=="trust" else "did:chip:") + uuid.uuid4().hex[:16]
    now = datetime.datetime.utcnow().replace(microsecond=0).isoformat()+"Z"
    run_manifest = {"realm": req.realm, "intent": req.intent, "inputs": req.inputs, "options": req.options or {}, "ts": now}
    run_cid = b3demo(canon(run_manifest))
    card_url = f"https://cert.tdln.foundry/r/{run_cid}"
    body = {
        "did": did,
        "run_cid": run_cid,
        "links": {"card_url": card_url},
        "status": "RUNNING",
        "receipt_preview": {"accepted_at": now, "issuer": "did:tdln:foundry:m1", "signature": "ed25519:DEMO"}
    }
    return JSONResponse(status_code=201, content=body)

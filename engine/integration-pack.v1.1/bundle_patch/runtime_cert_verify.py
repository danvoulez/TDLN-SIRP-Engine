#!/usr/bin/env python3
import sys, json, datetime
def fail(c): print(json.dumps({"result":"FAIL","code":c})); exit(2)
cert = json.load(open(sys.argv[1]))
if cert.get("kind")!="runtime.cert.v1": fail("BAD_KIND")
vu = cert.get("valid_until")
try:
    from datetime import datetime, timezone
    vu = datetime.fromisoformat(vu.replace("Z","+00:00"))
    if vu <= datetime.now(timezone.utc): fail("EXPIRED")
except Exception:
    fail("BAD_TIME")
print(json.dumps({"result":"PASS"}))

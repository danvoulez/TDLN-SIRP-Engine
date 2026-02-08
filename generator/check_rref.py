#!/usr/bin/env python3
import sys, json, re
data = json.load(open(sys.argv[1]))
links = data.get("links", {}) or {}
url = links.get("card_url") or links.get("url") or ""
pat = re.compile(r"^https://cert\.tdln\.foundry/r/b3:[0-9a-f]{16,}$")
assert pat.match(url), f"RREF FAIL: links.card_url/url inválido -> {url}"
print("RREF PASS:", url)

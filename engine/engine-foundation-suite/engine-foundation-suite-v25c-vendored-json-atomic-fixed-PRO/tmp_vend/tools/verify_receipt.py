#!/usr/bin/env python3
import argparse, json, sys, os
PRIMARY={'engine.auth.role.v1','engine.required.components_nonempty.v1','engine.version.semver_like.v1'}
ap=argparse.ArgumentParser(); ap.add_argument('--receipts', required=True); args=ap.parse_args()
if not os.path.exists(args.receipts): sys.exit('receipts file not found')
last=None
for line in open(args.receipts,'r',encoding='utf-8'):
    line=line.strip(); 
    if not line: continue
    last=json.loads(line)
if not last: sys.exit('no receipts')
decision=last.get('decision')
pds={p.get('id'):p.get('decision') for p in last.get('policy_decisions',[])}
out=last.get('output',{})
out_cid=last.get('output.cid') or out.get('cid'); out_did=last.get('output.did') or out.get('did')
missing=[]
if decision not in ('Allow','Doubt','Deny'): missing.append('decision')
for pol in PRIMARY:
    if pol not in pds: missing.append('policy:'+pol)
if not out_cid: missing.append('output.cid')
if not out_did: missing.append('output.did')
if not last.get('proof',{}).get('hash_chain'): missing.append('proof.hash_chain')
if missing: print('FAIL', *missing, sep=' | '); sys.exit(1)
print('OK', decision, out_cid, out_did); sys.exit(0)

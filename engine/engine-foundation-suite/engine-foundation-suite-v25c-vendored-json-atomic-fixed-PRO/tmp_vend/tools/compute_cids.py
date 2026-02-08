#!/usr/bin/env python3
import argparse, json, os
from pathlib import Path
try:
    from blake3 import blake3
except Exception:
    from hashlib import blake2s as blake3
IGNORES={'.git','target','__pycache__','.DS_Store'}
def iter_files(root: Path):
    if root.is_file(): yield root; return
    for p in sorted(root.rglob('*')):
        if any(part in IGNORES for part in p.parts): continue
        if p.is_file(): yield p
def b3_cid_for_path(path: Path) -> str:
    h = blake3()
    if path.is_file():
        with open(path,'rb') as f:
            while True:
                c=f.read(1<<20)
                if not c: break
                h.update(c)
    else:
        for f in iter_files(path):
            rel = f.relative_to(path).as_posix().encode('utf-8')
            h.update(rel)
            with open(f,'rb') as fh:
                while True:
                    c=fh.read(1<<20)
                    if not c: break
                    h.update(c)
    return f"b3:{h.hexdigest()}"
ap=argparse.ArgumentParser()
ap.add_argument('--manifest', required=True)
ap.add_argument('--out', required=True)
ap.add_argument('--set', dest='sets', action='append', default=[], help='components[NAME]=/path')
args=ap.parse_args()
data=json.load(open(args.manifest,'r',encoding='utf-8'))
comps=data.get('product',{}).get('components',[])
idx={c.get('name'):i for i,c in enumerate(comps)}
for item in args.sets:
    assert item.startswith('components[') and ']=' in item, '--set syntax'
    name=item.split('components[')[1].split(']')[0]
    path=Path(item.split(']=',1)[1]).resolve()
    if name not in idx: raise SystemExit(f'component not in manifest: {name}')
    comps[idx[name]]['cid']=b3_cid_for_path(path)
json.dump(data, open(args.out,'w',encoding='utf-8'), ensure_ascii=False, indent=2)
print('Wrote', args.out)

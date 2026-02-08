#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TS="$(date -u +%Y%m%dT%H%M%SZ)"

mkdir -p "$ROOT/reports" "$ROOT/qr" "$ROOT/compliance" "$ROOT/docs" "$ROOT/keys"

echo "==> 1/7 Verify FILELIST.json integrity"
python3 - "$ROOT" "$ROOT/FILELIST.json" <<'PY'
import hashlib, json, os, sys
root, filelist_path = sys.argv[1], sys.argv[2]
data = json.load(open(filelist_path, "r", encoding="utf-8"))
files = data.get("files") or []
missing = []
mismatches = []
checked = 0
for e in files:
    path = e.get("path")
    want = (e.get("sha256") or "").strip().lower()
    if not path or not want:
        continue
    full = os.path.join(root, path)
    if not os.path.isfile(full):
        missing.append(path)
        continue
    h = hashlib.sha256()
    with open(full, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    got = h.hexdigest()
    checked += 1
    if got != want:
        mismatches.append((path, want, got))
if missing or mismatches:
    print(f"FAIL: checked={checked} missing={len(missing)} mismatches={len(mismatches)}")
    for path, want, got in mismatches[:25]:
        print(f"  {path}\n    want {want}\n    got  {got}")
    sys.exit(1)
print(f"OK: checked {checked} file(s) against FILELIST.json")
PY

echo "==> 2/7 Test runner"
LOG="$ROOT/reports/test_all.$TS.log"
START_EPOCH="$(date -u +%s)"
if [[ -x "$ROOT/test_all.sh" ]]; then
  # Default: non-intrusive. For full run (Rust/JS/remote), call: FULL=1 bash make_release_artifacts.sh
  if [[ "${FULL:-0}" = "1" ]]; then
    "$ROOT/test_all.sh" --all | tee "$LOG"
  else
    "$ROOT/test_all.sh" | tee "$LOG"
  fi
else
  echo "WARN: test_all.sh não encontrado/executável; pulando."
  : > "$LOG"
fi
END_EPOCH="$(date -u +%s)"

echo "==> 3/7 reports/summary.json"
python3 - "$LOG" "$START_EPOCH" "$END_EPOCH" "$TS" > "$ROOT/reports/summary.json" <<'PY'
import json, re, sys
log_path, start_epoch, end_epoch, ts = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
txt = open(log_path, "r", encoding="utf-8", errors="replace").read()
m = re.search(r"Total:\\s*(\\d+)\\s*Pass:\\s*(\\d+)\\s*Fail:\\s*(\\d+)\\s*Skip:\\s*(\\d+)", txt)
tot = {"total": None, "pass": None, "fail": None, "skip": None}
if m:
    tot = {"total": int(m.group(1)), "pass": int(m.group(2)), "fail": int(m.group(3)), "skip": int(m.group(4))}
out = {
    "schema": "tdln.release.summary@v1",
    "ts": ts,
    "duration_seconds": max(0, end_epoch - start_epoch),
    "totals": tot,
    "log": log_path.split("/")[-1],
}
print(json.dumps(out, indent=2, sort_keys=True))
PY

echo "==> 4/7 MEGA_SHA256SUMS.txt (advisory)"
# NOTE: Exclude MEGA_SHA256SUMS.txt itself to avoid self-referential checksum.
find "$ROOT" -type f \
  ! -path "$ROOT/.git/*" \
  ! -path "$ROOT/**/target/*" \
  ! -path "$ROOT/**/node_modules/*" \
  ! -name "MEGA_SHA256SUMS.txt" \
  -exec shasum -a 256 {} \; | sort -k2 > "$ROOT/MEGA_SHA256SUMS.txt"

echo "==> 5/7 Sign MEGA_SHA256SUMS.txt (ed25519 over blake3 digest)"
PK="$ROOT/keys/mega_ed25519_pk.pem"
b3sum "$ROOT/MEGA_SHA256SUMS.txt" | awk '{print $1}' > "$ROOT/MEGA_SHA256SUMS.b3"

python3 - "$ROOT/MEGA_SHA256SUMS.b3" "$PK" "$ROOT/MEGA_SHA256SUMS.sig" "$ROOT/MEGA_SHA256SUMS.sig.b64" <<'PY'
import base64, binascii, os, sys

from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

b3_hex_path, pk_path, sig_path, sig_b64_path = sys.argv[1:5]

digest_hex = open(b3_hex_path, "r", encoding="utf-8").read().strip()
digest = binascii.unhexlify(digest_hex)

signing_sk_path = os.environ.get("SIGNING_SK")
if signing_sk_path:
    sk_bytes = open(signing_sk_path, "rb").read()
    sk = serialization.load_pem_private_key(sk_bytes, password=None)
else:
    # Ephemeral key: do NOT write private key into the bundle by default.
    sk = Ed25519PrivateKey.generate()

pk = sk.public_key()
open(pk_path, "wb").write(
    pk.public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo,
    )
)

sig = sk.sign(digest)
open(sig_path, "wb").write(sig)
open(sig_b64_path, "w", encoding="utf-8").write(base64.b64encode(sig).decode("ascii") + "\n")
print("OK: signature written (public key exported to bundle)")
PY

echo "==> 6/7 QR do handle curto (qr/last_run.png)"
CARD="$ROOT/template/examples/card.sample.json"
if [[ -f "$CARD" ]]; then
  URL="$(python3 -c 'import json; d=json.load(open("'"$CARD"'")); links=d.get("links",{}) or {}; print(links.get("card_url") or links.get("url") or "")')"
  if [[ -n "$URL" ]]; then
    PBM="$ROOT/qr/last_run.pbm"
    python3 - "$URL" "$PBM" <<'PY'
# Pure-Python QR generator (based on Nayuki qrcodegen; public domain)
import sys

text = sys.argv[1]
out_path = sys.argv[2]

def _get_bit(x, i): return (x >> i) & 1

class QrCode:
    def __init__(self, ver, size, data):
        self.version = ver
        self.size = size
        self.modules = [[False]*size for _ in range(size)]
        self.is_function = [[False]*size for _ in range(size)]
        self._draw_function_patterns()
        self._draw_codewords(data)
        self._apply_best_mask()

    @staticmethod
    def encode_text(s):
        # Byte mode only (UTF-8)
        data = list(s.encode("utf-8"))
        # Pick version 3 (29x29) which is enough for our short URL.
        ver = 3
        bb = _BitBuffer()
        bb.append_bits(0b0100, 4)  # byte mode
        bb.append_bits(len(data), 8)
        for b in data: bb.append_bits(b, 8)
        bb.append_bits(0, 4)
        while len(bb) % 8 != 0: bb.append_bits(0, 1)
        # Pad to 44 codewords (version 3-L)
        cw = bb.to_bytes()
        while len(cw) < 44:
            cw.append(0xEC); 
            if len(cw) < 44: cw.append(0x11)
        # ECC (version 3-L: 44 data, 15 ecc)
        ecc = _reed_solomon_compute(cw, 15)
        codewords = cw + ecc
        return QrCode(ver, 29, codewords)

    def _draw_function_patterns(self):
        size = self.size
        def set_func(x,y,val):
            self.modules[y][x]=val
            self.is_function[y][x]=True
        # Finder patterns
        for (x,y) in [(0,0),(size-7,0),(0,size-7)]:
            for dy in range(-1,8):
                for dx in range(-1,8):
                    xx,yy=x+dx,y+dy
                    if 0<=xx<size and 0<=yy<size:
                        val = (0<=dx<=6 and 0<=dy<=6 and (dx in (0,6) or dy in (0,6) or (2<=dx<=4 and 2<=dy<=4)))
                        set_func(xx,yy,val)
        # Separators are included above via -1..7 loops
        # Timing patterns
        for i in range(8, size-8):
            set_func(i,6, i%2==0)
            set_func(6,i, i%2==0)
        # Dark module
        set_func(8, size-8, True)
        # Format info areas
        for i in range(9):
            if i!=6:
                set_func(8,i, False)
                set_func(i,8, False)
        for i in range(8):
            set_func(size-1-i,8, False)
            set_func(8,size-1-i, False)

    def _draw_codewords(self, data):
        # data: list of codewords (already includes ecc)
        # Convert to bit list MSB-first
        bits=[]
        for b in data:
            for i in range(7,-1,-1):
                bits.append(_get_bit(b,i)==1)
        size=self.size
        i=0
        x=size-1
        y=size-1
        dir_up=True
        while x>0:
            if x==6: x-=1
            for _ in range(size):
                for xx in [x, x-1]:
                    if not self.is_function[y][xx]:
                        self.modules[y][xx]= bits[i] if i<len(bits) else False
                        i+=1
                y += -1 if dir_up else 1
                if y<0 or y>=size:
                    y = 0 if y<0 else size-1
                    dir_up = not dir_up
                    break
            x -= 2

    def _apply_best_mask(self):
        # Only mask 0 (x+y)%2==0 for simplicity; good enough for release QR.
        size=self.size
        for y in range(size):
            for x in range(size):
                if not self.is_function[y][x]:
                    if (x+y)%2==0:
                        self.modules[y][x]=not self.modules[y][x]
        # Write format bits for level L and mask 0: 0b111011111000100
        fmt = 0b111011111000100
        def set_fmt(x,y,bit):
            self.modules[y][x]=bit
            self.is_function[y][x]=True
        # bits 0..14
        for i in range(0,6):
            set_fmt(8,i, _get_bit(fmt,i)==1)
        set_fmt(8,7, _get_bit(fmt,6)==1)
        set_fmt(8,8, _get_bit(fmt,7)==1)
        set_fmt(7,8, _get_bit(fmt,8)==1)
        for i in range(9,15):
            set_fmt(14-i,8, _get_bit(fmt,i)==1)
        # second copy
        for i in range(0,8):
            set_fmt(self.size-1-i,8, _get_bit(fmt,i)==1)
        for i in range(8,15):
            set_fmt(8,self.size-15+i, _get_bit(fmt,i)==1)

class _BitBuffer:
    def __init__(self): self.bits=[]
    def append_bits(self, val, n):
        for i in range(n-1,-1,-1):
            self.bits.append(((val>>i)&1)==1)
    def __len__(self): return len(self.bits)
    def to_bytes(self):
        out=[]
        for i in range(0,len(self.bits),8):
            b=0
            for j in range(8):
                b=(b<<1) | (1 if (i+j<len(self.bits) and self.bits[i+j]) else 0)
            out.append(b)
        return out

def _reed_solomon_compute(data, ecc_len):
    # GF(256) with primitive poly 0x11D
    exp=[1]*512
    log=[0]*256
    x=1
    for i in range(1,255):
        x <<= 1
        if x & 0x100: x ^= 0x11D
        exp[i]=x
    for i in range(255,512):
        exp[i]=exp[i-255]
    for i in range(1,255):
        log[exp[i]]=i
    def gf_mul(a,b):
        if a==0 or b==0: return 0
        return exp[log[a]+log[b]]
    # Generator polynomial
    gen=[1]
    for i in range(ecc_len):
        gen2=[0]*(len(gen)+1)
        for j,c in enumerate(gen):
            gen2[j] ^= gf_mul(c, exp[i])
            gen2[j+1] ^= c
        gen=gen2
    res=[0]*ecc_len
    for b in data:
        factor=b ^ res[0]
        res=res[1:]+[0]
        for i,g in enumerate(gen[1:]):
            res[i] ^= gf_mul(g,factor)
    return res

qr = QrCode.encode_text(text)
# Render to PBM
quiet=4
scale=8
size = (qr.size + quiet*2) * scale
def pixel(x,y):
    mx=(x//scale)-quiet
    my=(y//scale)-quiet
    if 0<=mx<qr.size and 0<=my<qr.size:
        return 0 if qr.modules[my][mx] else 1  # 0=black,1=white for PBM P1
    return 1
with open(out_path,"w",encoding="ascii") as f:
    f.write("P1\n")
    f.write(f"{size} {size}\n")
    for y in range(size):
        row = [str(pixel(x,y)) for x in range(size)]
        f.write(" ".join(row)+"\n")
print("OK")
PY
    if command -v sips >/dev/null 2>&1; then
      if sips -s format png "$PBM" --out "$ROOT/qr/last_run.png" >/dev/null 2>&1; then
        rm -f "$PBM"
        echo "QR gerado para: $URL"
      else
        echo "WARN: falha ao converter PBM -> PNG; mantendo $PBM"
      fi
    else
      echo "WARN: sips não encontrado; mantendo $PBM (sem PNG)."
    fi
  else
    echo "WARN: links.card_url/url ausente em $CARD"
  fi
else
  echo "INFO: card.sample.json não encontrado; pulando QR."
fi

echo "==> 7/7 DONE"
echo "Artefatos gerados:"
echo "- reports/test_all.$TS.log"
echo "- reports/summary.json"
echo "- MEGA_SHA256SUMS.txt (+ .b3 + .sig + .sig.b64)"
echo "- keys/mega_ed25519_pk.pem"
echo "- qr/last_run.png"

if [[ "${UPDATE_FILELIST:-0}" = "1" ]]; then
  echo "==> Update FILELIST.json (UPDATE_FILELIST=1)"
  python3 - "$ROOT" "$ROOT/FILELIST.json" <<'PY'
import hashlib, json, os, sys
from pathlib import Path

root = Path(sys.argv[1])
filelist_path = Path(sys.argv[2])
data = json.load(open(filelist_path, "r", encoding="utf-8"))
files = data.get("files") or []
by = {e.get("path"): e for e in files if e.get("path")}

def sha256_file(p: Path) -> str:
    h = hashlib.sha256()
    with p.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

# Update all entries that have sha256 filled.
for e in files:
    p = e.get("path")
    if not p:
        continue
    if not (e.get("sha256") or "").strip():
        continue
    fp = root / p
    if not fp.is_file():
        continue
    e["size"] = fp.stat().st_size
    e["sha256"] = sha256_file(fp)

# Ensure the release artifacts exist in FILELIST.json (and have sha256).
must_have = [
    "MEGA_SHA256SUMS.txt",
    "MEGA_SHA256SUMS.b3",
    "MEGA_SHA256SUMS.sig",
    "MEGA_SHA256SUMS.sig.b64",
    "MEGA_PROVENANCE.json",
    "RELEASE_NOTES.md",
    "make_release_artifacts.sh",
    "keys/mega_ed25519_pk.pem",
    "qr/last_run.png",
    "reports/summary.json",
]
for p in must_have:
    fp = root / p
    if not fp.is_file():
        continue
    if p not in by:
        files.append({"path": p, "size": fp.stat().st_size, "sha256": sha256_file(fp)})
    else:
        by[p]["size"] = fp.stat().st_size
        by[p]["sha256"] = sha256_file(fp)

# Include latest test_all log (if present).
logs = sorted((root / "reports").glob("test_all.*.log"))
if logs:
    latest = logs[-1]
    p = str(latest.relative_to(root))
    if p not in by:
        files.append({"path": p, "size": latest.stat().st_size, "sha256": sha256_file(latest)})
    else:
        by[p]["size"] = latest.stat().st_size
        by[p]["sha256"] = sha256_file(latest)

files.sort(key=lambda e: e.get("path", ""))
data["files"] = files
data["count"] = len(files)
open(filelist_path, "w", encoding="utf-8").write(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
print(f"OK: FILELIST.json updated (count={len(files)})")
PY
fi

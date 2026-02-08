#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────
# LogLine TDLN — Master Test Script
# Runs every *locally* testable component from zero to hero.
# Exit code 0 = all green.  Non-zero = something failed.
# ─────────────────────────────────────────────────────────────────────
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
FAST=0
RUN_REMOTE=0
RUN_RUST=0
RUN_JS=0

usage() {
  cat <<'USAGE'
usage: ./test_all.sh [--rust] [--js] [--remote] [--fast] [--all]

  --rust    Run Rust workspace builds/tests (uses cargo; may download deps).
  --js      Run JS/TS builds/tests (requires npm install done ahead of time).
  --remote  Run smoke tests that require a running engine (needs curl+jq and HOST/TDLN_HOST).
  --fast    Skip heavyweight Rust tests (engine-foundation-suite cargo test).
  --all     Equivalent to: --rust --js --remote (and disables --fast).

env:
  FAST=1       same as --fast
  RUN_REMOTE=1 same as --remote
  RUN_RUST=1   same as --rust
  RUN_JS=1     same as --js
USAGE
}

for arg in "$@"; do
  case "$arg" in
    --fast) FAST=1 ;;
    --rust) RUN_RUST=1 ;;
    --js) RUN_JS=1 ;;
    --remote) RUN_REMOTE=1 ;;
    --all) RUN_RUST=1; RUN_JS=1; RUN_REMOTE=1; FAST=0 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $arg"; usage; exit 2 ;;
  esac
done

if [ "${FAST:-0}" = "1" ]; then FAST=1; fi
if [ "${RUN_REMOTE:-0}" = "1" ]; then RUN_REMOTE=1; fi
if [ "${RUN_RUST:-0}" = "1" ]; then RUN_RUST=1; fi
if [ "${RUN_JS:-0}" = "1" ]; then RUN_JS=1; fi

PASS=0
FAIL=0
SKIP=0
RESULTS=()

# ── helpers ──────────────────────────────────────────────────────────
red()   { printf '\033[1;31m%s\033[0m\n' "$*"; }
green() { printf '\033[1;32m%s\033[0m\n' "$*"; }
yellow(){ printf '\033[1;33m%s\033[0m\n' "$*"; }
bold()  { printf '\033[1m%s\033[0m\n' "$*"; }

record_pass() { PASS=$((PASS+1)); RESULTS+=("  PASS  $1"); green "  PASS  $1"; }
record_fail() { FAIL=$((FAIL+1)); RESULTS+=("  FAIL  $1"); red   "  FAIL  $1"; }
record_skip() { SKIP=$((SKIP+1)); RESULTS+=("  SKIP  $1"); yellow "  SKIP  $1"; }

section() { echo; bold "━━━ $1 ━━━"; }

have() { command -v "$1" >/dev/null 2>&1; }

run_step() {
  local name="$1"; shift
  if "$@"; then
    record_pass "$name"
  else
    record_fail "$name"
  fi
}

run_step_in_dir() {
  local name="$1"
  local dir="$2"
  shift 2
  if [ ! -d "$dir" ]; then
    record_skip "$name (missing: $dir)"
    return 0
  fi
  (cd "$dir" && "$@") && record_pass "$name" || record_fail "$name"
}

section "0/7  Toolchain"
echo "  root: $ROOT"
have cargo   && echo "  cargo:   $(cargo --version)" || true
have rustc   && echo "  rustc:   $(rustc --version)" || true
have node    && echo "  node:    $(node --version)" || true
have npm     && echo "  npm:     $(npm --version)" || true
have python3 && echo "  python3: $(python3 --version)" || true
have jq      && echo "  jq:      $(jq --version)" || true
have curl    && echo "  curl:    $(curl --version | head -1)" || true

# ─────────────────────────────────────────────────────────────────────
# 1. INTEGRITY — FILELIST.json (authoritative for this bundle)
# ─────────────────────────────────────────────────────────────────────
section "1/7  Integrity · FILELIST.json"
FILELIST="$ROOT/FILELIST.json"
if have python3 && [ -f "$FILELIST" ]; then
  if python3 - "$ROOT" "$FILELIST" <<'PY'
import hashlib, json, os, sys
root, filelist_path = sys.argv[1], sys.argv[2]
data = json.load(open(filelist_path, "r", encoding="utf-8"))
files = data.get("files") or []
checked = 0
mismatches = []
missing = []
for e in files:
    path = e.get("path")
    want = (e.get("sha256") or "").strip().lower()
    if not path or not want:
        continue
    full = os.path.join(root, path)
    if not os.path.isfile(full):
        missing.append(path); continue
    h = hashlib.sha256()
    with open(full, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    got = h.hexdigest()
    checked += 1
    if got != want:
        mismatches.append((path, want, got))
print(f"  checked: {checked} file(s) with sha256 in FILELIST.json")
if missing:
    print(f"  missing: {len(missing)} (example: {missing[0]})")
if mismatches:
    print(f"  mismatches: {len(mismatches)}")
    for path, want, got in mismatches[:20]:
        print(f"    {path}\n      want {want}\n      got  {got}")

sys.exit(0 if (not missing and not mismatches) else 1)
PY
  then
    record_pass "FILELIST.json integrity"
  else
    record_fail "FILELIST.json integrity"
  fi
else
  record_skip "FILELIST.json integrity (need python3 + FILELIST.json)"
fi

# ─────────────────────────────────────────────────────────────────────
# 2. RUST — SDK + Packs
# ─────────────────────────────────────────────────────────────────────
section "2/7  Rust · workspaces"
if [ "$RUN_RUST" -ne 1 ]; then
  record_skip "Rust workspaces (use --rust)"
elif have cargo; then
    run_step_in_dir "sdk-rust: cargo test --workspace" \
      "$ROOT/engine/sdk-rust.v1.1-ed25519" \
      cargo test --workspace

    run_step_in_dir "integration-pack tdln-rust: cargo test --workspace" \
      "$ROOT/engine/integration-pack.v1.1/tdln-rust" \
      cargo test --workspace

    run_step_in_dir "kits certified-runtime-wasm: cargo test --workspace" \
      "$ROOT/kits/certified-runtime-wasm/rust" \
      cargo test --workspace

    run_step_in_dir "engine certified-runtime-wasm.v1.1: cargo test --workspace" \
      "$ROOT/engine/certified-runtime-wasm.v1.1/rust" \
      cargo test --workspace

    run_step_in_dir "integration-pack cert_runtime: cargo test --workspace" \
      "$ROOT/engine/integration-pack.v1.1/cert_runtime/rust" \
      cargo test --workspace

    if [ "$FAST" -eq 1 ]; then
      record_skip "engine-foundation-suite: cargo test --workspace (FAST=1)"
    else
      run_step_in_dir "engine-foundation-suite: cargo test --workspace" \
        "$ROOT/engine/engine-foundation-suite/engine-foundation-suite-v25c-vendored-json-atomic-fixed-PRO/tmp_vend" \
        cargo test --workspace
    fi
else
    record_skip "Rust workspaces (cargo not found)"
fi

# ─────────────────────────────────────────────────────────────────────
# 3. JAVASCRIPT — TypeScript builds + selftests
# ─────────────────────────────────────────────────────────────────────
section "3/7  JavaScript · packages"
EFS_DIR="$ROOT/engine/engine-foundation-suite/engine-foundation-suite-v25c-vendored-json-atomic-fixed-PRO/tmp_vend"
VERIFY_JS_DIR="$EFS_DIR/packages/verify"
SDK_JS_DIR="$EFS_DIR/sdk/js"
if [ "$RUN_JS" -ne 1 ]; then
  record_skip "JS packages (use --js)"
elif have node && have npm; then
  if [ -d "$VERIFY_JS_DIR" ]; then
    if [ ! -d "$VERIFY_JS_DIR/node_modules" ]; then
      record_skip "@logline/verify (run: cd $VERIFY_JS_DIR && npm install)"
    else
      run_step_in_dir "@logline/verify: npm test" "$VERIFY_JS_DIR" npm test
    fi
  else
    record_skip "@logline/verify (missing)"
  fi

  if [ -d "$SDK_JS_DIR" ]; then
    if [ ! -d "$SDK_JS_DIR/node_modules" ]; then
      record_skip "@tdln/engine-sdk (run: cd $SDK_JS_DIR && npm install)"
    else
      run_step_in_dir "@tdln/engine-sdk: npm run build" "$SDK_JS_DIR" npm run build
    fi
  else
    record_skip "@tdln/engine-sdk (missing)"
  fi
else
  record_skip "JS packages (need node+npm)"
fi

# ─────────────────────────────────────────────────────────────────────
# 4. PYTHON — Conformance vectors
# ─────────────────────────────────────────────────────────────────────
section "4/7  Python · conformance"
RREF_VERIFY="$ROOT/conformance/rref-pack/rref11_verify.py"
RREF_VECTORS="$ROOT/conformance/rref-pack/rref-v1.1-test-vectors.json"
if have python3 && [ -f "$RREF_VERIFY" ] && [ -f "$RREF_VECTORS" ]; then
  if python3 - "$RREF_VERIFY" "$RREF_VECTORS" <<'PY'
import json, subprocess, sys, tempfile, os
verify_py, vectors_json = sys.argv[1], sys.argv[2]
vectors = json.load(open(vectors_json, "r", encoding="utf-8"))
all_ok = True
print(f"  Running {len(vectors)} RREF vectors ...")
for v in vectors:
    name = v.get("name", "<unnamed>")
    expected = v.get("expected")
    card = v.get("card")
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8") as f:
        json.dump(card, f)
        tmp = f.name
    try:
        out = subprocess.check_output([sys.executable, verify_py, tmp], stderr=subprocess.STDOUT)
        actual = json.loads(out.decode("utf-8")).get("result")
    except Exception as e:
        actual = f"ERROR({type(e).__name__})"
    finally:
        try: os.unlink(tmp)
        except OSError: pass
    ok = (actual == expected)
    all_ok = all_ok and ok
    print(("    OK " if ok else "    !! ") + f"{name}: expected={expected} got={actual}")
sys.exit(0 if all_ok else 1)
PY
  then
    record_pass "RREF v1.1 conformance"
  else
    record_fail "RREF v1.1 conformance"
  fi
else
  record_skip "RREF conformance (python3 or files missing)"
fi

# ─────────────────────────────────────────────────────────────────────
# 5. DATA — runtime-vectors.json sanity
# ─────────────────────────────────────────────────────────────────────
section "5/7  Data · runtime conformance vectors"
RUNTIME_VECTORS=(
  "$ROOT/kits/certified-runtime-wasm/conformance/runtime-vectors.json"
  "$ROOT/engine/certified-runtime-wasm.v1.1/conformance/runtime-vectors.json"
  "$ROOT/engine/integration-pack.v1.1/cert_runtime/conformance/runtime-vectors.json"
)
if have python3; then
  VOK=true
  for f in "${RUNTIME_VECTORS[@]}"; do
    if [ ! -f "$f" ]; then
      yellow "  missing: $f"
      VOK=false
      continue
    fi
    echo "  checking $f"
    python3 - "$f" <<'PY' || VOK=false
import json, sys
path = sys.argv[1]
vs = json.load(open(path, "r", encoding="utf-8"))
assert isinstance(vs, list), "vectors must be a list"
for i, v in enumerate(vs):
    assert isinstance(v, dict), f"vector[{i}] must be an object"
    assert "name" in v and isinstance(v["name"], str) and v["name"], f"vector[{i}].name missing"
    assert "policy" in v and isinstance(v["policy"], dict), f"vector[{i}].policy missing"
    assert "expect" in v and v["expect"] in ("PASS","FAIL"), f"vector[{i}].expect must be PASS|FAIL"
print("    OK")
PY
  done
  if $VOK; then
    record_pass "runtime-vectors.json sanity"
  else
    record_fail "runtime-vectors.json sanity"
  fi
else
  record_skip "runtime-vectors.json sanity (python3 not found)"
fi

# ─────────────────────────────────────────────────────────────────────
# 6. SHELL — local JSON checks
# ─────────────────────────────────────────────────────────────────────
section "6/7  Shell · local checks"
VERIFY_SH="$ROOT/template/scripts/verify.sh"
CARD_SAMPLE="$ROOT/template/examples/card.sample.json"
if [ -f "$VERIFY_SH" ] && [ -f "$CARD_SAMPLE" ]; then
    chmod +x "$VERIFY_SH"
    run_step "template/verify.sh on card.sample.json" bash "$VERIFY_SH" "$CARD_SAMPLE"
else
    record_skip "verify.sh or card.sample.json not found"
fi

# Also run the python variants if present (same checks, different wiring).
if have python3 && [ -f "$ROOT/template/check_rref.py" ] && [ -f "$CARD_SAMPLE" ]; then
  run_step "template/check_rref.py on card.sample.json" python3 "$ROOT/template/check_rref.py" "$CARD_SAMPLE"
else
  record_skip "template/check_rref.py (missing python3 or files)"
fi
if have python3 && [ -f "$ROOT/template/check_sirp.py" ] && [ -f "$CARD_SAMPLE" ]; then
  # Note: this script intentionally warns/fails if the exact SIRP kinds aren't present.
  run_step "template/check_sirp.py on card.sample.json" python3 "$ROOT/template/check_sirp.py" "$CARD_SAMPLE"
else
  record_skip "template/check_sirp.py (missing python3 or files)"
fi

# ─────────────────────────────────────────────────────────────────────
# 7. REMOTE — smoke tests (requires running engine)
# ─────────────────────────────────────────────────────────────────────
section "7/7  Remote · smoke (optional)"
if [ "$RUN_REMOTE" -eq 1 ]; then
  if have curl && have jq; then
    if [ -f "$ROOT/engine/ops/smoke.sh" ]; then
      chmod +x "$ROOT/engine/ops/smoke.sh"
      run_step "engine/ops/smoke.sh (HOST/TDLN_HOST)" bash "$ROOT/engine/ops/smoke.sh"
    else
      record_skip "engine/ops/smoke.sh (missing)"
    fi
  else
    record_skip "remote smoke (need curl+jq)"
  fi
else
  record_skip "remote smoke (use --remote)"
fi

# ─────────────────────────────────────────────────────────────────────
# SUMMARY
# ─────────────────────────────────────────────────────────────────────
echo
bold "═══════════════════════════════════════════════════"
bold " RESULTS"
bold "═══════════════════════════════════════════════════"
for r in "${RESULTS[@]}"; do
    echo "$r"
done
echo
bold "  Total: $((PASS+FAIL+SKIP))   Pass: $PASS   Fail: $FAIL   Skip: $SKIP"
bold "═══════════════════════════════════════════════════"

if [ "$FAIL" -gt 0 ]; then
    red "SOME TESTS FAILED"
    exit 1
else
    green "ALL TESTS PASSED"
    exit 0
fi

#!/usr/bin/env bash
set -euo pipefail
find . -type f -name '*.sha256' -print0 | xargs -0 -I{} sh -c 'sha256sum -c {} || exit 2'
echo '[OK] all checksums PASS'

#!/usr/bin/env bash
set -euo pipefail
artifact=${1:?artifact path required}; review=${2:?review path required}
set +e
cargo run --offline --manifest-path tools/claimwright/Cargo.toml -- publication check --artifact "$artifact" --review "$review" --format json --output publication-report.json
status=$?
set -e
case "$status" in 0) exit 0;; 1|2|3) exit "$status";; *) exit 3;; esac

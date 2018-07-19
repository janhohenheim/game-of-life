#!/usr/bin/env bash
set -e

./scripts/build.sh
ulimit -n 8192 || true
serve

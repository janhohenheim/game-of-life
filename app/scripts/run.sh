#!/usr/bin/env bash
set -e

./app/scripts/build.sh
ulimit -n 8192
caddy

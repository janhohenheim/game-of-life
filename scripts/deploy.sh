#!/usr/bin/env bash
set -e

./scripts/build.sh
now --public
now alias

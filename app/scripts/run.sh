#!/usr/bin/env bash

./app/scripts/build.sh
ulimit -n 8192
caddy

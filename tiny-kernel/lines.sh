#!/usr/bin/env bash

find . -type d -name "target" -prune -o -name "*.rs" -type f -print0 | \
xargs -0 wc -l
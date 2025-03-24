#!/usr/bin/bash

do-after-change "clear; cargo test;" "$(fdfind -e rs | paste -s -d " ")"

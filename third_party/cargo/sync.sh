#!/bin/bash
#
# Syncs local build system files w/ changes in Cargo.toml

# Tell the user what we're running from here on out.
set -x

cargo vendor --versioned-dirs 2>&1 1>/dev/null
cargo raze

# Clean up quietly.
set +x

# TODO: git stash, commit, pop

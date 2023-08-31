#!/usr/bin/env bash

# why: https://stackoverflow.com/questions/39554879/does-set-euo-pipefail-have-an-effect-on-eval
# -e: exit when a command fails, don't continue
# -u: treat unset variables as an error
# -o pipefail: don't keep piping if a command fails
set -euo pipefail # exit immediately if command ends with non-zero exit status.

# necessary for rustfmt full features
# rustup default nightly
# rustup component add rustfmt

# lint test fmt 
rustup default nightly && rustup component add clippy rustfmt
cargo clippy -- -Dwarnings
cargo test --all-features --verbose
cargo fmt --all -- --check

# https://taplo.tamasfe.dev/cli/installation/binary.html
# Binary releases currently broken
# options are to install via npm or with cargo.
# npm requires install npm (yikes), cargo requires building from source.
cargo install taplo-cli --locked
taplo fmt --check

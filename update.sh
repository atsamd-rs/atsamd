#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/wez/svd2rust --branch no_unions svd2rust
cargo install --force --version 0.99.2 rustfmt-nightly
cargo install --force --version 0.3.0 form

TOP=$PWD

for chip in atsamd21g18a atsamd21e18a atsamd51j19a ; do
  cd $TOP/pac/$chip
  CHIP=$(echo $chip | tr a-z A-Z)
  rm -rf src
  mkdir src
  svd2rust -i $TOP/svd/$CHIP.svd
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt

  # workaround for https://github.com/rust-embedded/svd2rust/issues/232
  grep -v 'deny(warnings)' src/lib.rs > lib.rs
  mv lib.rs src/lib.rs

  rustfmt build.rs
done

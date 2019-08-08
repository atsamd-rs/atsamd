#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/wez/svd2rust --branch no_unions svd2rust
cargo +nightly install --force --version 1.0.1 rustfmt-nightly
cargo install --force --version 0.6.0 form

TOP=$PWD

for chip in atsamd21g18a atsamd21e18a atsamd21j18a atsamd51g19a atsamd51j19a atsamd51j20a; do
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

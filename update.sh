#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/rust-embedded/svd2rust svd2rust
cargo install --force --version 0.7.0 form

TOP=$PWD

for chip in atsamd11c14a atsamd21g18a atsamd21e18a atsamd21j18a atsamd51g19a atsamd51j19a atsamd51j20a atsame54p20a; do
  cd $TOP/pac/$chip
  CHIP=$(echo $chip | tr a-z A-Z)
  rm -rf src
  mkdir src
  svd2rust --nightly -i $TOP/svd/$CHIP.svd
  form -i lib.rs -o src
  rm lib.rs
  cargo fmt

  rustfmt build.rs
done

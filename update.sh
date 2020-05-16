#!/usr/bin/env bash

set -xe


# INSTALL DEPENDENCIES

cargo install --force --git https://github.com/gkelly/svd2rust --branch \
    bleeding-edge --rev 2bbb60590096bcb67c91f38bedd1f63f98132abe svd2rust
cargo install --force --version 0.7.0 form

python -m pip install -r requirements.txt


# PATCH SVD FILES

count=$(find svd/ -maxdepth 1 -name "*.svd.patched" | wc -l)
if [ "$count" != 0 ]; then 
  rm svd/*.svd.patched
fi 

for device in svd/devices/*.yaml; do
  svd patch "$device"
done


# GENERATE PAC FOR EACH DEVICE

TOP=$PWD

for svd in svd/*.svd.patched; do
  CHIP=$(echo "${svd##*/}" | cut -f 1 -d '.')
  chip=$(echo "$CHIP" | tr '[:upper:]' '[:lower:]')

  cd "$TOP/pac/$chip"
  rm -rf src/

  svd2rust --nightly -i "$TOP/$svd"
  form -i lib.rs -o src/
  rm lib.rs

  cargo fmt
  rustfmt build.rs
done

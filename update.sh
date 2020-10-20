#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install --force --git https://github.com/gkelly/svd2rust --branch \
    bleeding-edge --rev 2bbb60590096bcb67c91f38bedd1f63f98132abe svd2rust
cargo install --force --version 0.7.0 form

# PATCH SVD FILES AND GENERATE CRATES

TOP="${PWD}"
SED="${TOP}/sed"

for xsl in svd/devices/*\.xsl; do
  chip=$(basename "${xsl}" .xsl)
  CHIP=$(echo "${chip}" | tr '[:lower:]' '[:upper:]')
  svd=svd/${CHIP:0:9}.svd
  cp svd/${CHIP}.svd $svd
  
  # remove last characters, because they just represent the memory size
  pushd "${TOP}/pac/${chip:0:9}"

  xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust --nightly

  rm "${TOP}/${svd}"
  rm -rf src/
  form -i lib.rs -o src
  rm lib.rs
  SED -i "s/${CHIP}/${CHIP:0:9}/g" src/lib.rs
  cargo +nightly fmt
  rustfmt +nightly build.rs

  popd
done

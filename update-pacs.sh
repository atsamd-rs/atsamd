#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install --force --version 0.20.0 svd2rust
cargo install --force --version 0.8.0 form

# PATCH SVD FILES AND GENERATE CRATES

TOP="${PWD}"

# Use sed from a supplied SED environment variable, or /bin/sed if unset
SED=${SED:-/bin/sed}

for xsl in svd/devices/*\.xsl; do
  chip=$(basename "${xsl}" .xsl)
  CHIP=$(echo "${chip}" | tr '[:lower:]' '[:upper:]')
  svd=svd/${CHIP:0:9}.svd
  cp "svd/${CHIP}.svd" "$svd"

  # remove last characters, because they just represent the memory size
  pushd "${TOP}/pac/${chip:0:9}"

  xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust --nightly

  rm "${TOP}/${svd}"
  rm -rf src/
  form -i lib.rs -o src
  rm lib.rs

  # remove last characters, because they just represent the memory size
  ${SED} -i "s/${CHIP}/${CHIP:0:9}/g" src/lib.rs

  cargo +nightly fmt
  rustfmt +nightly build.rs

  # ignore all clippy warnings/errors in auto-generated code
  ${SED} -i "s/#\!\[no_std\]/#\!\[allow\(clippy::all\)\]\n#\!\[no_std\]/g" src/lib.rs

  popd
done

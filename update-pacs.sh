#!/usr/bin/env bash

set -xe

# INSTALL DEPENDENCIES

cargo install --version 0.33.5 svd2rust
cargo install --version 0.12.1 form

# PATCH SVD FILES AND GENERATE CRATES

TOP="${PWD}"

# Use sed from a supplied SED environment variable, or /bin/sed if unset
SED=${SED:-/bin/sed}

for xsl in svd/devices/*\.xsl; do
  chip=$(basename "${xsl}" .xsl)
  CHIP=$(echo "${chip}" | tr '[:lower:]' '[:upper:]')

  # Shorten chip names to remove memory size parts of the name
  if [[ "${CHIP}" == "ATSAM"* ]]; then
    short_name="${CHIP:0:9}"
  elif [[ "${CHIP}" == "PIC32CX"* ]]; then
    # PIC32CX<memsize>SG41<pin count>
    short_name="${CHIP:0:7}${CHIP:11}"
  fi

  short_name_lower_case=$(echo "$short_name" | tr '[:upper:]' '[:lower:]')

  svd="${short_name}.svd"

  cp "svd/${CHIP}.svd" "$svd"

  pushd "${TOP}/pac/${short_name_lower_case}"

  xsltproc "${TOP}/${xsl}" "${TOP}/${svd}" | svd2rust --reexport-core-peripherals --reexport-interrupt

  rm "${TOP}/${svd}"
  rm -rf src/
  form -i lib.rs -o src
  rm lib.rs

  ${SED} -i "s/${CHIP}/${short_name}/g" src/lib.rs

  cargo +nightly fmt
  rustfmt +nightly build.rs

  # ignore all clippy warnings/errors in auto-generated code
  # FIXME: Remove allowing `mismatched_lifetime_syntaxes` and `unknown_lints` when upgrading svd2rust >= 0.36.2
  ${SED} -i "s/#\!\[no_std\]/#\!\[allow\(clippy::all\)\]\n#\!\[allow\(unknown_lints\)\]\n#\!\[allow\(mismatched_lifetime_syntaxes\)\]\n#\!\[no_std\]/g" src/lib.rs

  ${SED} -ri '/#\[cfg\(feature = "critical-section"\)\]/d' src/lib.rs

  popd
done

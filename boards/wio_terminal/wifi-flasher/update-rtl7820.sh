#!/bin/bash
# Make sure your Wio terminal already has the update sketch loaded and ready
# to go:
#   cp rtl8720_update.uf2 <drive>
#
# Once the terminal has the flash program loaded and has rebooted, you can
# run this script:
#
# sudo ./update-rtl8720.sh <path-to-usb-serial-port>

SCRIPT_BASE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

DEVICE='/dev/ttyACM0'
if [[ "$1" != '' ]]; then
  DEVICE="${1}"
fi

function check_installed
{
  progs=( debootstrap touch wget tar unzip chroot mount umount )
  for prog in "${progs[@]}"
  do
    loc=`which ${prog}`
    if [[ ${#loc} -lt 3 ]]; then
      if [[ -x "/sbin/${prog}" ]]; then
        echo "Error: Found '${prog}' in /sbin but it is not in your PATH. Are you root?"
        exit 1
      fi
      echo "Error: ${prog} is not installed."
      exit 1
    else
      echo "Found ${prog} at ${loc}."
    fi
  done
  echo ""
}

check_installed
set -e

TMP_AREA=$(mktemp -d)
DEV_MOUNT=''
cleanup () {
  if [ ! -z ${1+x} ]; then
    echo "Error on line $1."
  fi

  if [[ "${DEV_MOUNT}" != "" ]]; then
    umount "${DEV_MOUNT}"
    DEV_MOUNT=''
  fi

  if [[ "${TMP_AREA}" != "" ]]; then
    rm -r "${TMP_AREA}"
    TMP_AREA=''
  fi
}

trap 'cleanup $LINENO' ERR EXIT
set -x

debootstrap bullseye "${TMP_AREA}"
cp -v "${SCRIPT_BASE_DIR}/amebad_flash_tool_v0.1.0_linux.tar.gz" "${TMP_AREA}"
tar -C "${TMP_AREA}" -xvf "${TMP_AREA}/amebad_flash_tool_v0.1.0_linux.tar.gz"
cp -v "${SCRIPT_BASE_DIR}/20200730-rtl8720d-images-v2.2.0.2.zip" "${TMP_AREA}"
unzip "${TMP_AREA}/20200730-rtl8720d-images-v2.2.0.2.zip" -d "${TMP_AREA}"

touch "${TMP_AREA}/device"
mount --bind "${DEVICE}" "${TMP_AREA}/device"
DEV_MOUNT="${TMP_AREA}/device"

LC_ALL='C.UTF-8' LANG='C.UTF-8' chroot "${TMP_AREA}" /amebad_flash_tool erase --port '/device'
LC_ALL='C.UTF-8' LANG='C.UTF-8' chroot "${TMP_AREA}" /amebad_flash_tool flash -d '/20200730-rtl8720d-images-v2.2.0.2' --port '/device'

#!/usr/bin/env bash

RET=0

# first, all boards & their examples
DIRECTORIES=$(find ./boards -type f -iname Cargo.toml)
echo $DIRECTORIES
for DIR in $DIRECTORIES
do
    pushd $(dirname $DIR)
    cargo +nightly fmt -- --check
    RET=$(($RET + $?))
    popd
done

# now the PAC & main HAL
DIRECTORIES=$(find ./hal ./pac -type f -iname Cargo.toml)
echo $DIRECTORIES
for DIR in $DIRECTORIES
do
    pushd $(dirname $DIR)
    cargo +nightly fmt -- --check
    RET=$(($RET + $?))
    popd
done

exit $RET

#!/bin/bash
set -xe

HERE=$PWD

for board in boards/* ; do
  cd $board
  cargo +nightly fmt
  cd $HERE
done

cd hal
cargo +nightly fmt
cd $HERE

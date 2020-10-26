RET=0

# first, all boards & their examples
DIRECTORIES=$(find ./boards -type f -iname Cargo.toml)
echo $DIRECTORIES
for DIR in $DIRECTORIES
do
    pushd $(dirname $DIR)
    rustfmt --edition 2018 --check ./src/lib.rs ./examples/*.rs
    RET=$(($RET + $?))
    popd
done

# now the PAC & main HAL
DIRECTORIES=$(find ./hal ./pac -type f -iname Cargo.toml)
echo $DIRECTORIES
for DIR in $DIRECTORIES
do
    pushd $(dirname $DIR)
    rustfmt --edition 2018 --check ./src/lib.rs
    RET=$(($RET + $?))
    popd
done

exit $RET

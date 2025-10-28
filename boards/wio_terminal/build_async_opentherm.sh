CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --example async_opentherm
objcopy -I elf32-little -O binary ../../target/thumbv7em-none-eabihf/debug/examples/async_opentherm async_opentherm.bin
export BINARY_FILE=async_opentherm.bin; gdb-multiarch --batch asyn_opentherm.bin --ex "target remote localhost:3333" --ex "mon reset halt" --ex "mon program $(readlink -f ${BINARY_FILE}) 0x00000 verify"
# export BINARY_FILE=async_opentherm.bin; /home/cooler1989/programs/cgdb/cgdb/cgdb -d gdb-multiarch ../../target/thumbv7em-none-eabihf/debug/examples/async_opentherm --ex "target remote localhost:3333" --ex "mon reset halt" -ex "mon arm semihosting enable" -ex "mon arm semihosting_redirect tcp 2999"

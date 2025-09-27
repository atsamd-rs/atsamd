target remote :3333

set print asm-demangle on

load

mon reset halt

break DefaultHandler

break HardFault

break rust_begin_unwind

# print demangled symbols by default
set print asm-demangle on

# JLink started via Bobbin
target extended-remote :2331
monitor flash breakpoints 1
# allow hprints to show up in gdb
#monitor semihosting enable
#monitor semihosting IOClient 3

monitor reset
load
#break usb_serial::usb_handler
break HardFault
break DefaultHandler

# OpenOCD
#target extended-remote :3333
#monitor arm semihosting enable
#load
#step

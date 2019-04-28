MEMORY
{
  /* Leave 16k for the default bootloader on the PyPortal */
  FLASH (rx) : ORIGIN = 0x00000000 + 16K, LENGTH = 512K - 16K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 192K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);


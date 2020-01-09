MEMORY
{
  /* Leave 8k for the default bootloader on the Serpente */
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K - 256 - 256
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);


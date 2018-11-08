MEMORY
{
  /* Leave 8k for the default bootloader on the Metro M4 */
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 512K - 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 192K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);


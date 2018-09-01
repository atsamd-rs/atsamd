MEMORY
{
  /* Leave 8k for the default bootloader on the Metro M0 */
  /* FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K */
  /* Actually: don't, as the bootloader interferes with flashing
     via the JLink and blocks interrupt handling */
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);


MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K /* First 8KB used by bootloader */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}
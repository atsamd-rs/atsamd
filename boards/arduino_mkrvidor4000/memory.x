MEMORY
{
  FLASH_FPGA (r) : ORIGIN = 0x40000, LENGTH = 2M
  FLASH (rx) : ORIGIN = 0x2000, LENGTH = 256K - 8K /* First 8KB used by bootloader */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}

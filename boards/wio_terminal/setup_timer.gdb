printf "MCLK reg:\n"
x /xw 0x4000081c

# enable de-assert enable signal:
mon mww 0x42001400 0x604
shell sleep 1
# enable assert SWRT:
mon mww 0x42001400 0x605
shell sleep 1
# Change mode to 32-bits
mon mww 0x42001400 0x608
shell sleep 1
# enable COPEN
mon mww 0x42001400 0x10608
shell sleep 1
# enable CAPTEN
mon mww 0x42001400 0x110608
shell sleep 1
# enable timer:
mon mww 0x42001400 0x11060A
shell sleep 1
# enable timer when DBG
mon mwb 0x42001409 0x01
# Capture data:
mon mwb 0x42001405 0x80
# read counter:
x /xw 0x42001414

set $counter = 0
while $counter < 5
  printf "iter: %d\n", $counter
  # Capture data:
  mon mwb 0x42001405 0x80
  # read counter:
  x /xw 0x42001414
  shell sleep 0.1
  set $counter = $counter + 1
end

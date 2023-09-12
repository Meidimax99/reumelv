# Short overview of some useful gdb commands

## IPC Registers

`-exec printf "%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n%08x\n", $s0,$s1,$s2,$s3,$s4,$s5,$s6,$s7,$s8,$s9,$s10,$s11`

## All Registers

`-exec i registers

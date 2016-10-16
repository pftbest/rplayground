target remote :3333
set remote hardware-breakpoint-limit 6
set remote hardware-watchpoint-limit 4
monitor reset halt
monitor arm semihosting enable
load
monitor reset halt
layout asm

define hook-quit
    set confirm off
end

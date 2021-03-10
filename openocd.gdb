target remote :3333
set print asm-demangle on
set print pretty on
load
break DefaultHandler
break UserHardFault
break main
continue
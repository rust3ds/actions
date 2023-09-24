# https://github.com/devkitPro/libctru/blob/master/libctru/source/system/stack_adjust.s#LL28C23-L28C23
# or should this be `_exit` ?
break __ctru_exit
commands
    # ARM calling convention will put the exit code in r0 when __ctru_exit is called.
    # Just tell GDB to exit with the same code, since it doesn't get passed back when
    # the program exits
    quit $r0
end

target extended-remote :4000
continue
quit

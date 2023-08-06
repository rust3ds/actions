# https://github.com/devkitPro/libctru/blob/master/libctru/source/system/stack_adjust.s#LL28C23-L28C23
# or should this be `_exit` ?
break __ctru_exit
commands
    # TODO: needed?
    continue
    # ARM calling convention will put the exit code in r0 when __ctru_exit is called.
    # Just tell GDB to exit with the same code, since it doesn't get passed back when
    # the program exits
    quit $r0
end

# TODO: parametrize or pass as command line arg instead
target extended-remote 192.168.0.167:4003
# target extended-remote :4000
continue

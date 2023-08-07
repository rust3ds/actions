#!/bin/bash

# Clean up child processes on exit: https://stackoverflow.com/a/2173421/14436105
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

ls -lart $@

ERRS=0
# shellcheck disable=SC2068
for EXE in $@; do
    VIDEO_OUT="$(dirname "$EXE")/$(basename "$EXE" .elf)_out.webm"

    # colored logs would be nice, but we can always just grab the plaintext log file
    xvfb-run citra --appimage-extract-and-run --dump-video="$VIDEO_OUT" "$EXE" &>/dev/null &

    # Citra takes a little while to start up, so wait a little before we try to connect
    sleep 3

    arm-none-eabi-gdb --batch-silent --command /app/test-runner.gdb "$EXE"
    STATUS=$?
    if [ $STATUS -ne 0 ]; then
        ERRS=$((ERRS + 1))
    fi
done

exit $ERRS

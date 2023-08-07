#!/bin/bash

# Clean up child processes on exit: https://stackoverflow.com/a/2173421/14436105
trap "pkill -P $$" EXIT INT TERM

mkdir -p ~/.config/citra-emu
cp /app/sdl2-config.ini ~/.config/citra-emu
# For some reason, log file is only written when this dir already exists,
# but it is only created after the first run of citra (our only run, in the container)
mkdir -p ~/.local/share/citra-emu/

ERRS=0
# shellcheck disable=SC2068
for EXE in $@; do
    VIDEO_OUT="$(dirname "$EXE")/$(basename "$EXE" .elf)_capture.webm"

    # colored logs would be nice, but we can always just grab the plaintext log file
    xvfb-run citra \
        --appimage-extract-and-run \
        --dump-video="$VIDEO_OUT" \
        "$EXE" \
        &>/dev/null &
    PID=$!

    # Citra takes a little while to start up, so wait a little before we try to connect
    sleep 3

    arm-none-eabi-gdb --silent --batch-silent --command /app/test-runner.gdb "$EXE"
    STATUS=$?
    if [ $STATUS -ne 0 ]; then
        echo >&2 "FAILED (exit status $STATUS): $EXE"
        ERRS=$(( ERRS + 1 ))
    fi

    kill -INT $PID &>/dev/null
    sleep 1
    if kill -0 $PID &>/dev/null; then
        kill -KILL $PID &>/dev/null
    fi

    CITRA_LOG=~/.local/share/citra-emu/log/citra_log.txt
    CITRA_LOG_OUT="$(dirname "$EXE")/$(basename "$EXE" .elf)_citra_log.txt"
    if test -f "$CITRA_LOG"; then
        cp "$CITRA_LOG" "$CITRA_LOG_OUT"
    else
        echo "WARNING: citra log not found"
    fi
done

exit $ERRS

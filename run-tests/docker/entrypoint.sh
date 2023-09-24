#!/bin/bash

# Uncomment for debugging the action itself. Maybe consider a job summary or
# grouping the output, to keep this stuff visible but make it simpler to use:
# https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions

# set -x

function cleanup_jobs() {
    # shellcheck disable=SC2317 # Unreachable because it's only used in trap
    if [ -n "$(jobs -p)" ]; then
        sleep 5 &
        wait -n
        # shellcheck disable=SC2046 # We want to expand jobs here and for `wait`
        kill -9 $(jobs -p)
        # shellcheck disable=SC2046
        wait $(jobs -p) &>/dev/null
    fi
}

trap cleanup_jobs EXIT

EXE_ELF=$1
EXE_3DSX="$(dirname "$EXE")/$(basename "$EXE" .elf).3dsx"

EXE_TO_RUN="$EXE_ELF"
if [ -f "$EXE_3DSX" ]; then
    echo >&2 "Found $(basename "$EXE_3DSX"), it will be run instead of $(basename "$EXE_ELF")"
    EXE_TO_RUN="$EXE_3DSX"
fi

VIDEO_OUT="$(dirname "$EXE_ELF")/$(basename "$EXE_ELF" .elf)_capture.webm"

CITRA_LOG_DIR=~/.local/share/citra-emu/log
CITRA_OUT="$CITRA_LOG_DIR/citra_output.txt"

xvfb-run --auto-servernum \
    citra \
    --appimage-extract-and-run \
    --dump-video="$VIDEO_OUT" \
    "$EXE_TO_RUN" \
    &>"$CITRA_OUT" &
CITRA_PID=$!

# Citra takes a little while to start up, so wait a little before we try to connect
sleep 5

arm-none-eabi-gdb --silent --batch-silent --command /app/test-runner.gdb "$EXE_ELF"
STATUS=$?

kill $CITRA_PID
cleanup_jobs

CITRA_LOG="$CITRA_LOG_DIR/citra_log.txt"

for f in "$CITRA_LOG" "$CITRA_OUT"; do
    OUT="$(dirname "$EXE_ELF")/$(basename "$EXE_ELF" .elf)_$(basename "$f")"
    if test -f "$f"; then
        cp "$f" "$OUT"
        if [ $STATUS -ne 0 ]; then
            echo >&2 "$(basename $f) copied to $OUT"
        fi
    else
        echo >&2 "WARNING: $(basename "$f") not found"
    fi
done

exit $STATUS

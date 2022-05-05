#!/bin/bash

while ! vncdo -s citra pause 0 &>/dev/null; do
    echo "waiting for VNC server..."
    sleep 1
done

exec vncdo -s citra --nocursor --delay 500 "$@"

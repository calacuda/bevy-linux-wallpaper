#!/bin/bash

PIDFILE="/var/run/user/$UID/bg.pid"

declare -a PIDs

_screen() {
    echo "$1"
    xwinwrap -ov -ni -g "$1" -- mpv --fullscreen\
        --no-stop-screensaver \
        --hwdec=vdpau \
        --loop-file --no-audio --no-osc --no-osd-bar -wid WID --no-input-default-bindings \
        /tmp/frame.gif &
    PIDs+=($!)
}

while read p; do
  [[ $(ps -p "$p" -o comm=) == "xwinwrap" ]] && kill -9 "$p";
done < $PIDFILE
killall bg-serv

rm /tmp/frame.gif
$HOME/Code/rust/game-background/target/debug/bg-serv &
# PIDs+=($!)
sleep 0.5
sleep 5.5

for i in $( xrandr -q | grep ' connected' | grep -oP '\d+x\d+\+\d+\+\d+')
do
    _screen "$i"
done

printf "%s\n" "${PIDs[@]}" > $PIDFILE


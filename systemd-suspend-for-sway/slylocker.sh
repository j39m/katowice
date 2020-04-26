#!/bin/sh

set -eu;

uid="$(id -u)";

for sock in /run/user/"${uid}"/sway-ipc.*.*.sock; do
    [ -S "${sock}" ] || continue;
    swaymsg -s "${sock}" exec -- /usr/bin/swaylock;
done;

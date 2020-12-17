#!/bin/sh

set -u;

uid="$(id -u)";

for sock in /run/user/"${uid}"/sway-ipc.*.*.sock; do
    [ -S "${sock}" ] || continue
    swaymsg -s "${sock}" exec -- /usr/bin/swaylock
done

for _ in {1..26}; do
    pgrep -u "${uid}" -x swaylock && break
    sleep 0.13
done
sleep 1.82

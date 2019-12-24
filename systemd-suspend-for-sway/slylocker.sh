#!/bin/sh

# slylocker.sh is so named because I might swap between swaylock and
# i3lock. That's no longer true, but the slightly weasely way in which
# this script locks your screen merits its name.

#function env_var_from_swaybg() {
#    local swaybg_pid="$(pgrep -x swaybg | head -n1)"
#    [ "$(realpath /proc/${swaybg_pid}/exe)" = /usr/bin/swaybg ] || return;
#    xargs --null /usr/bin/printf "%s\n" < /proc/"${swaybg_pid}"/environ \
#    | grep '^'"${1}=" | cut -d "=" -f2
#}

#export DISPLAY=:0
#export WAYLAND_DISPLAY="$(env_var_from_swaybg WAYLAND_DISPLAY)"
#export XDG_RUNTIME_DIR="$(env_var_from_swaybg XDG_RUNTIME_DIR)"

set -eu;

uid="$(id -u "${1}")";

for sock in /run/user/"${uid}"/sway-ipc.*.*.sock; do
    [ -S "${sock}" ] || continue;
    swaymsg -s "${sock}" exec -- /usr/bin/swaylock -fk \
        -i /usr/share/pixmaps/lock.png \
        --indicator-radius 130 --ring-color c35b9c80 \
        --inside-color c35b9c80 --ring-ver-color c35b9c80 \
        --inside-ver-color c35b9c80;
done;

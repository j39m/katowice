#!/bin/bash

set -u

LOGFILE="${XDG_RUNTIME_DIR}/rclone.log"

systemd-inhibit --what=sleep \
rclone sync \
    -v -x -M -l \
    --log-file="${LOGFILE}" \
    --filter '- ' \
    --filter '- /.cache/' \
    --filter '- /.cargo/' \
    --filter '- /.local/share/' \
    --filter '- /.mozilla/' \
    --filter '- /.thunderbird/' \
    --filter '- /.var/app/org.signal.Signal/' \
    --filter '- /.var/app/org.jellyfin.JellyfinServer/' \
    --filter '- /Downloads/.firefox-nightly/' \
    --filter '- /Downloads/.thunderbird-beta/' \
    ~/ rsync.net-crypt:
echo

#!/bin/bash

set -u

LOGFILE="${XDG_RUNTIME_DIR}/rclone.log"

rclone sync \
    -v -M -l \
    --log-file="${LOGFILE}" \
    --filter '- ' \
    --filter '- /.cache/' \
    --filter '- /.cargo/' \
    --filter '- /.local/share/' \
    --filter '- /.mozilla/' \
    --filter '- /.thunderbird/' \
    --filter '- /Downloads/.firefox-nightly/' \
    --filter '- /Downloads/.thunderbird-beta/' \
    ~/ rsync.net-crypt:

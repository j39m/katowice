#!/bin/bash

NANISORE_WIDTH=52
NANISORE_SCREENFUL=42

function _nanisore() {
    ps -eo args:"${NANISORE_WIDTH}",euser,pid,pcpu,pmem,state \
        --sort="${1}"
}

function nanisore() {
    _nanisore -rss | head -n "${NANISORE_SCREENFUL}"
}

function hahisore() {
    _nanisore state
}

function mamisore() {
    _nanisore -pcpu | head -n "${NANISORE_SCREENFUL}"
}

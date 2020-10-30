#!/bin/bash

MONITOR_DEVICE="pulse://alsa_output.pci-0000_06_00.6.analog-stereo"
MONITOR_DEVICE+=".monitor"

SOUT="#transcode{vcodec=none,acodec=opus,"
SOUT+="ab=130,channels=2,samplerate=44100,scodec=none}"
SOUT+=":http{dst=:13910/klaus.ogg}"

unner vlk -I "dummy" -vvv "${MONITOR_DEVICE}" --sout "${SOUT}"

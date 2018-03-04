#!/usr/bin/python3

"""
Given a file as an argument, create a symlink to said file, named by its
modification time and extension.

This little thing is intended to be called with ``find ... | xargs -n 1.''
"""

import sys
import os
#import time


# Parse arguments.
TARGET = sys.argv[1]
EXT = os.path.splitext(TARGET)[-1]

# Get the mtime.
statObj = os.stat(TARGET)
mtime = statObj.st_mtime

# Make the symlink.
while True:
    try:
        symlink = str(mtime) + EXT
        os.symlink(TARGET, symlink)
        sys.exit(0)
    except FileExistsError:
        mtime += 0.1

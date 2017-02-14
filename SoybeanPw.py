#!/usr/bin/env python3

"""
A script that generates not-very-random passwords.
"""

import random
import re
import sys


DEFAULT_LENGTH =    5
MAX_LENGTH =        26

WF = "/usr/share/dict/linux.words"

# If we were given a good arg, read it.
try:
    pw_length = int(sys.argv[1])
    if pw_length < 1 or pw_length > MAX_LENGTH:
        raise ValueError("Too short or too long")
except (ValueError, IndexError) as e:
    pw_length = DEFAULT_LENGTH

# Open the words file and read it.
all_words = [word.strip()#.lower()
             for word in open(WF, "r").readlines()]

# Get several words.
print(" ".join(random.sample(all_words, pw_length)))

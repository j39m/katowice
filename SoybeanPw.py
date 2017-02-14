#!/usr/bin/env python3

"""
A script that generates not-very-random passwords.
"""

import random
import re
import sys


DEFAULT_LENGTH =    4
MAX_LENGTH =        26


# Whenever a random selection is made, we should check bounds.
def saneLength(selMethod):
    def fnWrapper(self, lenSelect=DEFAULT_LENGTH):
        if lenSelect < 1:
            lenSelect = 1
        elif lenSelect > MAX_LENGTH:
            lenSelect = MAX_LENGTH
        return selMethod(self, lenSelect)
    return fnWrapper


class enSoybean(object):
    """
    Yields up random English words.
    Actually yields up random selections from the system dictionary...
    """
    WF = "/usr/share/dict/linux.words"

    def __init__(self,):
        with open(self.WF) as wordsFile:
            allWords = [w.strip() for w in wordsFile.readlines()]
            self.selection = allWords

    @saneLength
    def getWords(self, numWords):
        """
        Return a nonrandom list of numWords.
        """
        return random.sample(self.selection, numWords)


def main(*args):
    wordsEn = enSoybean()
    print(" ".join(wordsEn.getWords()))
    return 0


if __name__ == "__main__":
    _r = main(*sys.argv)
    sys.exit(_r)

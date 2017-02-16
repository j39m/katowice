#!/usr/bin/env python3

"""
A script that generates not-very-random passwords.

Do not use this script. It does not generate good passwords.
If you wish to know why, post the source code online and make the claim
that you've found a good script that generates good passwords. An angry
cryptologist will come around in due course to explain why that is untrue.
"""

import random
import re
import sys


DEFAULT_LENGTH =    4
MAX_LENGTH =        26

CHANCE_MISSPELL =   0.26


def saneLength(selMethod):
    """
    Whenever a random selection is made, we should check bounds.
    """
    def fnWrapper(self, lenSelect=DEFAULT_LENGTH):
        if lenSelect < 1:
            lenSelect = 1
        elif lenSelect > MAX_LENGTH:
            lenSelect = MAX_LENGTH
        return selMethod(self, lenSelect)
    return fnWrapper

def misspell(aWord):
    """
    Misspell a word some of the time.
    I shouldn't rely on this; assuming all words in the dictionary are
    5 characters long, there are 30 misspellings and 1 proper spellings.
    So for a 4-word password, your search space is roughly multiplied by
    (31 spellings) ** (4 words) == 923.521, which is okay. Not sure if
    that math makes sense.
    """
    # If a misspelling is not called for, return the word untouched.
    if random.random() > CHANCE_MISSPELL:
        return aWord
    # If a misspelling is due, listify the word, pick a random vowel,
    # and insert it into a random index.
    lWord = [l for l in aWord]
    randVowel = random.choice("aoeui")
    randIndex = random.randint(0, len(aWord))
    lWord.insert(randIndex, randVowel)
    return "".join(lWord)


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

    @saneLength
    def getCorruptedWords(self, numWords):
        words = self.getWords(numWords)
        return [misspell(w) for w in words]


class jpSoybean(object):
    """
    Yields up random (probably invalid) hiragana arrangements.
    """
    def __init__(self,):
        pass


def main(*args):
    wordsEn = enSoybean()
    print(" ".join(wordsEn.getCorruptedWords()))
    return 0


if __name__ == "__main__":
    _r = main(*sys.argv)
    sys.exit(_r)

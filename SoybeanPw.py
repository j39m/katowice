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
    def fnWrapper(self, lenSelect=DEFAULT_LENGTH, **kwargs):
        if lenSelect < 1:
            lenSelect = 1
        elif lenSelect > MAX_LENGTH:
            lenSelect = MAX_LENGTH
        return selMethod(self, lenSelect, **kwargs)
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
    def __init__(self, defaultWordLen=4):
        self.defaultWordLen = defaultWordLen
        # The individual hiragana components are given separately.
        self._base = (
            "a",    "i",    "u",    "e",    "o",
            "ka",   "ki",   "ku",   "ke",   "ko",
            "sa",   "shi",  "su",   "se",   "so",
            "ta",   "chi",  "tsu",  "te",   "to",
            "na",   "ni",   "nu",   "ne",   "no",
            "ha",   "hi",   "hu",   "he",   "ho",
            "ma",   "mi",   "mu",   "me",   "mo",
            "ya",           "yu",           "yo",
            "ra",   "ri",   "ru",   "re",   "ro",
            "wa",                           "wo",
        )
        self._digraphs = (
            "kya",  "kyu",  "kyo",
            "sha",  "shu",  "sho",
            "cha",  "chu",  "cho",
            "nya",  "nyu",  "nyo",
            "hya",  "hyu",  "hyo",
            "mya",  "myu",  "myo",
            "rya",  "ryu",  "ryo",
        )
        self._diacritics = (
            "ga",   "gi",   "gu",   "ge",   "go",
            "za",   "ji",   "zu",   "ze",   "zo",
            "da",                   "de",   "do",
            "ba",   "bi",   "bu",   "be",   "bo",
            "pa",   "pi",   "pu",   "pe",   "po",
        )
        self._digraphs_with_diacritics = (
            "gya",  "gyu",  "gyo",
            "ja",   "ju",   "jo",
            "bya",  "byu",  "byo",
            "pya",  "pyu",  "pyo",
        )

        # We combine all the hiragana into one big tuple.
        self.hiragana = (
            *self._base,
            *self._digraphs,
            *self._diacritics,
            *self._digraphs_with_diacritics,
        )

        # The neutered set offers only 68 choices per word,
        # but is much easier to deal with when typing.
        self.neutered = (
            *self._base,
            *self._diacritics
        )

    def getSyllables(self, numSyl=None, neutered=False):
        if not numSyl:
            numSyl = self.defaultWordLen
        choiceSet = self.hiragana
        if neutered:
            choiceSet = self.neutered
        return random.sample(choiceSet, numSyl)

    @saneLength
    def getWords(self, numWords, neuter=False):
        return [
            "".join(self.getSyllables(neutered=neuter))
            for _ in range(numWords)
        ]

def main(*args):
    wordsEn = enSoybean()
    wordsJp = jpSoybean()
    # Prints a password in English.
    #print(" ".join(wordsEn.getWords()))
    # Prints a password in something not Japanese.
    #print(" ".join(wordsJp.getWords(neuter=True)))
    # Prints a shorter password in something not Japanese.
    shorter = [
        "".join(wordsJp.getSyllables(numSyl=i, neutered=True))
        for i in (4, 3, 3, 3,)
    ]
    print(" ".join(shorter))
    return 0


if __name__ == "__main__":
    _r = main(*sys.argv)
    sys.exit(_r)

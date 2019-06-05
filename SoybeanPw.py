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

DEFAULT_LENGTH = 4
MAX_LENGTH = 26

CHANCE_MISSPELL = 0.26

def misspell(word):
    """
    Misspells a word some of the time by inserting a vowel.

    I shouldn't rely on this; assuming all words in the dictionary are
    5 characters long, there are 30 misspellings and 1 proper spellings.
    So for a 4-word password, your search space is roughly multiplied by
    (31 spellings) ** (4 words) == 923.521, which is okay. Not sure if
    that math makes sense.
    """
    # If a misspelling is not called for, return the word untouched.
    if random.random() > CHANCE_MISSPELL:
        return word
    # If a misspelling is due, listify the word, pick a random vowel,
    # and insert it into a random index.
    listified = [letter for letter in word]
    random_vowel = random.choice("aoeui")
    random_index = random.randint(0, len(word))
    listified.insert(random_index, random_vowel)
    return "".join(listified)

class EnglishSoybean(object):
    """
    Yields up random English words.
    Actually yields up random selections from the system dictionary...
    """
    WORDS_PATH = "/usr/share/dict/linux.words"

    def __init__(self):
        with open(self.WORDS_PATH) as words_fp:
            self.words = [word.strip() for word in words_fp]

    def get_words(self, count):
        """Return a list of words."""
        word_list = random.sample(self.words, count)
        return [word.lower() for word in word_list]

    def get_misspelled_words(self, count):
        return list()
        words = self.getWords(numWords)
        return [misspell(w) for w in words]

class JapaneseSoybean(object):
    """
    Yields up random (probably invalid) hiragana arrangements.
    """
    def __init__(self, default_count=4):
        self.default_count = default_count
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

    def get_syllables(self, count=None, neutered=True):
        """
        Returns a syllable list of length <count>. The bool neutered
        controls selection from all hiragana or the easy hiragana.
        """
        count = count if count else self.default_count
        syllables = self.neutered if neutered else self.hiragana
        return random.sample(syllables, count)

    def get_word(self, syl_count=None, neutered=True):
        """
        Returns a single word per arguments specified.
        """
        syl_list = self.get_syllables(syl_count, neutered)
        return "".join(syl_list)

    def get_words(self, *args, neutered=True):
        """
        Return a list of several words. *args should contain numbers
        specifying the length of each word. The neutered arg determines
        whether we select from easy or all hiragana.
        """
        return [self.get_word(syl_count, neutered) for syl_count in args]

def main(*args):
    pw_obj = JapaneseSoybean()
    word_list = pw_obj.get_words(4, 3, 3, 3, neutered=True)
    #pw_obj = EnglishSoybean()
    #word_list = pw_obj.get_words(3)
    print(" ".join(word_list))
    return 0

if __name__ == "__main__":
    _r = main(*sys.argv)
    sys.exit(_r)

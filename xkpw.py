#!/usr/bin/env python3

"""
A script that generates not-very-random passwords.

Do not use this script. It does not generate good passwords.
If you wish to know why, post the source code online and make the claim
that you've found a good script that generates good passwords. An angry
cryptologist will come around in due course to explain why that is untrue.
"""

import argparse
import random
import sys

CHANCE_MISSPELL = 0.26

DEFAULT_JAPANESE_WORDS = [4, 3, 3, 3]
DEFAULT_ENGLISH_WORDS = 4

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

class EnglishSoybean:
    """
    Yields up random English words.
    Actually yields up random selections from the system dictionary...
    """
    WORDS_PATH = "/usr/share/dict/linux.words"

    def __init__(self):
        with open(self.WORDS_PATH) as words_fp:
            self.words = [word.strip() for word in words_fp]

    def get_words(self, count):
        """
        Returns a list of words.

        Args:
            count: the number of words returned.
        """
        word_list = random.sample(self.words, count)
        return [word.lower() for word in word_list]

    def get_misspelled_words(self, count):
        """
        Returns a list of possibly misspelled words.

        Args:
            count: the number of possibly misspelled words.
        """
        words = self.get_words(count)
        return [misspell(w) for w in words]

class JapaneseSoybean:
    """
    Yields up random (probably invalid) hiragana arrangements.
    """
    def __init__(self, default_count=4):
        self.default_count = default_count
        # The individual hiragana components are given separately.
        # pylint: disable=bad-whitespace
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
        Returns a list of syllables.

        Args:
            count: controls how many syllables are returned in the list.
            neutered: selects a set of syllables; if True, we select from
                the easy hiragana. If False, we select from all hiragana.
        """
        count = count if count else self.default_count
        syllables = self.neutered if neutered else self.hiragana
        return random.sample(syllables, count)

    def get_word(self, syllable_count=None, neutered=True):
        """
        Returns a single word.

        Args:
            syllable_count: controls how many syllables are composed
                into the returned word.
            neutered: composes the word only of easy hiragana if True and
                of any hiragana if False.
        """
        syllables = self.get_syllables(syllable_count, neutered)
        return "".join(syllables)

    def get_words(self, *word_lengths, neutered=True):
        """
        Returns a list of several words.

        Args:
            word_lengths: a list of natural numbers specifying the
                length of each word.
            neutered: controls whether the returned list of words is
                composed of easy hiragana or all hiragana.
        """
        return [
            self.get_word(word_length, neutered)
            for word_length in word_lengths
        ]

def generate_japanese_password(args):
    """
    Generates a pseudo-Japanese password composed of random syllables.

    Args:
        args: Namespace object parsed by argparse.

    Returns:
        A list of pseudo-Japanese words.
    """
    password_maker = JapaneseSoybean()
    print(args.counts)
    return password_maker.get_words(*args.counts, neutered=args.difficult)

def generate_english_password(args):
    """
    Generates an English password composed of random words.

    Args:
        args: Namespace object parsed by argparse.

    Returns:
        A list of English words.
    """
    return list()

def init_japanese_parser(subparsers):
    """
    Adds the "jp" subparser to the argparse subparsers.

    Args:
        subparsers: the object returned from an earlier call to
                    ArgumentParser.add_subparsers().
    """
    japanese_parser = subparsers.add_parser("jp")
    japanese_parser.add_argument(
        "--difficult",
        "-d",
        help="use all hiragana",
        action="store_true")
    japanese_parser.add_argument(
        "counts",
        help="list of word lengths",
        default=DEFAULT_JAPANESE_WORDS,
        type=int,
        nargs="*",
    )

    japanese_parser.set_defaults(func=generate_japanese_password)

def init_english_parser(subparsers):
    """
    Adds the "en" subparser to the argparse subparsers.

    Args:
        subparsers: the object returned from an earlier call to
                    ArgumentParser.add_subparsers().
    """
    english_parser = subparsers.add_parser("en")
    english_parser.add_argument(
        "--misspell",
        "-m",
        help="randomly introduce vowels into words",
        action="store_true",
    )
    english_parser.add_argument(
        "count",
        help="number of words to print",
        type=int,
        default=DEFAULT_ENGLISH_WORDS,
        nargs="*",
    )

    english_parser.set_defaults(func=generate_english_password)

def main(*args):
    """Read arguments. Pick a language. Print a password."""
    parser = argparse.ArgumentParser()
    parser.set_defaults(func=generate_japanese_password)
    subparsers = parser.add_subparsers()

    init_japanese_parser(subparsers)
    init_english_parser(subparsers)

    parsed_args = parser.parse_args(args[1:])

    word_list = parsed_args.func(parsed_args)
    print(" ".join(word_list))
    return 0

if __name__ == "__main__":
    main(*sys.argv)

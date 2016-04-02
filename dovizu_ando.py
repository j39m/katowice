#!/usr/bin/python3

import sys
import random

# Practice with Hiragana

class BasicHiragana(object):

    def __init__(self, strlen, adaptive=False):
        """
        sets up the Hiragana we want to practice with.
        @param strlen:       how long should the challenge phrase be?
        @param adaptive:     should the challenges grow in difficulty?
        """

        self.basic_vowels = {"a" : "あ",
                             "i" : "い",
                             "u" : "う",
                             "e" : "え",
                             "o" : "お",}
        self.k_column = {"ka" : "か",
                         "ki" : "き",
                         "ku" : "く",
                         "ke" : "け",
                         "ko" : "こ",}
        self.s_column = {"sa" : "さ",
                         "shi" : "し",
                         "su" : "す",
                         "se" : "せ",
                         "so" : "そ",}
        self.t_column = {"ta" : "た",
                         "chi" : "ち",
                         "tsu" : "つ",
                         "te" : "て",
                         "to" : "と",}
        self.n_column = {"na" : "な",
                         "ni" : "に",
                         "nu" : "ぬ",
                         "ne" : "ね",
                         "no" : "の",}
        self.h_column = {"ha" : "は",
                         "hi" : "ひ",
                         "fu" : "ふ",
                         "he" : "へ",
                         "ho" : "ほ",}
        self.m_column = {"ma" : "ま",
                         "mi" : "み",
                         "mu" : "む",
                         "me" : "め",
                         "mo" : "も",}
        self.y_column = {"ya" : "や",
                         "yu" : "ゆ",
                         "yo" : "よ",}
        self.r_column = {"ra" : "ら",
                         "ri" : "り",
                         "ru" : "る",
                         "re" : "れ",
                         "ro" : "ろ",}
        self.w_column = {"wa" : "わ",
                         "wo" : "を",
                         "n"  : "ん",}

        self.all_hiragana = {}

        all_dicts = (self.basic_vowels,
                     self.k_column,
                     self.s_column,
                     self.t_column,
                     self.n_column,
                     self.h_column,
                     )
        for dictionary in all_dicts:
            for (k, v) in dictionary.items():
                self.all_hiragana[k] = v

        self.strlen = strlen if (strlen > 0) else 13
        self.adaptive = adaptive

    def create_challenge(self):
        """
        creates and returns the random hiragana string.
        """

        strlen = self.strlen
        challenge = []
        while strlen:
            challenge.append(random.choice(tuple(self.all_hiragana.values())))
            strlen -= 1
        return "".join(challenge)

    def score_challenge(self, response, challenge):
        """
        checks the user input to see if the result is correct.
        @param response:    a space-delimited romaji string input by
                            the user.
        @param challenge:   a string in hiragana that this instance
                            generated to challenge the user.
        @return:            a tuple; the integer score and the hiragana
                            conversion of the response.
        """

        response_h_list = []
        try:
            for romaji in response.split():
                response_h_list.append(self.all_hiragana[romaji])
        except KeyError:
            return (0, "")
        response_h = "".join(response_h_list)
        if response_h == challenge:
            return (len(challenge), response_h)
        return (0, response_h)

    def collect_input(self):
        """
        collects one line of user input and chomps the trailing newline.
        @return:    the collected string.
        """
        user_input = sys.stdin.readline()
        return user_input[:-1]

    def main(self):
        """
        the main routine in which the game runs.
        """
        running_score = 0
        while True:
            challenge = self.create_challenge()
            print(challenge)

            response = self.collect_input()
            (score, response_h) = self.score_challenge(response, challenge)
            if score:
                running_score += score
            else:
                print("KABOOM! Your final score was %d." % (running_score,))
                print("You erred thus:\n%s" % (response_h,))
                return 0


if __name__ == "__main__":
    game = BasicHiragana(1)
    sys.exit(game.main())

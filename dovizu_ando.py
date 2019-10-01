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
                     self.m_column,
                     self.y_column,
                     self.r_column,
                     self.w_column,
                     )
        for dictionary in all_dicts:
            for (k, v) in dictionary.items():
                self.all_hiragana[k] = v

        self.strlen = strlen if (strlen > 0) else 1
        self.adaptive = adaptive
        self.hiragana_vals = tuple(self.all_hiragana.values())

    def create_challenge(self):
        """
        creates and returns the random hiragana string.
        """

        strlen = self.strlen
        challenge = []
        while strlen:
            challenge.append(random.choice(self.hiragana_vals))
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
        response_orig_list = response.split()
        for romaji in response_orig_list:
            try:
                response_h_list.append(self.all_hiragana[romaji])
            except KeyError:
                response_h_list.append("?")

        response_h = "".join(response_h_list)
        score = 0
        if response_h == challenge:
            score = len(response_h)
        else:
            for car_tup in \
                    zip(response_h_list, [car for car in challenge]):
                if car_tup[0] == car_tup[1]:
                    score += 1
        return (score, response_h)

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
        running_total = 0
        game_active = True

        while game_active:
            challenge = self.create_challenge()
            print(challenge)

            response = self.collect_input()
            (score, response_hiragana) = self.score_challenge(
                response, challenge)
            running_score += score
            running_total += self.strlen
            print("{} - {} / {} == {:.3f}%".format(
                response_hiragana if score < self.strlen else "Congratulations",
                running_score,
                running_total,
                100 * running_score / running_total))

            # Terminates the busy loop if this round was a total gas.
            game_active = True if score else False


if __name__ == "__main__":
    try:
        game = BasicHiragana(int(sys.argv[1]))
    except (IndexError, ValueError) as excepted:
        game = BasicHiragana(1)
    try:
        sys.exit(game.main())
    except KeyboardInterrupt:
        sys.exit(0)

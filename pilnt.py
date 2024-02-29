#!/usr/bin/python -i

import sys

"""
A horrendous implementation of a reddit (tumblr?) joke.

It would seem (from the diagnostic print at the end of
`PiNumber.__build_table()`) that pi to 100,000 digits doesn't contain
all possible 5-digit numbers. We're only good to 4 digits.
"""


class PiNumber:
    default_digit_len = 5
    noisy = True

    def get(self, num):
        table = self.__table(len(str(num)))
        value = table[num]
        assert self.pistr[value[0]:value[1]] == str(num)
        return value

    def __noisy(self, message):
        if not self.noisy:
            return
        sys.stdout.write(message)

    def __table(self, digit_len):
        try:
            return self.table[digit_len - 1]
        except IndexError:
            raise ValueError(f"{digit_len} > asks too much")

    def __table_is_full(self, digit_len):
        table = self.__table(digit_len)
        # Special case: there are ten single-digit numbers.
        if digit_len == 1:
            return len(table) == 10
        return len(table) == (10 ** digit_len - 10 ** (digit_len - 1))

    def __substr_int(self, start, length):
        return int(self.pistr[start:start+length])

    def __try_add_table_entry(self, start, end):
        length = end - start
        if self.__table_is_full(length):
            return
        table = self.__table(length)
        value = self.__substr_int(start, length)
        if len(str(value)) == length and value not in table:
            table[value] = (start, end)

    def __read_indices(self, start, end):
        while end > start:
            self.__try_add_table_entry(start, end)
            end -= 1

    def __init_table(self):
        self.table = [dict() for _ in range(self.default_digit_len)]

    def __build_table(self):
        start = 0
        end = start + self.default_digit_len
        while end <= len(self.pistr):
            self.__read_indices(start, end)
            start += 1
            end += 1
            if not start % 1000:
                self.__noisy(f"{start}, ")
            if not start % 10000:
                self.__noisy("\n")
        self.__noisy(str([len(tab) for tab in self.table]))
        self.__noisy("\n")

    def __init__(self):
        with open("pi.txt") as pfp:
            self.pistr = pfp.read().strip()
        self.__init_table()
        self.__build_table()


def print_missing_five_digit_numbers(pinum):
    for num in range(int(1e5)):
        try:
            pinum.get(num)
        except KeyError:
            print(num)


if __name__ == "__main__":
    pinum = PiNumber()

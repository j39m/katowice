#!/usr/bin/python

import sys

TENS = [
    # THERE'S NO TEN
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
]
DIGIT = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]
LOW = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
]


def _stringify_digit(num):
    return DIGIT[num - 1]


def _stringify_low(num):
    return LOW[num - 10]


def _stringify_twenty_or_greater(num):
    tens = int(num / 10)
    tens_repr = TENS[tens - 2]

    ones = num % 10
    if not ones:
        return tens_repr
    return f"{tens_repr}-{_stringify_digit(ones)}"


def stringify(num):
    assert num > 1
    assert num < 100
    if num < 10:
        return _stringify_digit(num)
    if num < 20:
        return _stringify_low(num)
    return _stringify_twenty_or_greater(num)


def main():
    for c in range(2, 100):
        tester = f"Here's a self-referential length of {stringify(c)}."
        if len(tester) == c:
            print(tester)
    return 0


if __name__ == "__main__":
    sys.exit(main())

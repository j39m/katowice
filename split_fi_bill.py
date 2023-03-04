#!/usr/bin/python3

"""
Splits our monthly Fi bill.
"""

import sys

from fractions import Fraction


class Splitter:
    """Main splitter context."""

    def __init__(self, *args):
        self.__parse(*args)
        self.__check_parsing()

    def __parse_single(self, dollar_amount: str) -> Fraction:
        (dollars, cents) = dollar_amount.split(".")
        return Fraction((int(dollars) * 100) + int(cents), 100)

    def __parse(self, *args):
        self.total = self.__parse_single(args[0])
        self.mine = self.__parse_single(args[1])
        self.sis = self.__parse_single(args[2])
        self.mom = self.__parse_single(args[3])

    def __report_parse_error(self, sum_total: Fraction):
        print(f"Total: {float(self.total)}")
        print(f"Mine:  {float(self.mine)}")
        print(f"Sis:   {float(self.sis)}")
        print(f"Mom:   {float(self.mom)}")
        print(f"Sum:   {float(sum_total)}")

    def __check_parsing(self):
        sum_total = sum((self.mine, self.sis, self.mom))
        try:
            assert sum_total == self.total
        except AssertionError as exc:
            self.__report_parse_error(sum_total)
            raise exc

    def __report_results(self, new_mine: Fraction, new_sis: Fraction):
        print(f"Total:    {float(self.total)}")
        print(f"YOU PAY:  {float(new_mine)}")
        print(f"Sis pays: {float(new_sis)}")

    def split(self):
        """Do the splitting."""
        new_mine = self.mine + (self.mom * Fraction(1, 2))
        new_sis = self.sis + (self.mom * Fraction(1, 2))
        assert sum((new_mine, new_sis)) == self.total

        self.__report_results(new_mine, new_sis)


def main():
    """Accepts `<total> <me> <sis> <mom>` as four decimal values."""
    splitter = Splitter(*sys.argv[1:5])
    splitter.split()


if __name__ == "__main__":
    sys.exit(main())

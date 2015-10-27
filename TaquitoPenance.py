#!/usr/bin/python2

"""
Plot my penance.
"""

import datetime
import matplotlib
import re

import os
import sys

PENANCE_FILE = os.path.join(os.path.expanduser("~"),
                            "Documents",
                            "personal",
                            "queenie")


class TaquitoReader(object):
    """
    An iterator to read the penance file line by line. Implemented for
    convenience.
    """

    def __init__(self, penance):
        self.infile = open(penance, "r")
        self.start_re = re.compile(r'^\s*/\*\s+PENANCE\s+\*/.*$')
        self.end_re = re.compile(r'^\s*/\*[^*]*\*/.*$')
        self.comment_re = re.compile(r'^\s*#.*$')
        self.curr_state = 0     # 0: search for header
                                # 1: iterate through data
                                # 2: ended

    def __iter__(self):
        return self

    def next(self):
        """
        Reads the input file until it gets to the /* PENANCE */ section, then
        begins returning data lines until it hits a new section or the end of
        the file.
        """

        inload = self.infile.readline()
        while inload and not self.curr_state:
            if self.start_re.search(inload):
                self.curr_state += 1
            inload = self.infile.readline()

        while inload and self.curr_state == 1:
            stripped = inload.strip()
            if self.end_re.search(stripped):
                self.curr_state += 1
                break
            if (self.comment_re.search(stripped)
                    or not stripped):
                inload = self.infile.readline()
                continue
            return stripped

        raise StopIteration


class TaquitoPenance(object):

    def __init__(self, *args):

        penance = None
        if len(args) > 1:
            penance = args[1]
        if not penance:
            self.penance = TaquitoReader(PENANCE_FILE)
        else:
            self.penance = TaquitoReader(penance)

    def parse(self, line):
        """
        Parses a single data entry from the penance file.
        """

        splitted_and_stripped = [x.strip() for x in line.split(",", 4)]
        (date, delta, total, comment) = splitted_and_stripped

        date = datetime.date(*[int(x) for x in date.split(".")])
        total = float(total)

        return (date, delta, total, comment)

    def run(self):

        for line in self.penance:
            (date, delta, total, comment) = self.parse(line)


if __name__ == "__main__":
    main_obj = TaquitoPenance(sys.argv)
    retv = main_obj.run()
    sys.exit(retv)

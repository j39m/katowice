#!/usr/bin/python3

"""
beans_refried.py

USAGE: beans_refried.py <file> [<maxlen>]

reflow text to spec.
Give a file and (optionally) a maximum line length. This script
will attempt to split on whitespace whereever overflow is
detected.
"""

import sys
import os
import string


class refried_beans:

    def __init__(self, args):

        self.name = "beans_refried.py"

        if not args:
            self.snark("Assuming stdin for reflow. Start inputing.")
            self.reflow_file = sys.stdin
        else:
            self.reflow_file = None
            if not os.path.exists(args[0]):
                if args[0] == "-":
                    self.reflow_file = sys.stdin
                else:
                    self.snark("Couldn't find file `%s!'" % args[0])
            try:
                self.reflow_file = open(args[0], "r")
            except IOError:
                self.snark("Couldn't open file `%s!'" % args[0])

        self.line_len = 78
        if len(args) > 1:
            try:
                self.line_len = int(args[1])
            except ValueError:
                self.snark("malformed line length arg?")
                self.line_len = 78

        return None


    def fry(self):

        if not self.reflow_file:
            return 1
        charcount = 0
        splitindex = 0
        for line in self.reflow_file:
            line = line.rstrip()
            while line:
                if len(line) < self.line_len:
                    break
                if line[charcount] in string.whitespace:
                    splitindex = charcount
                if charcount > self.line_len:
                    if splitindex:
                        lineout = line[:splitindex]
                        line = line[splitindex+1:]
                        print(lineout)
                        charcount = 0
                        splitindex = 0
                charcount += 1
            print(line)


    def snark(self, errstr):
        sys.stderr.write("%s: %s\n" % (self.name, str(errstr)))
        sys.stderr.flush()
        return 0


if __name__ == "__main__":
    retv = refried_beans(sys.argv[1:])
    sys.exit(retv.fry())

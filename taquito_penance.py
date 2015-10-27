#!/usr/bin/python2

"""
Plot my penance.
"""

import datetime
import matplotlib

import os
import sys

PENANCE_FILE = os.path.join(os.path.expanduser("~"),
                            "Documents",
                            "personal",
                            "queenie")

class TaquitoPenance(object):

    def __init__(self, *args):

        penance = None
        if len(args) > 1:
            penance = args[1]
        if not penance:
            self.penance = open(PENANCE_FILE, "r")
        else:
            self.penance = open(penance, "r")

    def run(self):
        print("HELLO CHAPS!")


if __name__ == "__main__":
    main_obj = TaquitoPenance(sys.argv)
    retv = main_obj.run()
    sys.exit(retv)

#!/usr/bin/env python3

"""
Playing with the notion of a reflection of a string; specifically,
sorting by the reflection of a string should be equivalent to reverse-
sorting the strings.

More generally, this script is a general proof that sorting strings by
their reflections is NOT equivalent to sorting strings in reverse.
"""


import string
import random
import sys
import time


class beetsup(object):

    def __init__(self):

        self.max_strlen = 26
        self.max_listlen = 104
        self.max_runs = 1300
        self.palette = string.printable
        self.init_time = int(time.time())
        forward_mapping = {}
        reverse_mapping = {}
        alphabet = [x for x in self.palette]
        alphabet.sort()
        self.alphabet_len = len(alphabet)

        i = 0
        for letter in alphabet:
            forward_mapping[i] = letter
            i += 1
        for index in forward_mapping:
            reverse_mapping[forward_mapping[index]] = index

        self.forward_mapping = forward_mapping
        self.reverse_mapping = reverse_mapping

        print("Using %d runs, lists of length %d, "
              "and strings of max length %d."
              % (self.max_runs, self.max_listlen, self.max_strlen))
        return None

    def ranstr(self):
        """
        a random string of len self.max_strlen
        """

        retv = []
        lim = random.choice(range(1,self.max_strlen))
        while lim:
            retv.append(random.choice(self.palette))
            lim -= 1
        return "".join(retv)

    def ranlist(self):
        """
        a list of len self.max_listlen random strings
        """

        retv = []
        lim = self.max_listlen
        while(lim):
            retv.append(self.ranstr())
            lim -= 1
        return retv

    def complement(self, instring):
        """
        the ``complement'' of a string
        """
        
        retv = []
        for letter in instring:
            retv.append(self.forward_mapping[self.alphabet_len - 1
                                             - self.reverse_mapping[letter]])
        return "".join(retv)

    def complements(self, stringlist):
        """
        deprecated iterator repeatedly generating string complements given
        some input list
        """
        
        retv = []
        for string in stringlist:
            retv.append(self.complement(string))
        return retv

    def test(self):
        failed_count = 0
        for run in range(self.max_runs):
            tester = self.ranlist()
            if (sorted(tester, reverse=True) !=
                    sorted(tester, key=lambda x: self.complement(x))):
                print("FAILED! (run %d)" % run)
                failed_count += 1
        now = int(time.time())
        print("%d runs passed and %d runs failed in %d seconds."
              % (self.max_runs - failed_count,
                 failed_count,
                 now - self.init_time)
             )
        return failed_count

    def work(self):
        """
        entry point
        """
        return self.test()


def main():
    goer = beetsup()
    return goer.work()

if __name__ == "__main__":
    sys.exit(main())

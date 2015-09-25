#! /usr/bin/env python3

import random
import codecs

# modular exponentiation. 

def expm(base, exponent, mod): 
  base_recall = base
  while (exponent != 1): 
    base *= base_recall
    base %= mod
    exponent -= 1
  return base

# remove every other character per line in a text file 
# of particular ascii art. 

def sat(filename): 
  fp = codecs.open(filename, "r", encoding = "utf-8")
  out = codecs.open("satsuki-out", "w", encoding = "utf-8")
  for line in fp: 
    strout = " "
    i = 0
    while (i < len(line)): 
      if (i % 3 == 1) or (i%3 == 2): 
        strout += line[i]
      i += 1
    if (strout[-1] != '\n'): 
      strout += '\n'
    out.write(strout)

  return 

#sat("satsuki")


class ProbabilisticFailure:
    """
    simulates probablistic failure of a linear network
    """

    def __init__(self, P=.78, N=78, K=13, TEST_TIMES=1300):
        self.P = P      # probability of failure
        self.N = N      # number of hops
        self.K = K      # number of retries in event of failure
        self.TEST_TIMES = TEST_TIMES # number of tests to run over a lot of hops

    def single_hop(self):
        retries = self.K
        while retries:
            result = random.random()
            if result > self.P:
                return True
            retries -= 1
        return False

    def many_hops(self):
        hops = self.N
        while hops:
            if not self.single_hop():
                return False
            hops -= 1
        return True

    def many_trials(self):
        trials = self.TEST_TIMES
        failures = 0
        successes = 0
        while trials:
            if self.many_hops():
                successes += 1
            else:
                failures += 1
            trials -= 1
        return (successes, failures)

    def diagnose(self):
        (successes, failures) = self.many_trials()
        expected = (1-(self.P**(self.K)))**self.N
        msg = str("expected %03f, got %03f"
                  % (expected, successes/self.TEST_TIMES))
        print(msg)

klaus = ProbabilisticFailure(.65)
klaus.diagnose()

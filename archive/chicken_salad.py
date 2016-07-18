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
    simulates probablistic failure of a linear network. That is,

    [ SIMPLE FORMULATION ]
    Given some linear network
    A -- B -- C -- D -- E -- F -- G [snip]
    with N links, each with identical bandwidth, propagation delay,
    and probability of dropping the packet being transmitted (!!),
    find the probability of a packet making it to the end of the line.
    Each waystation attempts k retransmissions in case of link failure.

    [ HARD FORMULATION ]
    Given the same conditions as above, cut out each individual link's
    ability to re-transmit; treat each link failure as a total line failure
    and have A re-transmit the packet from the start. A is allowed
    k re-transmissions in total, and failures can happen at any link.

    Note that every link is omniscient about the packet state; X is not
    made aware of a link failure by failing to receive a Y ACK - rather,
    it "just knows" (for the sake of this problem) when the packet doesn't
    go through. Similarly, for the hard formulation, A link failure between
    P and Q doesn't propagate all the way back to A by a shaky series of
    error packets. A will "just know" when any link fails. The code is
    written to reflect this.
    """

    def __init__(self, P=.78, N=78, K=13, TEST_TIMES=5200):
        self.P = P      # probability of failure
        self.N = N      # number of hops
        self.K = K      # number of retries in event of failure
        self.TEST_TIMES = TEST_TIMES # number of tests to run over a lot of hops

    def single_hop(self, mode):
        i = self.K
        while i:
            i -= 1
            result = random.random()
            if result > self.P:
                return True
            elif mode == "hard":
                break
        return False

    def many_hops(self, mode):
        hops = self.N
        while hops:
            hops -= 1
            if not self.single_hop(mode):
                return False
        return True

    def many_trials(self, mode):
        trials = self.TEST_TIMES
        failures = 0
        successes = 0
        outcome = None
        while trials:
            if mode == "hard":
                retries = self.K
                while retries:
                    outcome = self.many_hops(mode)
                    if outcome:
                        break
                    retries -= 1
            elif mode == "simple":
                outcome = self.many_hops(mode)
            if outcome:
                successes += 1
            else:
                failures += 1
            trials -= 1
        return (successes, failures)

    def diagnose(self, mode):
        (successes, failures) = self.many_trials(mode)
        if mode == "simple":
            expected = (1-(self.P**(self.K)))**self.N
        elif mode == "hard":
            expected = 1 - (1 - (1-self.P)**self.N)**self.K
        else:
            expected = -1
        inits = str("P = %f, N = %d, K = %d; %d trials - "
                    % (self.P, self.N, self.K, self.TEST_TIMES))
        msg = str("expected %03f, got %03f"
                  % (expected, successes/self.TEST_TIMES))
        print(str("%s%s" % (inits, msg)))


def test_PF():
    for n in range(5):
        for k in range(13):
            local_p = random.random()
            local_n = n
            local_k = k
            klaus = ProbabilisticFailure(local_p, local_n, local_k)
            klaus.diagnose("hard")

if __name__ == "__main__":
    test_PF()
#! /usr/bin/env python

# modular exponentiation. 

def expm(base, exponent, mod): 
  base_recall = base
  while (exponent != 1): 
    base *= base_recall
    base %= mod
    exponent -= 1
  return base

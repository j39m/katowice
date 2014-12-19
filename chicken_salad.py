#! /usr/bin/env python

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

import codecs 
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

sat("satsuki")

#! /usr/bin/env python3

# grape_chair.py solves Layton puzzle 95. Oh my god. 
# 
# My dyslexic girlfriend provided a solution in three minutes 
# through intuition: the median value, 5, should be placed in the 
# center, and then observing a "pinching" of sorts would then 
# place corresponding pairs summing to 10 in a ring around the 
# five; the values across every line then sum to 15 all around. 

def first (A, B): 
  return 2 + A + B

def second (C, D, E): 
  return C + D + E

def third (F, G): 
  return F + 1 + G

def fourth (C, F): 
  return 2 + C + F

def fifth (A, D): 
  return 1 + A + D

def sixth (B, E, G): 
  return B + E + G

def seventh (D, G): 
  return 2 + D + G

def eighth (F, D, B): 
  return F + D + B

# main begins here. permute across {A..G} belongs in {3..9}
# if sums on all are equal print sum and permutations

tiles_master = [3, 4, 5, 6, 7, 8, 9]
limit_a = len(tiles_master)

a = 0

while (a < limit_a): 
  A = tiles_master[a]
  tiles_a = tiles_master[0:a] + tiles_master[(a+1):limit_a]
  a += 1
  limit_b = limit_a - 1
  b = 0
  while (b < limit_b): 
    B = tiles_a[b]
    tiles_b = tiles_a[0:b] + tiles_a[(b+1):limit_b]
    b += 1
    limit_c = limit_b - 1
    c = 0
    while (c < limit_c): 
      C = tiles_b[c]
      tiles_c = tiles_b[0:c] + tiles_b[(c+1):limit_c]
      c += 1
      limit_d = limit_c - 1
      d = 0
      while (d < limit_d): 
        D = tiles_c[d]
        tiles_d = tiles_c[0:d] + tiles_c[(d+1):limit_d]
        d += 1
        limit_e = limit_d - 1
        e = 0
        while (e < limit_e): 
          E = tiles_d[e]
          tiles_e = tiles_d[0:e] + tiles_d[(e+1):limit_e]
          e += 1
          limit_f = limit_e - 1
          f = 0
          while (f < limit_f): 
            F = tiles_e[f]
            tiles_f = tiles_e[0:f] + tiles_e[(f+1):limit_f]
            f += 1
            limit_g = limit_f - 1
            g = 0
            while (g < limit_g): 
              G = tiles_f[g]
              if (first(A, B) == second(C, D, E) == third(F, G) 
              == fourth(C, F) == fifth(A, D)
              == sixth(B, E, G) == seventh(D, G) 
              == eighth(F, D, B)): 
                print ("A = %d, B = %d, C = %d, \
D = %d, E = %d, F = %d, G = %d" \
                %(A, B, C, D, E, F, G))
                # exit(); 
              tiles_g = tiles_f[0:g] + tiles_f[(g+1):limit_g]
              g += 1

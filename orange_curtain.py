#! /usr/bin/env python3 

# solves Layton Puzzle 100. 

# is_square takes four points and determines if they form
# a square. Since this script iterates across every 
# possible permutation anyway, we need not check order: 
# we can restrict our square-checking to one schema and 
# trust that all other permutations will eventually 
# be covered. The schema we'll use will be 

#      A - B 
#      |   | 
#      D - C 

def is_square (A, B, C, D): 
  
  # check equilength sides and perpendicular adjacency.

  # squared sidelengths can also be compared without 
  # loss of accuracy. 
  sqlen_ab = pow(A[0]-B[0],2) + pow(A[1]-B[1],2)
  sqlen_bc = pow(B[0]-C[0],2) + pow(B[1]-C[1],2)
  sqlen_cd = pow(C[0]-D[0],2) + pow(C[1]-D[1],2)
  sqlen_da = pow(D[0]-A[0],2) + pow(D[1]-A[1],2)

  equal_sidelengths = (sqlen_ab == sqlen_bc == sqlen_cd ==
  sqlen_da)

  # check if AB is perp to BC and BC is perp to CD. 
  # isolate numerators / denominators, extract signs. 
  # compare each separately to avoid pesky float-point
  # errors and divide-by-zeros
  # we require nume_x == deno_y and nume_y == deno_x and 
  # that their slopes have opposing signs 

  nume_ab = A[1]-B[1]
  deno_ab = A[0]-B[0]
  sign_ab = nume_ab*deno_ab
  nume_ab = abs(nume_ab) 
  deno_ab = abs(deno_ab)

  nume_bc = B[1]-C[1]
  deno_bc = B[0]-C[0]
  sign_bc = nume_bc*deno_bc
  nume_bc = abs(nume_bc)
  deno_bc = abs(deno_bc)

  nume_cd = C[1]-D[1]
  deno_cd = C[0]-D[0]
  sign_cd = nume_cd*deno_cd
  nume_cd = abs(nume_cd)
  deno_cd = abs(deno_cd)

  perp_abbc = (nume_ab==deno_bc) and (nume_bc==deno_ab) and (sign_ab == -1 * sign_bc)
  perp_bccd = (nume_bc==deno_cd) and (nume_cd==deno_bc) and (sign_bc == -1 * sign_cd)

  return (equal_sidelengths and perp_abbc and perp_bccd) 






basic = [(0,0),(1,0),(1,1),(0,1)]
master_pins = [(1,0), (2,0), (4,0), 
(0,1), (1,1), (2,1), (3,1),
(1,2), (2,2), (3,2), (4,2), (5,2),
(0,3), (1,3), (3,3), (4,3), (5,3),
(0,4), (1,4), (2,4), (3,4), (4,4), (5,4),
(0,5), (1,5), (2,5), (3,5), (4,5)]

from itertools import permutations
from random import shuffle

#exit()

shuffle(master_pins)

counter = 0

for state in permutations(master_pins): 
  counter += 1
  # recall "state" is a list of pins. Feed them four by four 
  # into is_square...
  if ( is_square(state[0],state[1], state[2], state[3])
  and is_square(state[4], state[5], state[6], state[7])
  and is_square(state[8], state[9], state[10], state[11])
  and is_square(state[12], state[13], state[14], state[15])
  and is_square(state[16], state[17], state[18], state[19])
  and is_square(state[20], state[21], state[22], state[23])
  and is_square(state[24], state[25], state[26], state[27]) ):
    i = 0
    while (i < 28): 
      print(state[i], state[i+1], state[i+2], state[i+3])
      i+=4
  
  if (counter % 13000000 == 0): 
    print ("processed %d permutations." % counter)

exit()

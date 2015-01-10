#! /usr/bin/env python3 

# solves Layton Puzzle 100. 

master_pins = [(1,0), (2,0), (4,0), 
(0,1), (1,1), (2,1), (3,1),
(1,2), (2,2), (3,2), (4,2), (5,2),
(0,3), (1,3), (3,3), (4,3), (5,3),
(0,4), (1,4), (2,4), (3,4), (4,4), (5,4),
(0,5), (1,5), (2,5), (3,5), (4,5)]

from itertools import permutations




# is_square takes four points and determines if they can be a 
# square (whether in the obvious way or in a baseball diamond).
# Since this script iterates across every possible permutation 
# anyway, we need not check order: we can restrict our square-
# checking to one schema and trust that all other permutations
# will eventually be covered. 
# the schema we'll use will be 

#      A - B 
#      |   |
#      C - D

# or 

#         A
#       /   \
#      B     C
#       \   /
#         D

def is_square (A, B, C, D): 
  
  # basically, compare side or diagonal lengths and make sure 
  # corresponding vertices share same x or y coordinates
  
  is_ordinary = ( (A[0] == C[0]) and (B[0] == D[0]) 
  and (A[1] == B[1]) and (C[1] == D[1]) 
  and (abs(A[0]-B[0]) == abs(A[1]-C[1])) )

  is_diamond = ( (A[0] == D[0]) and (B[1] == C[1])
  and (abs(A[0]-B[0]) == abs(A[0]-C[0]) 
  == abs(B[1]-A[1]) == abs(B[1]-D[1])) )

  return (is_diamond or is_ordinary)

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
    while (i < 7): 
      print(state[i], state[i+1], state[i+2], state[i+3])
      i+=1
  
  if (counter % 1300000 == 0): 
    print ("processed %d permutations." % counter)

exit()

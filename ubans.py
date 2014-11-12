#! /usr/bin/python

### USAGE: python -i <thisfilename> 
# then do "ubans ('someword')" 
# the pet example is "ubans ('ubans') "

# feed me a string. I spit out the same word as a list. 
def setarr ( word ): 
  retval = []
  i = 0; 
  while ( i < len(word) ): 
    retval.append(word[i])
    i += 1
  return retval 

# remaining is a list of remaining chars; currword is a string
def nubas( remaining, currword = "" ): 
  if (remaining == []): 
    print(currword+" "+currword)
    return 
  else: 
    i = 0
    while ( i < len(remaining) ) : 
      nubas( remaining[0:i]+remaining[i+1:], currword+remaining[i])
      i += 1
  return 

def ubans( scrambleme ) :
  return nubas(setarr(scrambleme))

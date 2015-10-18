#! /usr/bin/env python2 

# unskipper.py will prune all skipcounts from your Quod Libet library; 
# the resulting lack of '~#skipcount' in your per-song entries will all
# be interpreted by QL as being skipcount 0. 


import os
import os.path
import sys
import shutil

try: 
  import pickle 
except ImportError: 
  print ("Error importing pickle! Make sure you have the Python pickle module.")
  sys.exit(1) 

path_to_songs = os.getenv('HOME') + "/.quodlibet/songs"


# load the pickled library. returns the songs pickle if it can find it, 
# None if it cannot. 
def load_library(): 
  try: 
    sfh = open(path_to_songs, 'r')
    songs = pickle.load(sfh)
    sfh.close()
  except IOError: 
    print ("Error loading songs pickle. Does '%s' exist?" % path_to_songs)
    return None 

  return songs


# in case we muck things up, have a backup on hand 
def backup_library(): 

  path_to_bkup =  os.getenv('HOME') + "/.quodlibet/unpruned_songs"

  if os.path.isfile(path_to_bkup):
    print ("Couldn't backup current library state. Aborting.") 
    print ("Please remove '%s' before trying again." % path_to_bkup) 
    return False 

  sfh = open(path_to_songs, "rb")
  bfh = open(path_to_bkup, "wb")
  shutil.copyfileobj(sfh, bfh)
  sfh.close()
  bfh.close()

  print ("Current library state copied to %s" % path_to_bkup) 
  return True 


# weed out skipcounts wherever they may be
def prune_skips(song_pickle): 
  
  found_skips = False # make me true if we need to prune anything

  for song in song_pickle: 
    if ('~#skipcount' in song): 
      found_skips = True
      try: 
        print ("prune: %d skips on '%s'" % (song.pop('~#skipcount'), song['title'] ) )
      except KeyError:  # if song is untitled???
        print ("prune: %d skips on untitled song" % song.pop('~#skipcount') )

  if (not found_skips): 
    print ("No skips pruned.") 

  # write the finished pickle down 
  try: 
    pickle.dump(song_pickle, open(path_to_songs, 'w')) 
  except pickle.PicklingError: 
    print ("Unpicklable library! NANISORE?") 
    return 1 

  return 0 


def query_library_by_tag(lib, val, tag="artist", corr="~#playcount", rkey="title"):
    """
    query the library "lib" by the tag "tag," searching for entries with tag
    value "val."
    returns a dictionary of the results.
    """

    retv = {}
    for song in lib:
        if tag in song and val in song[tag] and corr in song:
            try:
                lkey = song[rkey]
            except KeyError:
                lkey = None
            if lkey in retv:
                if not isinstance(retv[lkey], list):
                    retv[lkey] = [retv[lkey],]
                retv[lkey].append(song[corr])
            else:
                retv[lkey] = song[corr]

    return retv

# the main routine 
def main(): 
  
  # the library is just a list of dictionaries, one dict per song
  songs = load_library() 

  if songs is None: 
    return 1 

  # try not to mess everything up. keep a backup, just in case. 
  if not backup_library(): 
    return 1 

  # mutate the songs pickle. 
  ret = prune_skips(songs) 

  return ret 


##### EXECUTION BEGINS HEEEERREEEEE #####

if __name__ == "__main__": 
  ret = main()
  sys.exit(ret) 

#! /usr/bin/env python2

# unskipper.py will prune all skipcounts from your Quod Libet library;
# the resulting lack of '~#skipcount' in your per-song entries will all
# be interpreted by QL as being skipcount 0.


import os
import sys
import shutil
import pickle

HOME = os.getenv("HOME")
QLDIR = ".quodlibet"
PATH_TO_SONGS = os.path.join(
    HOME,
    QLDIR,
    "songs",
)
PATH_TO_BKUP = os.path.join(
    HOME,
    QLDIR,
    "unpruned",
)


def load_library():
    sfh = open(PATH_TO_SONGS, 'r')
    songs = pickle.load(sfh)
    sfh.close()
    return songs

def backup_library():
    sfh = open(PATH_TO_SONGS, "rb")
    bfh = open(PATH_TO_BKUP, "wb")
    shutil.copyfileobj(sfh, bfh)
    sfh.close()
    bfh.close()
    return 0

def prune_skips(song_pickle):
    """Main function for pruning skips from a pickle."""
    found_skips = False
    skipfmt = "prune {:d} skips on ``{:s}.''"
    for song in song_pickle:
        try:
            skipmsg = skipfmt.format(song.pop("~#skipcount"), song["title"])
            found_skips = True
            print(skipmsg)
        except KeyError:
            continue
    # write the finished pickle down
    try:
        pickle.dump(song_pickle, open(PATH_TO_SONGS, "w"))
    except pickle.PicklingError:
        print ("NANISORE?")
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

def main():
    """The main entry point."""
    songs = load_library()
    backup_library()

    return prune_skips(songs)


##### EXECUTION BEGINS HEEEERREEEEE #####

if __name__ == "__main__":
    ret = main()
    sys.exit(ret)

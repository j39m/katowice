#! /usr/bin/env python3

# unskipper.py will prune all skipcounts from your Quod Libet library;
# the resulting lack of '~#skipcount' in your per-song entries will all
# be interpreted by QL as being skipcount 0.


import os
import sys
import shutil
import quodlibet.library

HOME = os.getenv("HOME")
QLDIR = ".quodlibet"
SONGS_PATH = os.path.join(
    HOME,
    QLDIR,
    "songs",
)
BKUP_PATH = os.path.join(
    HOME,
    QLDIR,
    "songs.bk",
)


def load_library():
    songs = quodlibet.library.init()
    songs.load(SONGS_PATH)
    return songs


def backup_library():
    with open(SONGS_PATH, "rb") as spt, open(BKUP_PATH, "wb") as bpt:
        shutil.copyfileobj(spt, bpt)


def prune_skips(song_pickle):
    """Main function for pruning skips from a pickle."""
    raise NotImplementedError
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
        pickle.dump(song_pickle, open(SONGS_PATH, "w"))
    except pickle.PicklingError:
        print ("NANISORE?")
        return 1
    return 0


def main():
    """The main entry point."""
    raise NotImplementedError
    backup_library()
    songs = load_library()

    return 0


##### EXECUTION BEGINS HEEEERREEEEE #####

if __name__ == "__main__":
    ret = main()
    sys.exit(ret)

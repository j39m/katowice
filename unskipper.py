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


def _query_callable(songs, tag, val_callable):
    """
    Helper function for queries passing a callable truth.
    Called by query().
    """
    ret = dict()
    for (spath, sdict) in songs.iteritems():
        try:
            if val_callable(sdict[tag]):
                ret[spath] = sdict
        except KeyError:
            continue
    return ret


def _query_simple(songs, tag, val):
    """Helper function for simple queries. Called by query()."""
    return _query_callable(songs, tag, lambda x: x == val)


def query(songs, tag, val=None, val_callable=None):
    """
    Given a Quod Libet library, return the sub-dict of songs that contain
    tags with the prescribed values.

    You can call query() with either some simple value passed in for ``val''
    or a more complex function for ``val_callable.''

    @param songs        the Quod Libet library to search
    @param tag          the tag to query for
    @param val          a simple comparable s.t. we can eval ``blah == val.''
    @param val_callable a callable s.t. we can eval ``val_callable(blah).''
    """
    if val is not None:
        return _query_simple(songs, tag, val)
    elif val_callable is not None:
        return _query_callable(songs, tag, val_callable)
    return None


def prune_skips(song_pickle):
    """Main function for pruning skips from a pickle."""
    raise NotImplementedError


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

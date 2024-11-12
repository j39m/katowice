import gi
gi.require_version("PangoCairo", "1.0")

import quodlibet
import quodlibet.cli
import quodlibet.library
import pathlib
import shutil
import sys


SONGS_PATH = pathlib.Path(quodlibet.get_user_dir()) / "songs"
BKUP_PATH = SONGS_PATH.with_suffix(".bk")


def quodlibet_is_running():
    return quodlibet.cli.is_running()


def backup_library():
    shutil.copy2(SONGS_PATH, BKUP_PATH)


def load_library():
    sys.modules.pop("gi.repository.Gtk")
    quodlibet.init()
    return quodlibet.library.init(SONGS_PATH)


def save_library(songs):
    tmppath = SONGS_PATH.with_suffix(".tmp")
    songs.save(tmppath)
    tmppath.rename(SONGS_PATH)


def _query_callable(songs, tag, val_callable):
    """
    Helper function for queries passing a callable truth.
    Called by query().
    """
    ret = {}
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

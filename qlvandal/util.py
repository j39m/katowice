import pathlib
import shutil
import sys

import gi

gi.require_version("PangoCairo", "1.0")
import quodlibet
import quodlibet.cli
import quodlibet.library

QL_USER_DIR = pathlib.Path(quodlibet.get_user_dir())
SONGS_PATH = QL_USER_DIR / "songs"
BKUP_PATH = SONGS_PATH.with_suffix(".bk")


def quodlibet_is_running():
    return quodlibet.cli.is_running()


class DontSaveLibrary(Exception):
    """
    A nonfatal exception that signals the `SongsContextManager` not to
    save the library and to swallow this exception, allowing control
    flow to continue.
    """

    def __init__(self, reason):
        self.reason = reason


class SongsContextManager:

    def __init__(self):
        sys.modules.pop("gi.repository.Gtk")
        quodlibet.init()
        self.songs = quodlibet.library.init(SONGS_PATH)

    def __enter__(self):
        return self.songs

    def __exit__(self, exc_type, exc_value, traceback):
        if exc_type == DontSaveLibrary:
            print(f"Not saving library: {exc_value.reason}")
            return True
        if quodlibet_is_running():
            raise ConnectionError("Quod Libet is running")
        if exc_type == exc_value == traceback == None:
            shutil.copy2(SONGS_PATH, BKUP_PATH)
            # Actually reading `save()`, this whole dance appears to be
            # redundant (ノ°益°)ノ
            tmppath = SONGS_PATH.with_suffix(".qlvandal")
            # Hmm. `atomic.py` seems to demand a string-like argument.
            self.songs.save(str(tmppath))
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
    if val_callable is not None:
        return _query_callable(songs, tag, val_callable)
    return None

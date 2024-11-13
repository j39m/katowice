import sys
import shutil
import pathlib
import traceback

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

    def __exit__(self, exc_type, exc_value, _stack_trace):
        if exc_type == DontSaveLibrary:
            print(f"Not saving library: {exc_value.reason}")
            return True
        if not (exc_type == exc_value == _stack_trace == None):
            print(traceback.format_exc())
            return False
        if quodlibet_is_running():
            raise ConnectionError("Quod Libet is running")

        shutil.copy2(SONGS_PATH, BKUP_PATH)
        # Actually reading `save()`, this whole dance appears to be
        # redundant (ノ°益°)ノ
        tmppath = SONGS_PATH.with_suffix(".qlvandal")
        # Hmm. `atomic.py` seems to demand a string-like argument.
        self.songs.save(str(tmppath))
        tmppath.rename(SONGS_PATH)


def query(songs, func):
    """
    Fully generic query that calls `func` to determine matching.
    """
    return {spath: sdict for spath, sdict in songs.items() if func(sdict)}

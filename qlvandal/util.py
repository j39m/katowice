import sys
import pathlib
import re
import shutil
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


def _swallow_keyerror(func):

    def inner(song, tag, val):
        try:
            return func(song, tag, val)
        except KeyError:
            return False

    return inner


def _split_value_lines(func):
    """
    Quod Libet stores multi-value tags as newline-separated strings.
    """

    def inner(song, tag, user_supplied_val):
        for val_line in song[tag].splitlines():
            if func(val_line, user_supplied_val):
                return True
        return False

    return inner


@_swallow_keyerror
@_split_value_lines
def match_regex(val, regex):
    return re.match(regex, val)


@_swallow_keyerror
@_split_value_lines
def match_fixed_value(val, user_supplied_val):
    return val == user_supplied_val


def query(songs, func):
    """
    Fully generic query that calls `func` to determine matching.
    """
    return {spath: sdict for spath, sdict in songs.items() if func(sdict)}

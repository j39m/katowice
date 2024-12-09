#!/usr/bin/python3

# Batch renamer for Quod Libet.

import sys
import pathlib
import qlvandal.util


def get_new_path(song_info):
    discnumber = int(song_info.get("discnumber", "1").split("/", 1)[0])
    tracknumber = int(song_info.get("tracknumber").split("/", 1)[0])

    path = pathlib.Path(song_info["~filename"])
    return (path,
            path.parent / f"{discnumber:03d}-{tracknumber:03d}{path.suffix}")


def _get_query():
    try:
        return sys.argv[1]
    except IndexError:
        return "album=/^Rachmaninov - Piano Concertos 1-4/"


def _get_dewit():
    try:
        return sys.argv[2] == "DEWIT"
    except IndexError:
        return False


def main():
    with qlvandal.util.SongsContextManager() as songs:
        selection = songs.query(_get_query())
        for song in selection:
            (current_path, new_path) = get_new_path(song)
            assert current_path.is_file()
            assert current_path.parent == new_path.parent
            print(f"{new_path} <- {current_path}")
            if _get_dewit():
                songs.rename(song, new_path)
    return 0


if __name__ == "__main__":
    sys.exit(main())

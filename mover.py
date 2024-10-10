#!/usr/bin/python3

# Batch renamer for Quod Libet.

import sys
import pathlib
import unskipper
import quodlibet.cli

DEWIT = sys.argv[1] == "dewit"


def get_new_path(song_info):
    discnumber = int(song_info.get("discnumber", "1").split("/", 1)[0])
    tracknumber = int(song_info.get("tracknumber").split("/", 1)[0])

    path = pathlib.Path(song_info["~filename"])
    return path.parent / f"{discnumber:03d}-{tracknumber:03d}{path.suffix}"


def main():
    assert not (DEWIT and quodlibet.cli.is_running())
    songs = unskipper.load_library()
    selection = unskipper.query(songs, "album",
                                val="Rachmaninov - Piano Concertos 1-4")

    for (_path, song_info) in selection.items():
        path = pathlib.Path(_path)
        assert str(path) == song_info["~filename"]
        assert path.is_file()
        new_path = get_new_path(song_info)

        assert path.parent == new_path.parent
        print(f"{new_path} <- {path}")
        if DEWIT:
            songs.rename(song_info, new_path)

    if DEWIT:
        unskipper.save_library(songs)
    return 0


if __name__ == "__main__":
    sys.exit(main())

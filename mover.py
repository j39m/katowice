#!/usr/bin/python3

# Batch renamer for Quod Libet.

import sys
import os
import os.path
import unskipper
import quodlibet.cli

DEWIT = sys.argv[1] == "dewit"

def get_new_path(song_info):
    discnumber = int(song_info.get("discnumber", "1").split("/", 1)[0])
    tracknumber = int(song_info.get("tracknumber").split("/", 1)[0])

    path = song_info["~filename"]
    extension = os.path.splitext(path)[1]
    dirname = os.path.dirname(path)

    return os.path.join(dirname, "{:03d}-{:03d}{}".format(
        discnumber, tracknumber, extension))

def main():
    assert not (DEWIT and quodlibet.cli.is_running())
    songs = unskipper.load_library()
    selection = unskipper.query(songs, "album",
            val="Rachmaninov - Piano Concertos 1-4")

    for (path, song_info) in selection.items():
        assert path == song_info["~filename"]
        assert os.path.isfile(path)
        new_path = get_new_path(song_info)

        assert os.path.dirname(path) == os.path.dirname(new_path)
        print("{} <- {}".format(new_path, path))
        if DEWIT:
            songs.rename(song_info, new_path)

    if DEWIT:
        unskipper.save_library(songs)
    return 0

if __name__ == "__main__":
    sys.exit(main())

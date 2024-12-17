#!/usr/bin/env python3

import sys
from pprint import pprint
from qlvandal import util

def main():
    with util.SongsContextManager() as songs:
        stale_poisons = songs.query("&(album=/Solo/, qlvandal_poison=/./)")
        pprint([song["~filename"] for song in stale_poisons])
        dewit = False
        try:
            dewit = sys.argv[1] == "DEWIT"
        except IndexError:
            pass
        if not dewit:
            raise util.DontSaveLibrary("not DEWIT")
        _ = [song.pop("qlvandal_poison") for song in stale_poisons]

if __name__ == "__main__":
    sys.exit(main())

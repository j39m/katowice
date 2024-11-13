import sys

import tomli
from pprint import pprint
from . import util

POISON_CONFIG = util.QL_USER_DIR / "qlpoison.toml"
POISON_LIBRARY_KEY = "qlvandal_poison"


class Poison:

    def __init__(self, poison_entry, songs):
        self.reason = poison_entry["reason"]
        self.query = poison_entry["query"]
        self._base_view = songs.query(self.query)
        self.applicable_songs = [
            song for song in self._base_view
            if song.get(POISON_LIBRARY_KEY, None) != self.reason
        ]
        self._enact()

    def __bool__(self):
        return bool(self.applicable_songs)

    def __str__(self):
        result = [f"## {self.reason}\n"]
        for song in self.applicable_songs:
            result.append(f"*   {song['title']}")
        result.append("")
        return "\n".join(result)

    def _enact(self):
        for song in self.applicable_songs:
            song[POISON_LIBRARY_KEY] = self.reason


def main():
    with open(POISON_CONFIG, "rb") as pfp, util.SongsContextManager() as songs:
        poison_top = tomli.load(pfp)
        poisons = [
            Poison(entry, songs) for entry in poison_top["poison"].values()
        ]
        applied = [print(p) for p in poisons if p]
        if not applied:
            raise util.DontSaveLibrary("no poisons to apply")
        if len(sys.argv) < 2 or sys.argv[1] != "DEWIT":
            raise util.DontSaveLibrary("not DEWIT")
    return 0

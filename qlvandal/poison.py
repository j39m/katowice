import sys

import tomli
from pprint import pprint
from . import util

POISON_CONFIG = util.QL_USER_DIR / "qlpoison.toml"
POISON_LIBRARY_KEY = "qlvandal_poison"


class Poison:

    @staticmethod
    def _parse_criterion(key, val):
        assert key != "reason", "BUG: didn't preprocess `reason` key"
        if key.endswith("_regex"):
            key = key.removesuffix("_regex")
            return lambda song: util.match_regex(song, key, val)
        return lambda song: util.match_fixed_value(song, key, val)

    def _get_applicable(self, songs):
        view = songs
        for criterion in self.criteria:
            view = util.query(view, criterion)
        self.applicable_songs = [
            song for song in view.values()
            if (POISON_LIBRARY_KEY not in song
                or song[POISON_LIBRARY_KEY] != self.reason)
        ]

    def __init__(self, poison_entry, songs):
        self.reason = poison_entry.pop("reason")
        self.criteria = [
            self._parse_criterion(key, val)
            for key, val in poison_entry.items()
        ]
        self._get_applicable(songs)

    def __str__(self):
        result = [f"## {self.reason}\n"]
        for song in self.applicable_songs:
            result.append(f"*   {song['title']}")
        result.append("")
        return "\n".join(result)


def main():
    with open(POISON_CONFIG, "rb") as pfp, util.SongsContextManager() as songs:
        poison_top = tomli.load(pfp)
        for (_, entry) in poison_top["poison"].items():
            poison = Poison(entry, songs)
            print(poison)
        if len(sys.argv) < 2 or sys.argv[1] != "DEWIT":
            raise util.DontSaveLibrary("not DEWIT")
    return 0

import random

from . import util


def main():
    """The main entry point."""
    roll_d20 = random.randint(1, 20)
    if roll_d20 < 15:
        print(f"Roll {roll_d20} < 15: doing nothing")
        return 0

    with util.SongsContextManager() as songs:
        skipped = [(song, song.pop("~#skipcount"))
                   for song in songs.query("#(skipcount>0)")]
        if not skipped:
            raise util.DontSaveLibrary("no skips pruned")
        for (sdict, skips) in skipped:
            print(
                f'Prune {skips} skip{"s" if skips > 1 else ""} on ``{sdict["title"]}\'\''
            )
    return 0

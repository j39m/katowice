import random

from . import util


def prune_skips(songs):
    """
    Prune all ``~#skipcount'' tags from the song library.
    Return a list of tuples (song_dict, skips) on all pruned songs.
    """
    SKIP_COUNT = "~#skipcount"
    have_skips = util.query(songs, SKIP_COUNT,
                            val_callable=lambda _: True).values()
    return [(song, song.pop(SKIP_COUNT)) for song in have_skips]


def _print_skips(skiplist):
    """
    Given a list as per return of prune_skips(), pretty-print songs that
    were impacted.
    """
    for (sdict, skips) in skiplist:
        print(
            f'Prune {skips} skip{"s" if skips > 1 else ""} on ``{sdict["title"]}\'\''
        )


def main():
    """The main entry point."""
    roll_d20 = random.randint(1, 20)
    if roll_d20 < 15:
        print(f"Roll {roll_d20} < 15: doing nothing.")
        return 0

    util.backup_library()
    songs = util.load_library()

    skipped = prune_skips(songs)
    if not skipped:
        print("No skips pruned.")
        return 0

    _print_skips(skipped)

    if util.quodlibet_is_running():
        print("Detected running Quod Libet - bailing!")
        return 1

    util.save_library(songs)
    return 0

import sys
import re
from pprint import pprint
from . import util


def Query(songs, query, filter_regex):
    titles = [s["title"] for s in songs.query(query)]
    return sorted(set([int(filter_regex.match(t).group(1)) for t in titles]))


def QueryCantatas(songs):
    TARGET_PLAYCOUNT = 7
    pprint(
        Query(
            songs,
            f"&(discsubtitle=cantatas, #(playcount<{TARGET_PLAYCOUNT}))",
            re.compile(r".+ BWV ([^\s]+) .+"),
        )
    )


def QueryBassoonConcerti(songs):
    TARGET_PLAYCOUNT = 6
    pprint(
        Query(
            songs,
            f"&(artist='Daniel Smith', #(playcount<{TARGET_PLAYCOUNT}))",
            re.compile(r".+ RV ([^\s]+) .+"),
        )
    )


def main():
    action_text = sys.argv[1]
    action_func = None
    if action_text.startswith("c"):
        action_func = QueryCantatas
    elif action_text.startswith("b"):
        action_func = QueryBassoonConcerti

    if action_func is None:
        return 1
    try:
        with util.SongsContextManager() as songs:
            action_func(songs)
            raise util.DontSaveLibrary("there ought to be a const view of this")
    except util.DontSaveLibrary:
        pass
    return 0

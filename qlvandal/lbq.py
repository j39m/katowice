import sys
import re
from pprint import pprint
from . import util


def QueryCantatas(songs):
    TARGET_PLAYCOUNT = 7
    titles = [
        s["title"]
        for s in songs.query(
            f"&(discsubtitle=cantatas, #(playcount<{TARGET_PLAYCOUNT}))"
        )
    ]
    PATTERN = re.compile(r".+ BWV ([^\s]+) .+")
    bwvs = sorted(set([int(PATTERN.match(t).group(1)) for t in titles]))
    pprint(bwvs)


def main():
    action_text = sys.argv[1]
    action_func = None
    if action_text == "c":
        action_func = QueryCantatas
    if action_func is None:
        return 1
    try:
        with util.SongsContextManager() as songs:
            action_func(songs)
            raise util.DontSaveLibrary("there ought to be a const view of this")
    except util.DontSaveLibrary:
        pass
    return 0

import sys

import tomli
from pprint import pprint
from . import util

POISON_CONFIG = util.QL_USER_DIR / "qlpoison.toml"


def main():
    with open(POISON_CONFIG, "rb") as pfp, util.SongsContextManager() as songs:
        poison = tomli.load(pfp)
        pprint(poison)
    return 0

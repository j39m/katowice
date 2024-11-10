import sys

import quodlibet.cli

from . import unskipper

def main():
    if quodlibet.cli.is_running():
        print("Detected running Quod Libet - bailing!")
        return 1
    return 0

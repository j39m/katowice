import sys

from . import util

def main():
    if util.quodlibet_is_running():
        print("Detected running Quod Libet - bailing!")
        return 1
    return 0

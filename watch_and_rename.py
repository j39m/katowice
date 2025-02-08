#!/usr/bin/python3

import sys
import argparse
import pathlib
import pyinotify

import logging

logging.basicConfig(stream=sys.stdout,
                    level=logging.INFO,
                    format='%(name)s: %(message)s')
logger = logging.getLogger("war")

CWD = "./"


def expect_nonempty_file(path):
    assert path.is_file()
    if not path.stat().st_size:
        logger.warning(f"Ruh-roh! “{path}” is empty.")


class EventHandler(pyinotify.ProcessEvent):
    # The defaults are actually set by `parse_args()`, but are retained
    # here to illustrate good defaults.
    def __init__(self, initial_index=1, prefix=""):
        self.index = initial_index
        self.prefix = prefix

    def target_filename(self, suffix):
        target = pathlib.Path(f"{self.prefix}{self.index:03d}{suffix}")
        assert not target.is_file()
        self.index += 1
        return target

    def process_IN_CLOSE_WRITE(self, event):
        name = pathlib.Path(event.name)
        expect_nonempty_file(name)
        target = self.target_filename(name.suffix)
        name.rename(target)
        expect_nonempty_file(target)
        logger.info(f"{target} <- {name}")


def _deduce_initial_index():
    files = [f for f in pathlib.Path(CWD).iterdir() if f.is_file()]
    return len(files) + 1


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("-i",
                        "--initial-index",
                        default=_deduce_initial_index(),
                        type=int)
    parser.add_argument("-p", "--prefix", default="")
    args = parser.parse_args()

    return EventHandler(args.initial_index, args.prefix)


def main():
    event_handler = parse_args()
    watch_manager = pyinotify.WatchManager()

    notifier = pyinotify.Notifier(watch_manager, event_handler)
    watch_manager.add_watch(CWD, pyinotify.IN_CLOSE_WRITE)
    logger.info(f"initial index is {event_handler.index}")

    notifier.loop()
    return 0


if __name__ == "__main__":
    sys.exit(main())

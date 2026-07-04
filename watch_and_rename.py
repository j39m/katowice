#!/usr/bin/python3

import sys
import argparse
import pathlib
import pyinotify
import time

from collections import deque

import logging

logging.basicConfig(
    stream=sys.stdout, level=logging.INFO, format="%(name)s: %(message)s"
)
logger = logging.getLogger("war")

CWD = "./"


def file_is_empty(path):
    if not path.is_file():
        logger.warning(f"{path} is not a file.")
        return True
    if not path.stat().st_size:
        logger.warning(f"Ruh-roh! “{path}” is empty.")
        return True
    return False


def block_until_size_stabilizes(path):
    if not path.is_file():
        logger.warning(f"{path} is not a file.")
        return
    size = None
    new_size = 0
    while size != new_size:
        time.sleep(0.52)
        size = new_size
        new_size = path.stat().st_size


class EventHandler(pyinotify.ProcessEvent):
    # The defaults are actually set by `parse_args()`, but are retained
    # here to illustrate good defaults.
    def __init__(self, initial_index=1, prefix="", abcd=False):
        self.index = initial_index
        self.prefix = prefix
        self.abcd = abcd
        # The deque is used to hold up to two elements: the current
        # element and (if it was empty) the previous element, whose
        # processing was deferred. This specifically addresses the
        # stutter-step observed in Firefox downloads, so we don't ever
        # expect two consecutive empty elements.
        self.queue = deque(maxlen=2)

    def target_filename(self, suffix):
        index = f"{self.index:03d}"
        if self.abcd:
            index = chr(self.index + 96)

        target = pathlib.Path(f"{self.prefix}{index}{suffix}")
        assert not target.is_file()

        self.index += 1
        if self.abcd:
            self.index %= 26

        return target

    def _process_queue(self) -> None:
        """Processes one or two elements."""
        element = self.queue.popleft()
        try:
            block_until_size_stabilizes(element)
        except FileNotFoundError:
            if len(self.queue):
                # Recurse, thereby dropping this element.
                return self._process_queue()
            return

        # There's no reason that I can think of that we would converge
        # on a stable file size of 0. This should be a tempfile that
        # ends up removed, raising `FileNotFoundError` above.
        if file_is_empty(element):
            logger.warning("BUG BUG BUG")

        target = self.target_filename(element.suffix)
        element.rename(target)
        logger.info(f"{target} <- {element}")

    def process_IN_CLOSE_WRITE(self, event):
        self.queue.append(pathlib.Path(event.name))
        self._process_queue()


def _deduce_initial_index():
    files = [f for f in pathlib.Path(CWD).iterdir() if f.is_file()]
    return len(files) + 1


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i", "--initial-index", default=_deduce_initial_index(), type=int
    )
    parser.add_argument("-p", "--prefix", default="")
    parser.add_argument("--abcd", default=False, action="store_true")
    args = parser.parse_args()

    return EventHandler(args.initial_index, args.prefix, args.abcd)


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

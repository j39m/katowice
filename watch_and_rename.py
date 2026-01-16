#!/usr/bin/python3

import sys
import argparse
import pathlib
import pyinotify

from collections import deque

import logging

logging.basicConfig(
    stream=sys.stdout, level=logging.INFO, format="%(name)s: %(message)s"
)
logger = logging.getLogger("war")

CWD = "./"


def file_is_empty(path):
    assert path.is_file(), f"{path} is not a file"
    if not path.stat().st_size:
        logger.warning(f"Ruh-roh! “{path}” is empty.")
        return True
    return False


class EventHandler(pyinotify.ProcessEvent):
    # The defaults are actually set by `parse_args()`, but are retained
    # here to illustrate good defaults.
    def __init__(self, initial_index=1, prefix=""):
        self.index = initial_index
        self.prefix = prefix
        # The deque is used to hold up to two elements: the current
        # element and (if it was empty) the previous element, whose
        # processing was deferred. This specifically addresses the
        # stutter-step observed in Firefox downloads, so we don't ever
        # expect two consecutive empty elements.
        self.queue = deque(maxlen=2)

    def target_filename(self, suffix):
        target = pathlib.Path(f"{self.prefix}{self.index:03d}{suffix}")
        assert not target.is_file()
        self.index += 1
        return target

    def _process_queue(self) -> None:
        """Processes one or two elements."""
        element = self.queue.popleft()
        if file_is_empty(element):
            # We just got this element. Return it to the queue and see
            # what happens when the next file comes.
            if not len(self.queue):
                self.queue.append(element)
                return
            # Recurse, thereby dropping this element.
            return self._process_queue()

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

#!/usr/bin/python3

import os.path
import sys

import glob
import subprocess

from pprint import pprint

def _exif_parse_key(raw_key: str) -> str:
    lowered = [fragment.lower() for fragment in raw_key.split()]
    return "_".join(lowered)

def _exif_parse_line(line: str) -> (str, str):
    (raw_key, raw_val) = line.split(":", maxsplit=1)
    return (_exif_parse_key(raw_key), raw_val.strip())

def _exif_parse(command_output: str) -> dict:
    parsed = dict()
    for line in command_output.splitlines():
        if not line.strip():
            continue
        (key, val) = _exif_parse_line(line)
        if not val:
            continue
        parsed[key] = val
    return parsed

class Exif:
    def __init__(self, path: str, exif: str):
        self.path = path
        self.raw_data = exif
        self.data = _exif_parse(exif)

def get_exif(path: str) -> None:
    assert os.path.isfile(path)
    exif = subprocess.Popen(
            args=("exiv2", path),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
    )
    (stdout, _) = exif.communicate()
    klaus = Exif(path, stdout.decode("utf-8"))
    pprint(klaus.data)

def main():
    target = sys.argv[1]
    for jpg in glob.glob(os.path.join(target, "*.jpg")):
        get_exif(jpg)

if __name__ == "__main__":
    sys.exit(main())

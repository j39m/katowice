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

def _run_exiv(path: str) -> str:
    assert os.path.isfile(path)
    exif = subprocess.Popen(
            args=("exiv2", path),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
    )
    (stdout, _) = exif.communicate()
    return stdout.decode()

class Exif:
    def __init__(self, path):
        self.path = path
        self.raw_data = _run_exiv(path)
        self.data = _exif_parse(self.raw_data)

def iter_exif_cb(exif: Exif) -> None:
    model = exif.data.get("camera_model")
    if not model or not model.startswith("Pixel 4"):
        print("{}: {}".format(exif.path, model))

def main():
    target = sys.argv[1]
    exifs = [Exif(path) for path in
                glob.glob(os.path.join(target, "*.jpg"))]
    for exif in exifs:
        iter_exif_cb(exif)

if __name__ == "__main__":
    sys.exit(main())

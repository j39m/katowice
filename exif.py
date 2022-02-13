#!/usr/bin/python3

import os.path
import sys

import datetime
import glob
import subprocess

from pprint import pprint
from zoneinfo import ZoneInfo

def datetime_from_basename(name: str) -> datetime.datetime:
    assert name.startswith("PXL_")
    return datetime.datetime.strptime(name[4:19], "%Y%m%d_%H%M%S")

def datetime_from_exif(exif_datetime: str) -> datetime.datetime:
    no_timezone = datetime.datetime.strptime(
            exif_datetime, "%Y:%m:%d %H:%M:%S")
    return no_timezone.replace(tzinfo=ZoneInfo("America/Los_Angeles"))

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

        self.basename_datetime = datetime_from_basename(
                os.path.basename(path))
        self.basename_utc_datetime = self.basename_datetime.astimezone(
                datetime.timezone.utc)

        try:
            self.exif_datetime = datetime_from_exif(
                    self.data["image_timestamp"])
            self.exif_utc_datetime = self.exif_datetime.astimezone(
                    datetime.timezone.utc)
        except KeyError:
            self.exif_datetime = None
            self.exif_utc_datetime = None

def iter_exif_cb(exif: Exif) -> None:
    if exif.exif_datetime:
        return
    print(exif.basename_utc_datetime)

def main():
    target = sys.argv[1]
    exifs = [Exif(path) for path in
                sorted(glob.glob(os.path.join(target, "*.jpg")))]
    for exif in exifs:
        iter_exif_cb(exif)

if __name__ == "__main__":
    sys.exit(main())

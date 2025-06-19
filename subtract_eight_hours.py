#!/usr/bin/python

import sys
from pathlib import Path
from datetime import datetime, timedelta

# Hong Kong
UTC_OFFSET = 9

DATE_FORMAT = "%Y%m%d"
TIME_FORMAT = "%H%M%S"


def validate_input_or_raise(src_path: str) -> Path:
    path = Path(src_path)
    if path.parts[0] != "third_party":
        raise ValueError(f"not under `third_party/`: {src_path}")
    if len(path.parts) != 2:
        raise ValueError(f"weird path: {src_path}")
    return path


def name_dst_symlink(src_path: str) -> Path:
    canonical_path = validate_input_or_raise(src_path)

    [pxl, date, nine_digit_time] = canonical_path.stem.split("_")
    three_last_digits = nine_digit_time[6:]
    new_datetime = datetime.strptime(
        f"{date}{nine_digit_time[:6]}", f"{DATE_FORMAT}{TIME_FORMAT}"
    ) - timedelta(hours=UTC_OFFSET)

    symlink_stem = new_datetime.strftime(
        f"{pxl}_{DATE_FORMAT}_{TIME_FORMAT}{three_last_digits}"
    )
    return Path(symlink_stem).with_suffix(canonical_path.suffix)


def main():
    symlinks_and_files = [(name_dst_symlink(f), f) for f in sys.argv[1:]]
    for s, f in symlinks_and_files:
        s.symlink_to(f)
        print(f"{s} points to {f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

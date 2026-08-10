#!/usr/bin/python3

"""
Sorts `bach-cantatas.csv` (currently by year of composition) and puts
the repr of a "dictionary for sorting" on stdout.
"""

import csv
import re


def parse_bwv(raw_bwv):
    """Handle only the funny zero-padding for BWVs < 100."""
    return re.sub(r"^0+\s", r"", raw_bwv)


def parse_year(raw_year):
    """
    The corner cases are interesting.
    1XXX-1YYY degrades naturally “1XXX” (assume the earlier year).
    1XXX? degrades naturally to “1XXX” (assume the earlier bound).

    Furthermore,
    ```
    "1700" < "1700?"
    "0" < "1700"
    ```
    I'm not sure I like the former case, but ah well.
    """
    return int(raw_year[:4])


with open("bach-cantatas.csv", "r") as csv_file:
    reader = csv.DictReader(csv_file)
    sortable_list = []
    for row in reader:
        # n.b. strings, not ints, to reflect things like “Anh. 999”
        # or “999a.” That's not actually important given my current
        # collection, but it would be needlessly restrictive to force
        # int keys.
        bwv = parse_bwv(row["BWV"])
        # Year of composition naturally forms a sort order!
        year = parse_year(row["Year"])
        sortable_list.append((bwv, year))
    sortable_list.sort(key=lambda pair: pair[1])

sorted_result = {bwv: year for (bwv, year) in sortable_list}
# 1 header row and 2 rows that span a line break.
assert len(sorted_result) == 292 - 1 - 2
print(repr(sorted_result))

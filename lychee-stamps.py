#!/usr/bin/python3
"""
Since datetime.time isn't actually capable of arithmetic, let's kludge
our own stuff.
"""

import sys

TSDELIM = ":"

def ts_to_s(tstr):
    """Given a timestamp in HH:MM:SS, return integral seconds."""
    teased = tstr.split(TSDELIM)
    (hrs, mins, secs) = (int(unit) for unit in teased)
    return (hrs * 3600) + (mins * 60) + secs

def read_ts_file(ts_fname):
    """
    Return a pair of lists (start_pts, durations), containing the start
    points and durations of each track (one per line).
    """
    start_pts = list()
    durations = list()
    with open(ts_fname, "r") as tsfp:
        for line in tsfp:
            (stp, dur) = line.strip().split()
            start_pts.append(ts_to_s(stp))
            durations.append(ts_to_s(dur))
    return (start_pts, durations)

def do_verify(start_pts, durations):
    """
    Given lists of start points and durations of each track, verify that
    every track is contiguous.
    """
    errfmt = "Wrong duration given at line {}!"
    prev_end = None
    for (cnt, (stpt, dur)) in enumerate(zip(start_pts, durations)):
        endpt = stpt + dur
        if prev_end is None:
            prev_end = endpt
            continue
        if prev_end != stpt:
            raise OSError(errfmt.format(cnt))
        prev_end = endpt

def main():
    """Verify timestamps for cutting the TLJ score."""
    ts_fname = sys.argv[1]
    (start_pts, durations) = read_ts_file(ts_fname)
    return do_verify(start_pts, durations)


if __name__ == "__main__":
    ret = main()
    sys.exit(ret)

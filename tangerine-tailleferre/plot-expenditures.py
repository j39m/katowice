#!/usr/bin/python3

import sys

import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import datetime

# Returns the lists of x and y values as a tuple (x, y).
def ingest_data():
    x = list()
    y = list()
    running_total = 0

    prior_value_error = None
    for line in sys.stdin:
        # Raise a previous ValueError iff it took place on a line not
        # the last.
        if prior_value_error is not None:
            raise prior_value_error
        try:
            (raw_date, _, raw_amount) = line.split(sep="|")
        except ValueError as e:
            prior_value_error = e
            continue

        point_date = datetime.date(
            *[int(token) for token in raw_date.split(sep="-")])
        running_total += float(raw_amount)
        # Overwrites the most current data point's y-value in place if
        # the date hasn't changed since the last loop pass.
        try:
            if x[-1] == point_date:
                y[-1] = running_total
            else:
                raise ValueError
        except (IndexError, ValueError):
            x.append(point_date)
            y.append(running_total)

    assert len(x) == len(y), "BUG: unbalanced x-y axes"
    return (x, y)

def main():
    (x, y) = ingest_data()

    plt.plot(x,y)
    plt.gcf().autofmt_xdate()
    plt.show()

    return 0

if __name__ == "__main__":
    sys.exit(main())

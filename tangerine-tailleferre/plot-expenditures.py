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
    for (count, line) in enumerate(sys.stdin):
        # Raise a previous ValueError iff it took place on a line not
        # the last.
        if prior_value_error is not None:
            raise prior_value_error
        try:
            (raw_date, _, raw_amount) = line.split(sep="|")
        except ValueError as e:
            prior_value_error = e
            continue

        x.append(datetime.date(*[int(token) for token in raw_date.split(sep="-")]))
        running_total += float(raw_amount)
        y.append(running_total)

    return (x, y)

def main():
    (x, y) = ingest_data()

    plt.plot(x,y)
    plt.gcf().autofmt_xdate()
    plt.show()

    return 0

if __name__ == "__main__":
    sys.exit(main())

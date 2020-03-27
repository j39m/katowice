#!/usr/bin/python3

import sys

import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import datetime

# Data-carrying struct.
class IngestedData:
    def __init__(self):
        # Plottable data.
        self.plottable_x = list()
        self.plottable_y = list()

        # Raw data. The plottable data is folded (so multiple
        # expenditures with the same date are telescoped into a single
        # point), but these are not. These are _at least_ as long as
        # self.x and self.y.
        self.raw_x = list()
        self.raw_y = list()

        # Stateful Exception carrier.
        self._prior_value_error = None

        # Stateful running total.
        self._running_total = 0

    def _update_plottable_data(self, point_date, amount):
        try:
            if self.plottable_x[-1] == point_date:
                self.plottable_y[-1] = self._running_total
            else:
                raise ValueError
        except (IndexError, ValueError):
            self.plottable_x.append(point_date)
            self.plottable_y.append(self._running_total)

    def _process_line(self, line):
        (raw_date, _, raw_amount) = line.split(sep="|")

        point_date = datetime.date(
            *[int(token) for token in raw_date.split(sep="-")])
        amount = float(raw_amount)

        self._running_total += amount
        self.raw_x.append(point_date)
        self.raw_y.append(amount)
        self._update_plottable_data(point_date, amount)

    def ingest_line(self, line):
        """
        Ingests a single line from the output of tangerine-tailleferre.
        """
        # Raises a previous ValueError iff it took place on a line not
        # the last.
        if self._prior_value_error is not None:
            raise _self.prior_value_error

        try:
            self._process_line(line)
        except ValueError as e:
            self._prior_value_error = e

def ingest_data():
    data = IngestedData()
    running_total = 0

    for line in sys.stdin:
        data.ingest_line(line)

    assert len(data.plottable_x) == len(data.plottable_y), \
            "BUG: unbalanced x- and y-axes"
    return data

def main():
    data = ingest_data()

    plt.plot(data.plottable_x, data.plottable_y)
    plt.gcf().autofmt_xdate()
    plt.show()

    return 0

if __name__ == "__main__":
    sys.exit(main())

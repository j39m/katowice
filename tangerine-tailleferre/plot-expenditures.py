#!/usr/bin/python3

import sys

import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import datetime

class PlottableData:
    def __init__(self):
        self.x = list()

        # Expenditures binned by day.
        self.daily_y = list()

        # Monotonically increasing running total of expenditures.
        self.cumulative_y = list()

    def _update_most_recent_date(self,
                                 date,
                                 expenditure_value,
                                 cumulative_total):
        if (date != self.x[-1]):
            raise IndexError
        self.daily_y[-1] += expenditure_value
        self.cumulative_y[-1] = cumulative_total

    def _add_new_date(self,
                      date,
                      expenditure_value,
                      cumulative_total):
        self.x.append(date)
        self.daily_y.append(expenditure_value)
        self.cumulative_y.append(cumulative_total)

    def add_data(self, date, expenditure_value, cumulative_total):
        try:
            self._update_most_recent_date(
                    date, expenditure_value, cumulative_total)
        except IndexError:
            self._add_new_date(date, expenditure_value, cumulative_total)

class IngestedData:
    def __init__(self):
        self.plottable = PlottableData()

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

    def _update_plottable_data(self):
        last_date = self.raw_x[-1]
        last_amount = self.raw_y[-1]
        self.plottable.add_data(last_date, last_amount, self._running_total)

    def _process_line(self, line):
        (raw_date, _, raw_amount) = line.split(sep="|")

        point_date = datetime.date(
            *[int(token) for token in raw_date.split(sep="-")])
        amount = float(raw_amount)

        self._running_total += amount
        self.raw_x.append(point_date)
        self.raw_y.append(amount)
        self._update_plottable_data()

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
    for line in sys.stdin:
        data.ingest_line(line)

    assert len(data.plottable.x) == len(data.plottable.cumulative_y), \
            "BUG: unbalanced x- and y-axes"
    return data

def main():
    data = ingest_data()

    _, ax = plt.subplots()

    # For any day `x`, the stack plot already visually raises the
    # cumulative curve by `x`'s daily value. But
    # `data.plottable.daily_y` already includes the value from day `x`,
    # resulting in a double-count for each day (causing visible downward
    # runs on certain spiky values) - i.e. every height on the
    # cumulative curve is actually
    # <-- cumulative val -->    |daily|
    # (y_0 + y_1 + ... + y_x   +  y_x).
    #
    # To handle this, we just need to trim off the contribution of day
    # `x` from the cumulative value - i.e. use `x-1`'s cumulative value,
    # i.e. shift the cumulative y array to the right by one.
    ax.stackplot(data.plottable.x,
                 (data.plottable.daily_y,
                  [0, *data.plottable.cumulative_y[:-1]]))
    plt.gcf().autofmt_xdate()
    plt.show()

    return 0

if __name__ == "__main__":
    sys.exit(main())

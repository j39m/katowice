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
        self.plottable.add_data(last_date, 0, self._running_total)

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

    plt.plot(data.plottable.x, data.plottable.cumulative_y)
    plt.gcf().autofmt_xdate()
    plt.show()

    return 0

if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/python3

"""
Let's plot the Chopin journey of Rafał Blechacz.
"""

import sys

import matplotlib
import matplotlib.pyplot as plt
import numpy


class Piece:
    def __init__(self, name: str, year: int, album: int):
        self.name = name
        self.year = year
        self.album = album


# Constant lists of pieces, organized by album.
# Where a range of years is specified, I typically take the earlier one.

preludes_album = (
    Piece("Préludes op. 28", 1835, 1),
    Piece("Prélude B. 86", 1834, 1),
    Piece("Prélude op. 45", 1841, 1),
    Piece("Nocturnes op. 62", 1845, 1),
    Piece("Mazurka op. 50 no. 1", 1841, 1),
)

concertos_album = (
    Piece("Concerto no. 2 op. 21", 1829, 2),
    Piece("Concerto no. 1 op. 11", 1830, 2),
    Piece("Mazurka op. 17 no. 4", 1831, 2),
)

polonaises_album = (
    Piece("Polonaises op. 26", 1835, 3),
    Piece("Polonaises op. 40", 1838, 3),
    Piece("Polonaise op. 44", 1841, 3),
    Piece("Polonaise op. 53", 1842, 3),
    Piece("Polonaise-Fantaisie op. 61", 1846, 3),
    Piece("Waltz op. 34 no. 2", 1838, 3),
)

chopin_album = (
    Piece("Sonata no. 2 op. 35", 1837, 4),
    Piece("Nocturne op. 48 no. 2", 1841, 4),
    Piece("Sonata no. 3 op. 58", 1844, 4),
    Piece("Barcarolle op. 60", 1845, 4),
)

# Organize these back-to-front so that the shortest stems (the first
# album recorded) are most foregrounded.
ALBUMS_AND_LINEFMTS = (
    (chopin_album, "C3--"),
    (polonaises_album, "C2--"),
    (concertos_album, "C1--"),
    (preludes_album, "C0--"),
)


def pieces_to_xy(pieces: list):
    x = [p.year for p in pieces]
    y = [p.album for p in pieces]
    return (x, y)


def piece_to_annotation(piece: Piece, y_offset: float):
    plt.annotate(piece.name, xy=(piece.year, piece.album),
                 xytext=(piece.year+0.1, piece.album+y_offset))


def pieces_to_annotations(pieces: list):
    y_offset = 0.0
    last_year = 0
    for piece in sorted(pieces, key=lambda p: p.year):
        # Annotation collision is unlikely with 2 intervening years.
        # Also unlikely if this label is blank (specific to op. 28's
        # train of blank labels).
        if (piece.year - last_year > 2) or (not piece.name):
            y_offset = 0.0
        piece_to_annotation(piece, y_offset)
        # Make a modest attempt to avoid colliding annotations.
        if piece.name:
            y_offset += 0.13
        last_year = piece.year


# Smear the op. 28 to 1839.
# IMSLP only mentions 1838-9, but Wikipedia sets it between 1835-9.
def smear_preludes():
    dotted_poles = [Piece("", year, 1)
                    for year in numpy.linspace(1835, 1839, num=8)]
    # Use dotted lines, not dashed ones.
    plt.stem(*pieces_to_xy(dotted_poles), linefmt="C0:")


def add_plot_data():
    for (album, linefmt) in ALBUMS_AND_LINEFMTS:
        plt.stem(*pieces_to_xy(album), linefmt=linefmt)
        pieces_to_annotations(album)
    smear_preludes()


def main():
    plt.style.use("fivethirtyeight")
    add_plot_data()

    plt.xlim(1828, 1849)
    plt.xlabel("Year of Composition")
    plt.ylabel("Album Index")
    # Restrict axes to showing integral values.
    plt.locator_params(axis="both", integer=True, tight=True)

    plt.show()


if __name__ == "__main__":
    sys.exit(main())

# katowice

... is a potpourri of projects I cobble together in my spare time.
They have no coherence, no point, no taste. What else could they be for
but to stroke my ego?

## avocado-attenborough.rs
... is a small executable that sets my screen brightness.

## firejail-locals
... contains all the local customizations I wish to retain for my
firejail usage.

## lychee-stamps.py
... verifies that a file full of timestamps and durations describes a
continuous timeline - i.e. that each duration spans the whole track into
the next timestamp, leaving no holes and causing no bleed-ins. It's used
to verify my work for cutting the score-only extract from TLJ.

## xkpw.py
... generates nonrandom passwords.

## undue\_instagram.py
... scrapes Instagram photos.

## ontrol.sh:
... is a little shell script I threw together to retag the set of HJ Lim's
Beethoven sonata traversal purchased from Google Play Music in which the
metadata is terrifyingly inconsistent.

## dovizu\_ando.py:
... is a game used to practice hiragana.

## nanisore.sh:
... is a shell script that quickly spits out a screenful of processes
ordered by top memory usage, CPU consumption, or process state.

## blueberry\_decibel.pl:
... is a tiny Perl script that tells me how my volumes are currently set.
Note to self: this is usually placed in `/usr/local/bin/howloud`.

## soup\_alphabet.py:
... is a Python script that proves that sorting strings by their
"reflection" is NOT equivalent to sorting strings in reverse order. It IS
equal when the strings are all of equal length, but the empty string is
always less than any other string, which means that the complements of two
non-equilength strings are NOT sorted the same as the strings sorted in
reverse.

## beans\_refried.py:
... is a Python script to reflow text files of unbroken lines for more
comfortable reading. Feed it a file (or "-" for stdin) and a maximum line
length (the default is 78) and it'll reflow the result to fit. Not for use
with the syntactically sensitive, e.g. LaTeX.

## guava\_gundam.sh
... is a bash script that performs the extremely hacky way of adjusting
my screen brightness, by parsing in the current value (via ACPI), doing
janky arithmetic with it, and writing it back.

## tomato\_enpassant.sh
... is a bash script to automate the repetitive stuff I usually have to
do to get a dual-screen setup working on Openbox. The usage is pretty
restrictive, but it works beautifully for my particular setup.

## unskipper.py
... is a Python script that prunes skipcounts from your Quod Libet
library. It's a selfish tic of mine that I don't like seeing nonzero
skipcounts on my songs because usually they happen by accident, and of
course increase proportionally with the number of times I listen to a
song (probably nonlinearly). It'll zero out all skipcounts in your
library by popping the associated key from the dictionaries in your
library and write the results back to disk.

## rhubarb\_lemonade.pl
... is a Perl script that counts word frequency in my quantum computing
term paper. It takes one mandatory and one optional argument; a file and
a discard threshold on frequency, respectively. So, for example, invoking
`./rhubarb_lemonade klaus 5` tells you how often each word in the file
`klaus` appears, sorted in descending numerical frequency. All words that
appear five or fewer times in `klaus` are left out of the accounting.

# katowice

... is a potpourri of projects I cobble together in my spare time.

> **NOTE**: This project is free software, a personal project by j39m.
> However, Google LLC owns the copyright on commits
> `e281f451bca055a7bcfc7d74814511b4d6763a19` and newer. This does not
> impact your ability to use and to hack at this free software; I
> provide this notice only for attribution purposes.

## satsuma-surveil.sh

... is a script that streams my default PulseAudio sink to my local
network, allowing me to treat my primary music player as a streaming
jukebox.

## tangerine-tailleferre

... is a small executable that controls my personal expenditures table.
Rust is not at all suited for this "scripting" work, but the original
GNU Make recipes have grown brambly and difficult to maintain.

## avocado-attenborough.rs

... is a small executable that sets my screen brightness.

## xkpw

... generates nonrandom passwords.

## unskipper.py

... is a Python script that prunes skipcounts from your Quod Libet
library. It's a selfish tic of mine that I don't like seeing nonzero
skipcounts on my songs because usually they happen by accident, and of
course increase proportionally with the number of times I listen to a
song (probably nonlinearly). It'll zero out all skipcounts in your
library by popping the associated key from the dictionaries in your
library and write the results back to disk.

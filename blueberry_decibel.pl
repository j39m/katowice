#!/usr/bin/perl

# a drop-in replacement for my "howloud" in Perl

use strict;
use warnings;

my $amixer_out = `amixer`;

$amixer_out =~ m/^\s+Front Left: Playback.*\[([0-9]+)%\].*$/m;
my $left_side = $1;
$amixer_out =~ m/^\s+Front Right: Playback.*\[([0-9]+)%\].*$/m;
my $right_side = $1;

if (!defined($left_side) || !defined($right_side)) {
    print "Oops! Abort.\n";
    exit 1;
}

print "$left_side | $right_side\n";

#!/usr/bin/perl

# show a screenful of processes by selected sort state
# suggestion: wrap this with ``watch'' to get a continuously running
# snapshot of processes (a bit like lazy top)

use strict;
use warnings;

$main::screenful = 39;      # how many lines fit on your screen?
$main::args_width = 52;     # about how wide is your screen?
$main::sort = "-rss";       # default sort is by memory

if ($0 =~ m/mamisore/i) {
    # cpu usage
    $main::sort = "-pcpu";
} elsif ($0 =~ m/hahisore/i) {
    # running state
    $main::sort = "state";
} else { # NANISORE
    # memory usage
    $main::sort = "-rss";
}

my $cmd = "ps -eo args:$main::args_width,euser,pid,pcpu,pmem,state "
          . "--sort=$main::sort";

$main::ps_out = `$cmd`;
my @ps_sp = split(/\n+/, $main::ps_out);

for my $line (@ps_sp) {
    print "  $line\n";
    $main::screenful--;
    if (!$main::screenful) {
        last;
    }
}

#! /usr/bin/env perl

# rhubarb_lemonade is a quick-and-dirty Perl script to tally up 
# word occurences. I'm using it to see how repetitive I am in my 
# quantum computing final paper. 
#
# You should invoke rhubarb_lemonade with two arguments: a file
# to process, and (optionally) the numerical cutoff beyond which 
# you count a word as trivial (the default is 1; so an exotic 
# word that appears only once in the document will not show up 
# in the final accounting). 

use strict; 
use warnings; 

my %wordtable; 

# thanks http://alvinalexander.com/perl/edu/qanda/plqa00016
sub descend { 
  $wordtable{$b} <=> $wordtable{$a}; 
} 

if (! defined $ARGV[0]) { 
  print "Gimme a file to work with!\n"; 
  exit 1; 
} 

open my $file, "<", $ARGV[0] 
  or die "Couldn't open file \"$ARGV[0]\"\n"; 

while (my $line = <$file>) { 
  my @wordtokens = split / |\.|,/, $line;
  if (defined $wordtokens[0]) { # if line has words
    for my $word (@wordtokens) { # for each word in the line

      if (($word =~ m/^[A-Za-z]+/) && (length($word) > 3)) { 
        if (defined $wordtable{$word}) { 
          $wordtable{$word} += 1; 
        } else { 
          $wordtable{$word} = 1; 
        } 

      } 
    } 
  } 
} 

close $file; 

if (defined $ARGV[1]) { 
  foreach my $key (sort descend (keys(%wordtable))) { 
    if ( $wordtable{$key} <= $ARGV[1] ) { 
      print "truncated. exiting... \n"; 
      exit 0; 
    } 
    printf "%03d", $wordtable{$key}; 
    print "x ... $key\n"; 
  } 
} else { 
  foreach my $key (sort descend (keys(%wordtable))) { 
    printf "%03d", $wordtable{$key}; 
    print "x ... $key\n"; 
  } 
} 

exit 0; 

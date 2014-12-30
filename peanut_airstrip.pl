#! /usr/bin/env perl

# extracts jpg URIs from ... stuff. Not gonna lie, it's very 
# single-use and not for anything noble.

use strict; 
use warnings; 

if (@ARGV != 1) { 
  print "usage: xtr.pl <input file>\n" and exit 1; 

} 

# else proceed to parse webpage. 

open my $oz, "<", $ARGV[0] 
  or die "couldn't open '$ARGV[0]!'\n"; 

my $counter = 10; 

while (my $line = <$oz>) { 
  
  my @stuff = split(" ", $line); # shot in the dark instead of 
  # multiple capture groups, eurgh

  foreach (@stuff) { 
    if ($_ =~ m/(http:\/\/[^ ]+\.jpg)/) { 
      # preserve read-only value
      my $tama = $1; 
      # "&amp;" = "&"
      $tama =~ s/&amp;/&/g; 
      # GET
      print "getting $tama\n"; 
      system("curl -s -o hahahohohehe_-_$counter.jpg '$tama'"); 
      $counter++; 
    } 
  } 

} 

exit 0; 

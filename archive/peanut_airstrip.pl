#! /usr/bin/env perl

# extracts jpg URIs from ... stuff. Not gonna lie, it's very 
# single-use and not for anything noble.

use strict; 
use warnings; 

if (@ARGV < 1) { 
  print "usage: xtr.pl <input file> <filter image type> <minimum width> <minimum height>\n" and exit 1; 

} 

# work with extensions
my $extension = ""; 
if (defined $ARGV[1]) { 
  $extension = $ARGV[1]; 
} else { 
  $extension = "jpg"
} 

# work with file sizes
my $size1 = 442; 
my $size2 = 442; 
if ((defined $ARGV[2]) and (defined $ARGV[3])) { 
  if (($ARGV[2] =~ m/[^0-9]+/) or ($ARGV[3] =~ m/[^0-9]+/)) { 
    printf STDERR "Incoherent size arguments $ARGV[2] and $ARGV[3];" .
      " defaulting to 442 by 442\n"; 
  } else { 
    $size1 = $ARGV[2]; 
    $size2 = $ARGV[3]; 
  } 
}

# else proceed to parse webpage. 

open my $oz, "<", $ARGV[0] 
  or die "couldn't open '$ARGV[0]!'\n"; 

my $name_prefix = "hahahohohehe_-_"; # change this for naming
my $counter = 10; 

# set params for files to be discarded (too small)
my $discard = "discard_-_"; 
my $discard_counter = 10; 
my $discard_fullname = ""; 

while (my $line = <$oz>) { 
  
  my @stuff = split(" ", $line); # shot in the dark instead of 
  # multiple capture groups, eurgh
  my $fullname = ""; # to be determined during run

  foreach (@stuff) { 
    if ($_ =~ m/(http:\/\/[^ ]+\.$extension)/) { 
      # preserve read-only value
      my $tama = $1; 
      # "&amp;" = "&"
      $tama =~ s/&amp;/&/g; 
      
      # GET
      print "getting $tama\n"; 
      $fullname = $name_prefix . $counter . "." . $extension; 
      system("curl -L -s -o $fullname '$tama'"); 
      
      # check if file is good? 
      my $identifier = `/usr/bin/identify $fullname`; 
      if ($identifier =~ m/ ([0-9]+)x([0-9]+) /) { 
        if (($1 < $size1) and ($2 < $size2)) { 
          $discard_fullname = $discard . $discard_counter . "." . $extension; 
          system("mv -fv $fullname $discard_fullname"); 
          $discard_counter++; 
        } else { 
          $counter++; 
        } 
      } else { 
        print STDERR "couldn't get size for $fullname!\n"; 
        print STDERR "identify outputs: $identifier\n"; 
      }
    } 
  } 

} 

exit 0; 

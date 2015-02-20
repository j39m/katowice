#! /usr/bin/env perl

# emuberry_elgar is a just-for-kicks "encoder" 
# implementing a totally breakable code that guards 
# only against the least competent of coders. It is 
# one part of a pair; the "decoder" is called 
# durian_elgar.pl. The spec for the code is formalized 
# in the README. 

use strict; 
use warnings; 

# the hasher variable determines which hash we encode 
# with. I set it to md5. You can use whatever all 
# parties have installed. 
my $hasher = "md5sum"; # better form: /usr/bin/md5sum

sub linefeed { 
  my $line = shift; 
  chomp($line); 
  # legality check of line in message. 
  while ($line =~ /([^\s]*[^a-z0-9 ]+[^\s]*)/g) { 
    unless ($1 =~ /^STOP$/) { 
      print STDERR "Illegal token(s) '$1' in line: ";
      print STDERR "'$line'\n";
      exit 1; 
    } 
  } 
  # tokenize line. 
  my @words = split(/\s+/, $line); 
  for my $word (@words) { 
    if ($word eq "STOP") { 
      print "STOP "
    } else { 
      my $len = length($word); 
      my $digest = `printf $word | $hasher`;
      $digest =~ /(\w+)/; 
      print "$len.$1 "; 
    } 
  } 
} 

if (!defined($ARGV[0])) { 
  print STDERR "usage: emuberry_elgar <message>\n";
  print STDERR "where <message> is either a file or a ";
  print STDERR "quoted string.\n"; 
} 

# the mode boolean determines if we are working with 
# a file or a string. 0 for file, 1 for string. 
my $mode = 0; 

open (my $fp, "<", $ARGV[0]) 
  or $mode = 1; 

if ($mode == 0) { 

  while (my $message= <$fp>) { 
    linefeed ($message); 
  } 
 
} elsif ($mode == 1) { 

  my $message = $ARGV[0]; 
  linefeed($message); 

} else { 

  print STDERR "Not designed for this!\n"; 
  exit 1; 

} 

print STDOUT "\n"; # final newline

exit 0; 

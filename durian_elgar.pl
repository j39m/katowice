#! /usr/bin/env perl

# durian_elgar is the counterpart to emuberry_elgar that
# decodes messages encoded per my ridiculously easy-to-
# break scheme. 

my $hasher = "md5sum"; 
my $hashlen = 32; 
my $ERRFAIL = -1; 
my @alphabet = ("a", "b", "c", "d", "e", "f", "g", "h", 
  "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
  "t", "u", "v", "w", "x", "y", "z"); 

# check takes a line and makes sure it is legal. (depr?)
sub check { 
} 

# linefeed tokenizes the line and decodes each word. 
# it calls the recursive helper function to this end. 
sub linefeed { 
  my @tokens = split(/\s+/, shift); 
  for my $token (@tokens) { 
    if ($token eq "STOP") { # stop word permissible
      print "$token "; 
    } else { # must be legal hashed token 
      unless ($token =~ /^[0-9]+\.[a-z0-9]{$hashlen}$/) { 
        print STDERR "illegal encoding: '$token'\n"; 
        exit 1; 
      } 
      my @lh = split(/\./, $token); 
      linehelper ($lh[0], "", $lh[1]); 
    } 
  } 
} 

# linehelper (linefeed's helper function) takes three
# arguments: (length, candidate_word, hash). It returns 
# the word (of the specified length) that has the 
# specified hash. 
sub linehelper { 

  my $length = shift; 
  my $word = shift; 
  my $hash = shift; 
  
  # bottom of recursion 
  if ($length == 0) { 
    my $truth = `printf "$word" | $hasher`; 
    $truth =~ /(\w+)/; 
    if ($1 eq $hash) { 
      print "$word "; 
      return 0; 
    } else { 
      return $ERRFAIL; 
    } 
  } 
  for my $letter (@alphabet) { 
    if (linehelper($length-1, $word . $letter, $hash)==0) {
      return 0; 
    } 
  } 
  return $ERRFAIL; # nothing found 
} 

if (!defined($ARGV[0])) { 
  print STDERR "usage: durian_elgar <message> \n" .
    "where <message> is either a file or a quoted " .
    "string containing a legally encoded message.\n"; 
  exit 1; 
} 

# 0: file; 1: string 
my $mode = 0; 

open (my $fp, "<", $ARGV[0]) 
  or $mode = 1; 

if ($mode == 0) { 

  while (my $line = <$fp>) { 
    chomp ($line); 
    check ($line); 
    linefeed($line); 
  } 

} elsif ($mode == 1) { 
  
  my $line = $ARGV[0]; 
  chomp ($line); 
  check($line); 
  linefeed($line); 

} else { 
  
  print STDERR "fault!\n"; 
  exit 1; 

} 

print "\n"; 

exit 0; 

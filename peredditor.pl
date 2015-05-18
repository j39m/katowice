#! /usr/bin/env perl
# peredditor is a Perl-based text browser for reddit. 
# format note: "title may-blank " href="..."...>TITLE HERE<...


use strict; 
use warnings; 
#use feature "switch"; 

# things used internally by peredditor, e.g. current state 
my $prompt = "[peredditor] > "; 
my @global_history = (); 

# things read in for control flow 
my $line = ""; 
my $command = ""; 
my $multiplier = 0; 
my $trailing  = ""; 

check_prereqs(); 

print "$prompt"; 

while($line = <STDIN>) { 
  chomp ($line); 
  ($command, $multiplier, $trailing) = parse_command ($line); 
  #print "$command; $multiplier; $aux\n"; 
  run_command ($command, $multiplier, $trailing); 
  print "$prompt"; 
} 

tap_out (0); 


##### peredditor's flow of execution ends here. #####


sub print_help { 
  my $hs = "A list of commands known to peredditor:\n"
    ."\th (print this help string)\n"
    ."\tg r/linux (go to the Linux subreddit)\n" 
    ."\tb (back)\n"
    ."\tf (forward)\n"
    ."\tfl [NUMBER] (follow link numbered"
      ." NUMBER - e.g. \"fl 0\")\n" 
    ."\tq (quit)\n"; 
  print "$hs"; 
  return; 
} 


# checks for required things. returns if all is good, terminates
# the whole script otherwise. 
sub check_prereqs {
  my $err = 0; 
  $err += system ("which curl > /dev/null"); 
  if ($err) { 
    print STDERR "One or more prereqs not satisfied. "; 
    tap_out (1); 
  } 
} 


# parses a line read in from STDIN and returns a tuple containing
# (command, multiplier, trailing). multiplier defaults to 1
# if parsing doesn't pick up otherwise. 
sub parse_command { 
  my ($entry) = @_; 
  if ($entry !~ /^[0-9]*[a-z]+/) { 
    print STDERR "Command \"$entry\" is malformed.\n"; 
    return ("", 0, ""); 
  } 
  $entry =~ /^([0-9]*)([a-z]+)[ ]*(.*)$/; 
  my $mul = $1; # the tricky compulsory numeric return value. 
  if ($mul eq "") { $mul = 1; } 
  return ($2, $mul, $3); 
} 


# takes in the result from parse_command() and acts appropriately.
# currently ignores the trailing part of the command. 
sub run_command { 
  my ($com, $mul, $tail) = @_; 

# run_command can be expressed as a given...when loop, but this 
# is still "experimental." 
#  given ( $com ) { 
#    when ("h") { 
#    } when ("g") { 
#    } when ("b") {
#    } when ("f") { 
#    } when ("fl") { 
#    } when ("q") { 
#      tap_out (0); 
#    } default { 
#      print STDERR "I don't understand \"$com\".\n"; 
#      continue; 
#    } 
#  } 
  if ($com eq "h") { 
  } elsif ($com eq "g") { 
  } elsif ($com eq "b") { 
  } elsif ($com eq "f") { 
  } elsif ($com eq "fl") { 
  } elsif ($com eq "q") { 
    tap_out (0); 
  } else { 
    print STDERR "I don't understand \"$com\".\n"; 
  } 

  return; 
} 


sub fetch_http { 
} 


# uniform exit interface. the single argument is optional, 
# defaulting to zero if not present. 
sub tap_out { 
  my ($exit_status) = @_; 
  if (!defined($exit_status)) { 
    $exit_status = 0; 
  } 
  print "BACK TO WORK WITH YOU.\n"; 
  exit $exit_status; 
} 

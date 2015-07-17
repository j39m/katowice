#! /usr/bin/env perl
# peredditor is a Perl-based text browser for reddit. 
# format note: "title may-blank " href="..."...>TITLE HERE<...


use strict; 
use warnings; 
#use feature "switch"; 

# things used internally by peredditor, e.g. current state. 
# implementation of history is that you leave a trail pushed
# into global_history_behind; when you use "back," it pops 
# _behind and pushes to _front. "forward" pops from _front and
# pushes to _behind. At all times your current state is defined
# by the last element in global_history_behind, which is loaded
# into global_current_page. 
my $prompt = "[peredditor] > "; 
my @global_history_behind = (); 
my @global_history_front = (); 
my $global_current_page = ""; 

# things read in for control flow 
my $line = ""; 
my $command = ""; 
my $multiplier = 0; 
my $trailing  = ""; 

check_prereqs(); 

print "$prompt"; 

my $err = 0; 

while($line = <STDIN>) { 
  chomp ($line); 
  ($command, $multiplier, $trailing) = parse_command ($line); 
  #print "$command; $multiplier; $aux\n"; 
  unless ($multiplier == 0 or $command eq "") { 
    $err = run_command ($command, $multiplier, $trailing); 
  } 
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
# returns 0 on success, nonzero otherwise. 
sub run_command { 
  my ($com, $mul, $tail) = @_; 

  # run_command can be expressed as a given...when loop, but this 
  # is still "experimental." 
  #given ( $com ) { 
    #when ("h") { 
    #} when ("g") { 
    #} when ("b") {
    #} when ("f") { 
    #} when ("fl") { 
    #} when ("q") { 
      #tap_out (0); 
    #} default { 
      #print STDERR "I don't understand \"$com\".\n"; 
      #continue; 
    #} 
  #} 
  if ($com eq "h") { 
    print_help (); 
  } elsif ($com eq "g") { 
    go_somewhere ($tail); 
  } elsif ($com eq "v") { 
    return 0; # no-op to view current page only. 
  } elsif ($com eq "b") { 
    return move_back (); 
  } elsif ($com eq "f") { 
    return move_forward (); 
  } elsif ($com eq "fl") { 
  } elsif ($com eq "q") { 
    tap_out (0); 
  } else { 
    print STDERR "I don't understand \"$com\".\n"; 
    return 1; 
  } 

  return 0; 
} 


# goes somewhere --- as of now, restricted to going to 
# subreddits. takes the subreddit in the form "r/somesubreddit" 
# and goes. updates program state to be viewing whatever 
# subreddit as arg. 
sub go_somewhere { 
  my ($sr) = @_; 
  my $page = fetch_subreddit ($sr); 
} 


# fetches a subreddit totally via curl and returns it as a 
# string. single argument is of the form "r/somesubreddit." 
sub fetch_subreddit { 
  my ($bare_sr) = @_; 
  my $full_url = "http://reddit.com/" . $bare_sr; 
  my $ret = `curl -L '$full_url' 2>/dev/null`; 
  return $ret; 
} 


# goes one page back in history. 
# returns 0 if success, nonzero if unable. 
sub move_back { 
  my $len = @global_history_behind; 
  if (!$len) { 
    return 1; 
  } 
  my $elem = pop (@global_history_behind); 
  push (@global_history_front, ($elem)); 
  return 0; 
} 


# goes one page forward in history. 
# returns 0 if success, nonzero if unable. 
sub move_forward { 
  my $len = @global_history_front; 
  if (!$len) { 
    return 1; 
  } 
  my $elem = pop (@global_history_front); 
  push (@global_history_behind, ($elem)); 
  return 0; 
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

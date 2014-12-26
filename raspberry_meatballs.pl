#! /usr/bin/env perl 

# raspberry_meatballs is a Perl script to recursively travel my 
# music directory and seek out directories that LACK a cover.jpg
# within. This is a sort of anti-find where it assumes every 
# directory contains a particular file and reports directories
# that do not. 
#
# You should use raspberry_meatballs with two arguments: the 
# directory to travel and the name to match. The directory
# must be absolute, but the name may be given as a regular
# expression. 
#
# This script is not meant to traverse all the way down a 
# directory tree. It peeks at all directories _within_ the given
# directory argument, searching for the file. Subdirectories are
# ignored. 

use strict; 
use warnings; 

if (@ARGV != 2) { 
  print "usage: raspberry_meatballs.pl <directory> <filename>\n"
    and exit 1; 
} 

# open specified dir
opendir my $wd_fp, $ARGV[0] 
  or print "Couldn't open directory '$ARGV[0]'!\n"
  and exit 1; 

# get all subdirectories (exactly one level down. easypeasy plz)
my @wd = readdir($wd_fp); 
my @directories_only; # populate me

for my $subdir (@wd) { 
  my $candidate = $ARGV[0]."/".$subdir; 
  if (-d $candidate and $subdir ne "." and $subdir ne "..") {
    push @directories_only, $subdir; 

  } 

} 

# read all the subdirectories.
for my $subdir (@directories_only) { 
  my $newpath = $ARGV[0]."/".$subdir; 
  opendir my $sd_fp, $newpath
    or print "Couldn't open subdirectory '$newpath'!\n"
    and exit 1; 
  my @sd_listing = readdir($sd_fp); 

  my $found_flag = 0; # set this if you find desired file.

  for my $file (@sd_listing) { 
    if (-f $newpath."/".$file and $file =~ m/$ARGV[1]/) { 
      $found_flag = 1; 
    } 
  } 
  unless ($found_flag) { 
    print "'$ARGV[1]' was not found in $subdir.\n"; 
  } 
} 
print "\n"; 

exit 0; 

#! /usr/bin/env perl 

# potato_wayfarer traverses a directory and all its
# subdirs, checking for duplicate files by mapping
# checksums to filenames 

use strict; 
use warnings; 

my $hash_function = "md5sum"; 
my $depth_control = 13; 
my %global_hash; 

my $wd = "/home/kalvin/Downloads/ksm/MMM/"; 
if (scalar(@ARGV)) {
  ($wd) = @ARGV;
}
chdir ($wd) 
  or die "Couldn't change directory: $wd\n"; 

traverse($wd, 0); 


sub traverse { # (directory, depth) 

  my ($dname, $depth) = @_;
  my @dirs; 

  if ($depth > $depth_control) { 
    return 1; 
  } 

  opendir (my $dh_, $dname)
    or die "Couldn't open directory: $dname\n"; 

  for my $file (readdir ($dh_)) { 
    if ($file eq "." or $file eq "..") { 
      next;  
    } else { 
      $file = join ("/", $dname, $file); 
    } 
    if (-d $file) { 
      push (@dirs, $file); 
    } elsif (-f $file) { 
      my $checksum = `$hash_function '$file' | cut -d " " -f 1`; 
      #my $checksum =  Digest->new("MD5");
      if (exists ($global_hash{$checksum})) { 
        print "$global_hash{$checksum} --> $file\n"; 
      } else { 
        $global_hash{$checksum} = $file; 
      } 
    } else { 
      print STDERR "Spurious file '$file'\n"; 
    } 
  } 

  for my $directory (@dirs) { 
    traverse ($directory, $depth+1); 
  } 

  return 0; 

} 



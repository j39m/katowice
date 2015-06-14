#! /usr/bin/env python3

# I'm tired of having "du -hs / | sort -h" choke on /proc, /home
# etc.


import os
import sys
import subprocess
import re


MY_NAME = "pickle_sickle"
INSPECTEE = "/" # an absolute path to inspect WITH trailing slash
EXCLUDE_US = ("proc", "home", "run", "lost+found")
EXCLUDED = tuple(INSPECTEE + ent for ent in EXCLUDE_US)
MEGABYTES = re.compile(r"^b'([0-9]+).*$")


def do_du(dir):
  """calls "du -ms" on dir if appropriate. returns the size
     in Mb, else None. dir is an absolute path!"""
  dnll = open("/dev/null", "w")
  if dir not in EXCLUDED:
    try:
      du = str(subprocess.check_output(("du", "-ms", dir), 
      stderr = dnll))
    except subprocess.CalledProcessError as err:
      annoy("du didn't work on " + dir + ": " + str(err.output))
      du = str(err.output)
    search = MEGABYTES.search(du)
    if not search:
      annoy("du didn't work on " + dir)
    else:
      return int(search.group(1))
  return None


def print_sizes(list_of_sizes):
  """prints a human-readable list of dirs and sizes. expects
     a list of tuples (dir, size_in_mb), and prints these out."""
  print_magic = 1
  for tup in list_of_sizes:
    if len(tup[0]) > print_magic:
      print_magic = len(tup[0])
  print_fmt = "%" + str(print_magic) + "s%26s"
  for tup in list_of_sizes:
    name = tup[0]
    size = tup[1]
    if size > 1024:
      size /= 1024
      size = str("%s GB" % size)
    else:
      size = str("%s MB" % size)
    printable = str(print_fmt % (name, size))
    print(printable)
  return


def annoy(msg):
  """generic shortened form for printing errors."""
  sys.stderr.write(MY_NAME + ": ")
  sys.stderr.write(msg)
  sys.stderr.write("\n")
  sys.stderr.flush()
  return


def do_work(pr=False):
  """main entry point. returns a list of tuples, each one being
     (dir, size_in_mb)."""
  retv = []
  for ent in os.listdir(INSPECTEE):
    dirent_str = INSPECTEE + ent
    dirent_siz = do_du(dirent_str)
    if dirent_siz:
      retv.append((dirent_str, do_du(dirent_str)))
  retv = sorted(retv, key=(lambda x: x[1]))
  if pr:
    print_sizes(retv)
  return retv


if __name__ == "__main__":
  do_work(pr=True)

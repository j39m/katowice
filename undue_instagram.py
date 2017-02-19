#!/usr/bin/python3

import sys
import os
import re
import subprocess


CURL_BIN =  "/usr/bin/curl"
TMP_FILE =  "klaus.undue.html"

#TRUE_RE =   re.compile(r'^.+content="(https://instagram\..+net/.+/([^/]+\.jpg)?.+\.2)".*$')
TRUE_RE =   re.compile(r'^.+content="(https://instagram\..+net/.+/([^/]+\.jpg)?(.+\.2)?)".*$')



def doCleanup():
    try:
        os.unlink(TMP_FILE)
    except OSError:
        print("Couldn't unlink %s!" % TMP_FILE)


class instagramPage(object):

    def __init__(self, url, *args):
        self.url =          url
        self.photoUrl =     None
        self.photoName =    None

    def webFetch(self, fetchee, tmpName=TMP_FILE):
        cmd = (CURL_BIN, "-s", "-o", tmpName, fetchee)
        output = subprocess.check_output(cmd)

    def fetchPage(self):
        self.webFetch(self.url)

    def parsePage(self):
        f = open(TMP_FILE, "r")
        slurp = f.readlines()
        f.close()

        match = None
        for line in slurp:
            match = TRUE_RE.match(line)
            if match:
                break
        if not match:
            print("Error parsing!")
            return 1
        else:
            (self.photoUrl, self.photoName) = (match.group(1), match.group(2))
        return 0

    def fetchPhoto(self):
        self.webFetch(self.photoUrl, self.photoName)
        print("%s" % self.photoName)

    def run(self):
        self.fetchPage()
        if self.parsePage():
            return 1

        self.fetchPhoto()
        doCleanup()
        return 0


def main(*args):
    retv = 0
    downQueue = list()
    stdinLine = None
    stdinQueue = list()

    # Case args: take them one by one.
    if args:
        for a in args:
            downQueue.append(instagramPage(a))
    # Case no args: read from stdin.
    else:
        while stdinLine != "":
            stdinLine = sys.stdin.readline().strip()
            stdinQueue.append(stdinLine)
        for line in stdinQueue:
            downQueue.append(instagramPage(line))

    for p in downQueue:
        retv += p.run()
    return retv

if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:]))

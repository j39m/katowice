#!/usr/bin/python3

import sys
import re
import curl


TRUE_RE = re.compile(
    r'^.+og:image.+(src|content)="(https://.*ins'
    r'tagram.+(com|net).+/([0-9a-z_]+_n.jpg(\.2)?))".+'
)


def simple_curl(targ):
    """Fetch something with curl."""
    mycurl = curl.Curl()
    gotten = mycurl.get(targ)
    mycurl.close()
    return gotten


def parse_page(content):
    """
    Parse an Instagram page and return the tuple of the contained photo
    URL and photo filename.
    """
    content_str = content.decode(encoding="UTF-8")
    content_l = content_str.splitlines()
    match = None
    for line in content_l:
        match = TRUE_RE.match(line)
        if match:
            break
    if not match:
        return (None, None)
    return (match.group(2), match.group(4))


def write_photo(content, fname):
    """
    Accept bytes of an Instagram photo and write the result to the param
    ``fname.''
    """
    with open(fname, "wb") as fpt:
        fpt.write(content)


class InstagramPage(object):
    """Represent an Instagram page."""
    def __init__(self, url):
        self.url = url

    def run(self):
        page = simple_curl(self.url)
        (purl, pname) = parse_page(page)

        if not purl:
            errmsg = "FATAL: Failed to parse ``{}!''\n"
            sys.stderr.write(errmsg.format(self.url))
            return 1
        photo_bytes = simple_curl(purl)
        write_photo(photo_bytes, pname)

        print(pname)
        return 0


def main(*args):
    """Main entry point."""
    retv = 0
    wkq = [InstagramPage(a) for a in args]

    for ent in wkq:
        retv += ent.run()
    return retv


if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:]))

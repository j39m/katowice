#!/usr/bin/python

import sys
import time
import subprocess
import pprint
import yaml


def seconds(hms):
    parsed = time.strptime(hms, "%H:%M:%S")
    return (parsed.tm_hour * 3600) + (parsed.tm_min * 60) + (parsed.tm_sec)


class TrackParams:

    def __init__(self, yaml_dict):
        self.start = seconds(yaml_dict["start"])
        self.end = seconds(yaml_dict["end"])

        self.fade_out = yaml_dict.get("fade_out", False)
        self.fade_in = yaml_dict.get("fade_in", False)

    def as_args(self, src, track_basename):
        result = [
            "/usr/bin/ffmpeg",
            "-i",
            src,
            "-ss",
            str(self.start),
            "-to",
            str(self.end),
        ]
        afs = []
        if self.fade_in:
            afs.append(f"afade=in:st={self.start}:d=2")
        if self.fade_out:
            afs.append(f"afade=out:st={self.end - 6}:d=5")
        if afs:
            result.append("-af")
            result.append(",".join(afs))
        result.append(track_basename)
        return result


class TrackSet:

    def __init__(self, src, config_file_name):
        self.src = src
        with open(config_file_name, "r") as yfp:
            config_yaml = yaml.safe_load(yfp)
            self.track_params_list = [
                TrackParams(entry) for entry in config_yaml
            ]

    def _track_params_generator(self):
        for (index, track_params) in enumerate(self.track_params_list):
            track_basename = f"{index+1:02d}.opus"
            yield track_params.as_args(self.src, track_basename)

    def do_cut(self, dewit=False):
        for params in self._track_params_generator():
            pprint.pprint(params)
            if not dewit:
                continue
            proc = subprocess.Popen(params)
            proc.wait()
            assert proc.returncode == 0


def main():
    try:
        dewit = sys.argv[3] == "DEWIT"
    except IndexError:
        dewit = False
    track_set = TrackSet(sys.argv[1], sys.argv[2])
    track_set.do_cut(dewit)
    return 0


if __name__ == "__main__":
    sys.exit(main())

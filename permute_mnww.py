#!/usr/bin/python3

from itertools import permutations

def permutation_is_acceptable(p):
    if p[0] == "ン":
        return False
    if "ワク" in p:
        return False
    if "ゲン" in p:
        return False
    return True

permutations = set(permutations([point for point in "ムゲンニワクワク"]))
joined = ["".join(p) for p in permutations]
filtered = [p for p in joined if permutation_is_acceptable(p)]
filtered.sort()
[print(p) for p in filtered]

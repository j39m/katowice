#!/bin/bash

set -e;

# We must be run from the top of the katowice repo.
# This doesn't actually check that condition.
[ -d .git ] || ( printf "Run me from the top of katowice.\n"; exit 1 )

for f in $(grep "^##" README.md \
            | sed -e 's/^## \([^:]\+\):*$/\1/' -e 's/\\//g'); do
    ls ./"$f" 2>/dev/null || ls ./archive/"$f";
done

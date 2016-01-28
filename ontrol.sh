#!/bin/bash

# SAFEGUARD:
exit 1;

total_track=1;
for f in *.mp3; do
    disc="$(grep "TPOS" "$f".tags | cut -d "=" -f 2)";
    track="$(grep "TRCK" "$f".tags | cut -d "=" -f 2 | sed -e 's/ $//')";
    title="$(sed -n "$total_track"p ./itles)";
    case "$disc" in
        "1/8")
            genre="Heroic Ideals";
            ;;
        "2/8")
            genre="Eternal Feminine - Youth";
            ;;
        "3/8")
            genre="Assertion of an Inflexible Personality";
            ;;
        "4/8")
            genre="Nature";
            ;;
        "5/8")
            genre="Extremes in Collision";
            ;;
        "6/8")
            genre="Resignation and Action";
            ;;
        "7/8")
            genre="Eternal Feminine - Maturity";
            ;;
        "8/8")
            genre="Destiny";
            ;;
        *)
            genre="Out-of-bounds";
            ;;
    esac;
    ((total_track++));

    # IT'S EXECUTION DAY
    mid3v2 --verbose --track="$track" --song="$title" --TPOS "$disc" --TSST "$genre" "$f";

done;

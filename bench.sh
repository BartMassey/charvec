#!/bin/sh
n=100000000
for charset in --unicode --ascii
do
    for alg in --baseline --charvec --string 
    do
        for m in `seq 1 16` 32 64
        do
            echo $charset / $alg / $m
            time target/release/charvec $charset $alg $n $m 2>&1 >/dev/null
        done
    done
done |
sed \
    -e '/^.inputs/d' \
    -e 's/^\([0-9][0-9.]*\).*/\1/' |
awk '/^--/ { opts = $0; next } { printf "%s: %s\n", opts, $0 }'

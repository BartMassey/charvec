#!/bin/sh
n=100000000
for alg in --baseline --charvec --string 
do
    for m in 1 2 3 4 5 6 7 8 16 64
    do
        echo $alg / $m
        time target/release/charvec $alg $n $m 2>&1 >/dev/null
    done
done |
sed -e '/^.inputs/d' -e 's/^\([0-9][0-9.]*\).*/\1/'

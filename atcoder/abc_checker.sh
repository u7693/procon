#!/bin/sh

for ((n=126 ; n<=166 ; n++))
do
    echo -n "ABC${n}: "
    for q in a b c d e f
    do
        if [ ! -f "abc${n}/src/bin/${q}.rs" ]; then
            echo -n " ${q}"
        fi
    done
    echo ""
done

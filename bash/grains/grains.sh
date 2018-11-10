#!/bin/bash

let agg=0

exp=$(($1-1))

if [[ "$1" = "total" ]]; then
    for i in range {1..63};
        do agg=$(bc <<< "$agg+(2^$i)");
    done;
    echo $agg
    exit
fi

if (($exp > 63)) || (($exp < 0)); then
    echo "Error: invalid input"
    exit 1
fi

echo $(bc <<< 2^$exp)

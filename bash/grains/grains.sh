#!/bin/bash

if [ "$1" = "total" ]; then 
    echo 18446744073709551615
    exit
fi

exp=$(($1-1))

if (($exp > 63)) || (($exp < 0)); then
    echo "Error: invalid input"
    exit 1
fi
if (($exp == 63)); then 
    echo 9223372036854775808
    exit
fi
echo $((2**$exp)) 

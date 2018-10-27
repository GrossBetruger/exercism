#!/bin/bash

exp=$(($1-1))
if (($exp == 63)); then 
    echo 9223372036854775808
    exit
fi
echo $((2**$exp)) 

#!/bin/bash

input=$1
log="x = l($input)/l(10); scale = 0; (x / 1) + 1"

num_of_digits=$(echo $log | bc -l)

let acc=0
while read -n1 digit; do
    acc=$((acc+$digit**$num_of_digits))
done < <(echo -n "$input")

if [[ $acc == $input ]];
then echo true && exit 0
fi

echo false


#!/bin/bash

log="x = l($1)/l(10); scale = 0; (x / 1) + 1"

num_of_digits=$(echo $log | bc -l)
echo $num_of_digits

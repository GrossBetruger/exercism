#!/bin/bash

str="$1"
res=""
for i in $(seq 1 ${#str});
  do res+=${str:${#str}-$i:1};
done
echo $res

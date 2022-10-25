#!/bin/bash

total_sum=0

function roll() {
    rolls=$1
    sides=$2

    echo "rolling ${sides}-sided die ${rolls}:"

    sum=0
    for i in $(seq 1 $rolls); do
        roll=$((1 + $RANDOM % $sides))
        sum=$(($sum + $roll))
        echo "  roll #${i} = ${roll}"
    done

    total_sum=$(($total_sum + $sum))

    echo -e "Rolled ${sides}-sided die ${rolls}x, which adds up to ${sum}.\n"
}

arg_count=0
for arg; do
    arg_count=$(($arg_count + 1))
    sides="${arg##*-}"
    if [[ ${#arg} -eq 1 ]]; then
        rolls=1
    else
        rolls="${arg%-*}"
    fi
    roll $rolls $sides
done

if [[ arg_count -gt 1 ]]; then
    echo "In total, all of those rolls add up to ${total_sum}."
fi

#!/bin/bash

function usage() {
	echo "Usage: ./thumbs.sh (-e|-p|-s) [-d] dimension image"
	exit 1
}

MODE=(EXACT PROPORTIONAL SQUARE)
for ((i = 0; i < ${#MODE[@]}; i++)); do
	name=${MODE[i]}
	declare -r ${name}=$i
done

delete_meta_data=False

while getopts "epsd" opt; do
	case $opt in
	e) [ -n "$mode" ] && usage || mode=$EXACT ;;
	p) [ -n "$mode" ] && usage || mode=$PROPORTIONAL ;;
	s) [ -n "$mode" ] && usage || mode=$SQUARE ;;
	d) delete_meta_data=True ;;
	esac
done
shift $(($OPTIND - 1))

if [[ -z $mode ]]; then
	echo "Mode not defined."
	usage
fi

dimension=$1
image=$2

echo "${MODE[$mode]}, $dimension, $image"

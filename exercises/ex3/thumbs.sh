#!/bin/bash

function usage() {
	echo 'Usage: ./thumbs.sh (-e|-p|-s) [-d] dimension image'
	exit 1
}

function err() {
	echo -e "\033[31m$1\033[0m" >&2
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
	err 'Mode not defined.'
	usage
	exit 2
fi

if [[ $1 -le 0 ]]; then
	err 'Invalid dimension'
	exit 3
fi

dimension=$1
image=$2
filename="${image%.*}"
extension="${image##*.}"
output_file="${filename}_${MODE[$mode]}_${dimension}.${extension}"

if [[ $delete_meta_data == True ]]; then
	strip="-strip"
else
	strip=
fi

function square_crop() {
	IFS=' ' read -r -a metadata <<<"$(magick identify "$image")"

	img_size=${metadata[2]}
	width=${img_size%x*}
	height=${img_size##*x}

	if [[ $width -gt $height ]]; then
		args="${image} -resize x${dimension} ${strip} -extent ${dimension} ${output_file}"
	else
		args="${image} -resize ${dimension}x ${strip} -extent ${dimension} ${output_file}"
	fi

	convert $args
}

function proportional_crop() {
	arg="${image} -resize ${dimension} ${strip} ${output_file}"
	convert $arg
}

function exact_crop() {
	arg="${image} -resize ${dimension}x${dimension}! ${strip} ${output_file}"
	convert $arg
}

case $mode in
$EXACT) exact_crop ;;
$PROPORTIONAL) proportional_crop ;;
$SQUARE) square_crop ;;
esac

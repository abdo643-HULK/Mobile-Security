#!/bin/bash

function hash() {
	hash=$(sha512sum "${1}" | cut -d " " -f 1)
	echo "$hash"
}

function check_exists() {
	if [[ ! -f "${1}" ]]; then
		echo "$1 doesn't exist." >&2
		exit 1
	fi
}

function check_doesnot_exist() {
	if [[ -f "${1}" ]]; then
		echo "$1 already exists." >&2
		exit 2
	fi
}

function check_read_permission() {
	if [[ ! -r "${1}" ]]; then
		echo "$1: No read permission" >&2
		exit 3
	fi
}

function check_write_permission() {
	check_read_permission $1

	if [[ ! -w "${1}" ]]; then
		echo "$1: No write permission" >&2
		exit 4
	fi
}

function build_map() {
	source_dir=$1
	map_file=$2

	check_read_permission $source_dir
	check_doesnot_exist $map_file

	files=$(find "${source_dir}" -type f)

	map=()
	for file_path in $files; do
		hash=$(hash "${file_path}")
		filename=$(basename "${file_path}")
		absolute_path=$(realpath "${file_path}")
		map+=("${filename},${absolute_path},${hash}")
	done

	printf "%s\n" "${map[@]}" >"${map_file}"
}

function check_map() {
	map_file=$1

	check_exists $map_file
	check_read_permission $map_file

	while IFS= read -r line; do
		IFS=',' read -r -a splitted <<<"${line}"
		filename=${splitted[0]}
		file_path=${splitted[1]}
		hash=${splitted[2]}

		checked_hash=$(hash "${file_path}")
		if [[ $hash != "${checked_hash}" ]]; then
			echo -e "\033[33mWarning:\033[0m ${filename} has been modified."
		fi
	done <"${map_file}"
}

function rebuild_map() {
	map_file=$1

	check_exists $map_file
	check_read_permission $map_file
	check_write_permission $map_file

	map=()
	while IFS= read -r line; do
		IFS=',' read -r -a splitted <<<"${line}"
		filename=${splitted[0]}
		file_path=${splitted[1]}
		hash=$(hash "${file_path}")
		map+=("${filename},${file_path},${hash}")
	done <"${map_file}"

	printf "%s\n" "${map[@]}" >"${map_file}"
}

while getopts "bcr" opt; do
	shift $(($OPTIND - 1))
	case $opt in
	b) build_map $1 $2 ;;
	c) check_map $1 ;;
	r) rebuild_map $1 ;;
	esac
done

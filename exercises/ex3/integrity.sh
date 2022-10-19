#!/bin/bash
function hash() {
	hash=$(sha512sum "${1}" | cut -d " " -f 1)
	echo "$hash"
}

function check_read_permission() {
	if [[ ! -r "${1}" ]]; then
		echo "$1: No read permission"
		return 3
	fi
}

function check_file_permissions() {
	check_read_permission $1

	if [[ ! -w "${1}" ]]; then
		echo "$1: No write permission"
		return 5
	fi
}

function check_exists() {

}

function build_map() {
	source_dir=$1
	map_file=$2

	if [[ -f "${map_file}" ]]; then
		echo "$map_file already exists."
		return 1
	fi

	check_dir_permissions source_dir

	files=$(find "${source_dir}" -type f)

	map=()
	for file_path in $files; do
		hash=$(hash "${file_path}")
		filename=$(basename "${file_path}")
		map+=("${filename},${file_path},${hash}")
	done

	printf "%s\n" "${map[@]}" >"${map_file}"
}

function check_map() {
	map_file=$1

	if [[ ! -f "${map_file}" ]]; then
		echo "$map_file doesn't exists."
		return 2
	fi

	check_file_permissions map_file

}

function rebuild_map() {
	map_file=$1

	if [[ ! -f "${map_file}" ]]; then
		echo "$map_file doesn't exists."
		return 2
	fi

	check_file_permissions map_file

	map=()
	while IFS= read -r line; do
		IFS=',' read -r -a splitted <<<"${line}"
		filename=$splitted[0]
		file_path=$splitted[1]
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

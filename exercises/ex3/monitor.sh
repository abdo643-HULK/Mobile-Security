#!/bin/bash

while getopts "bcr" opt; do
	shift $(($OPTIND - 1))
	case $opt in
	b) build_map $1 $2 ;;
	c) check_map $1 ;;
	r) rebuild_map $1 ;;
	esac
done

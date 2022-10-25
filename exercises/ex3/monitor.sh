#!/bin/bash

respawn=False
quiet=False
while getopts "qr" opt; do
	case $opt in
	q) quiet=True ;;
	r) respawn=True ;;
	esac
done
shift $(($OPTIND - 1))

pid=$1
interval=$2

if ! [[ $pid =~ '^[\d]+$' ]]; then
	pname=$1
	pids=$(pgrep "$pname")
	IFS=$'\n' read -rd '' -a pids <<<"$pids"
	if [[ ${#pids[@]} -gt 1 ]]; then
		printf -v joined '%s, ' "${pids[@]}"
		echo "Please enter one of the following pids: ${joined%, }"
		read pid

		if [[ ! " ${pids[*]} " =~ " ${pid} " ]]; then
			echo "Inalid Input" >&2
			exit 1
		fi
	fi
fi

process_info=$(ps -p "$pid" -o 'args=')

while true; do
	if ! ps -p $pid >/dev/null; then
		echo "$pid stopped" >&2

		if [[ $respawn == False ]]; then
			echo "Should the process be respawned: (y|n)"
			read should_respawn

			if [[ " $should_respawn " == " n " ]]; then
				break
			fi
		fi

		($process_info &)
		echo "$process_info respawned"
	else
		if [[ "$quiet" == False ]]; then
			echo "$pid still running"
		fi
	fi

	sleep $interval
done

#!/bin/bash

set -eu

die() { echo >&2 "$*"; exit 1; }

# HACK
export LD_LIBRARY_PATH=/tmp/dbusbench
export PATH=/tmp/dbusbench:$PATH

impls=(sdbus sdbusplus dbusrs zbus)
ops=(get set call)

opcount="${1:-500}"

trap 'kill $srvpid' EXIT

for s in "${impls[@]}"; do
	dbusbench-${s}-server &
	srvpid=$!
	sleep 1
	kill -0 $srvpid || die "$s server failed to start?"

	for op in "${ops[@]}"; do
		for c in "${impls[@]}"; do
			t="$(dbusbench-${c}-client "$op" "$opcount")"
			echo "$s $c $op $opcount $t"
		done
	done

	kill $srvpid
	wait $srvpid || :
done

trap - EXIT

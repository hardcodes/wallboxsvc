#!/bin/sh
#
# query stored relay state and print it

CURRENTDIR=$(dirname "$0")
STATEFILE="${CURRENTDIR}/../relay/state"

cat "${STATEFILE}"

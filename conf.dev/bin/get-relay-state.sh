#!/bin/sh
#
# dummy command for development

CURRENTDIR=$(dirname "$0")
STATEFILE="${CURRENTDIR}/../relay/state"

cat "${STATEFILE}"

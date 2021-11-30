#!/bin/sh
#
# dummy command for development

CURRENTDIR=$(dirname "${0}")
STATEFILE="${CURRENTDIR}/../relay/state"

set_state() {
    echo "{\"state\":"\"${1}\""}" > "${STATEFILE}"
    sleep 2
    "${CURRENTDIR}/get-relay-state.sh"
}

case "${1}" in
    "on")
    set_state "on"
    ;;
    "off")
    set_state "off"
    ;;
    *)
    set_state "unknown"
    ;;
esac
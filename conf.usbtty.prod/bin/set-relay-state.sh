#!/bin/bash
#
# set state of the usb relay that presents itself as tty
# this relay cannot be queried for its state hence it
# is written to a file
#
# Therefore the real relay state and stored can differ, e.g. after
# a power outage or device reconnect.

CURRENTDIR=$(dirname "${0}")
STATEFILE="${CURRENTDIR}/../relay/state"
USBTTY="/dev/ttyUSB0"

store_state() {
    echo "{\"state\":"\"${1}\""}" > "${STATEFILE}"
    "${CURRENTDIR}/get-relay-state.sh"
}

case "${1}" in
    "on")
    echo -e -n '\xA0\x01\x01\xA2' > ${USBTTY} 2> /dev/null && store_state "on"
    ;;
    "off")
    echo -e -n '\xA0\x01\x00\xA1' > ${USBTTY} 2>/dev/null && store_state "off"
    ;;
    *)
    store_state "unknown"
    ;;
esac
#!/bin/sh
#
# set state of the usb hid relay and query
# it afterwards
#
# after installing the package "usbrelay", the id was
# queryied by simply calling `usbrelay` without arguments

USB_RELAY_ID="959BI_1"
CURRENTDIR=$(dirname "${0}")

set_state() {
    /bin/usbrelay ${USB_RELAY_ID}=${1} 2>/dev/null
    sleep 1
    "${CURRENTDIR}/get-relay-state.sh"
}

case "${1}" in
    "on")
    set_state "1"
    ;;
    "off")
    set_state "0"
    ;;
    *)
    set_state "0"
    ;;
esac
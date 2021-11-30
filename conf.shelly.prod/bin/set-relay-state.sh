#!/bin/bash
#
# set state of the shelly relay that has to be connected
# via its wifi interface

CURRENTDIR=$(dirname "${0}")
SHELLY_IP="10.10.99.42"

set_state() {
    # for shelly API see
    # https://shelly-api-docs.shelly.cloud/gen1/#shelly1-1pm-relay-0
    SHELLY_STATE=$(/usr/bin/curl --connect-timeout 5 --no-buffer --silent --request GET "http://${SHELLY_IP}/relay/0?turn=${1}"|/usr/bin/jq '.ison')

    case ${SHELLY_STATE} in
        "true")
            echo '{"state":"on"}'
            ;;
        "false")
            echo '{"state":"off"}'
            ;;
        *)
            echo '{"state":"unknown"}'
            ;;
    esac

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

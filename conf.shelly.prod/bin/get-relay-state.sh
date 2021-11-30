#!/bin/sh
#
# query state of the shelly relay that has to be connected
# via its wifi interface

CURRENTDIR=$(dirname "$0")
SHELLY_IP="10.10.99.42"

# for API see
# https://shelly-api-docs.shelly.cloud/gen1/#shelly1-1pm-relay-0
SHELLY_STATE=$(/usr/bin/curl --connect-timeout 5 --no-buffer --silent --request GET "http://${SHELLY_IP}/relay/0"|/usr/bin/jq '.ison')


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


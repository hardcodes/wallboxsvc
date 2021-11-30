#!/bin/sh
#
# query state of the usb hid relay
#
# after installing the package "usbrelay", the id was
# queryied by simply calling `usbrelay` without arguments

USB_RELAY_ID="959BI_1"
RELAY_STATE=$(/bin/usbrelay 2>/dev/null|grep ${USB_RELAY_ID}|/usr/bin/cut -d "=" -f 2)

if [ "${RELAY_STATE}" = "0" ];then
    echo '{"state":"off"}'
else
    echo '{"state":"on"}'
fi

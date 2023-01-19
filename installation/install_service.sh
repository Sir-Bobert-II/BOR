#!/bin/sh

if [ "$(id -u)" != "0" ];then
    echo "This script is meant to be run as root!"
    exit 1
fi

if [-f ./leb.service]; then
    install -Dvm754 ./leb.service /etc/systemd/system/leb.service
else
    echo "Couldn't find ./leb.service"
    exit 1
fi
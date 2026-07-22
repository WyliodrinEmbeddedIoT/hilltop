#!/bin/bash

# Source: https://stackoverflow.com/a/72839021

restore(){
    mv $1.original $1
    echo -e "\nSocket $1 \e[33mRESTORED"
}

sock="$1"
trap "restore $sock" EXIT
mv "$sock" "$sock.original"

socat -t100 -v  UNIX-LISTEN:$sock,mode=777,reuseaddr,fork  UNIX-CONNECT:$sock.original

#!/bin/bash

die () {
    echo "$1"
    exit 1
}

check_config () {
    if [[ ! -d "/src" ]]; then
        die "/src not mounted"

    elif [[ -z "$( ls -A /src )" ]]; then
        die "/src is empty"

    elif [[ ! -d "/dst" ]]; then
        die "/dst not mounted"

    elif [[ -z "${QUALITY}" ]]; then
        die "QUALITY env not set"

    elif [[ -z "${VIDEOS}" ]]; then
        die "VIDEOS env not set"

    elif [[ -z "${TIME}" ]]; then
        die "TIME env not set"
    fi
}

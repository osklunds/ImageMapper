#!/bin/bash

source ./common.sh

check_config

if [[ "$VIDEOS" = "true" ]]; then
    videos="--include-videos"
fi

while :
do
    # sh -c 'exit 1' # for testing
    image_mapper "/src" "/dst" "$QUALITY" $videos --verbose
    last_status="$?"

    if [[ ! "$last_status" = "0" ]]; then
        die "image_mapper command crashed"
    fi

    echo "Sleeping $TIME seconds before converting again"
    sleep $TIME
done


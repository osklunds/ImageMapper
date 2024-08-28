#!/bin/bash

branch=$(git rev-parse --abbrev-ref HEAD)
commit=$(git rev-parse HEAD)
datestamp=$(date +"%Y-%m-%d")

file_name="image-mapper--$branch--$datestamp--$commit"
tag="osklunds/$file_name"

echo "Tagging with tag '$tag'"

docker tag osklunds/image-mapper-dev "$tag" || exit 1

out_path="saved_images/$file_name.tar"
echo "Saving to '$out_path'"

mkdir -p saved_images
docker image save "$tag" --output "$out_path"

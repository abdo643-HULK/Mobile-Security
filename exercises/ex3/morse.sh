#!/bin/bash

base='https://commons.wikimedia.org'
image_tags=$(curl "$base/wiki/Category:Straight_keys" | grep -oE '<a[^>]*href="[^"]*" class="galleryfilename.*"')
image_links=$(echo "$image_tags" | sed -r 's/.*href="([^"]*).*/\1/g')

while read image_link; do 
    link=$(curl "$base$image_link" | grep -oE '<a[^>]*class="internal"' | sed -r 's/.*href="([^"]*).*/\1/g')
    if [[ "$link" == https* ]]; then
        echo "$link"
        curl --output-dir "images" --create-dirs --remote-name -s "$link" 
    fi 
done <<< "$image_links"
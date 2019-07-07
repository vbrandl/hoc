#!/usr/bin/env sh

set -e

ACTIVE=${1}

if [ -z "${ACTIVE}" ]
then
    echo "Usage: $0 <list of active repos>"
    exit 1
fi

while IFS= read -r url
do
    imgs=$(curl "${url}" --silent | grep hitsofcode | grep -o -P 'https://camo.githubusercontent.com/[a-z0-9]+/[a-z0-9]+')
    [ -z "${imgs}" ] || echo "${url}"
    for img in ${imgs}
    do
        curl "$img" --silent > /dev/null &
    done
done < "${ACTIVE}"

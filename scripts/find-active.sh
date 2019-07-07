#!/usr/bin/env sh

set -e

DIR=${1:-repos}

for url in $(./scripts/list.sh "${DIR}")
do
    (curl "${url}" --silent | grep -q hitsofcode) && echo "${url}"
done

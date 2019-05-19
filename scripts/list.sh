#!/usr/bin/env sh

set -e

DIR=${1:-repos}

find "$DIR" -mindepth 3 -maxdepth 3 -type d \
    | sed -e "s/$DIR/https:\//g" \
    | sort

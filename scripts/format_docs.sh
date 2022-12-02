#!/usr/bin/env bash

# Format Markdown documents. Uses pandoc for intelligent formatting.

set -eu -o pipefail

for MARKDOWN in $(find . -iname "*md")
do
    pandoc -i $MARKDOWN -o $MARKDOWN --columns=80
done

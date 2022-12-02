#!/usr/bin/env bash

# Check and correct spelling in Markdown files, then update the list of known words.
# Depends on hunspell.

set -eu -o pipefail

MARKDOWNS=$(find . -iname "*.md")
hunspell -d en_US -p ./scripts/words $MARKDOWNS
hunspell -l $MARKDOWNS | cut -d ' ' -f 2 | sort | uniq > ./scripts/words

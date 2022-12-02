#!/bin/bash
set -euo pipefail
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 1 ]]; then
  echo "Please provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [[ ! "$1" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo "Argument '$1' is not a valid day."
  exit 1
fi

if [[ -z "${AOC_SESSION-""}" ]]; then
  echo "No session token set in \$AOC_SESSION."
  exit 1
fi

mkdir -p "$SCRIPT_DIR/data"

curl -s "https://adventofcode.com/2022/day/$(($1 + 0))/input" \
    --cookie "session=$AOC_SESSION" \
    -H 'User-Agent: Bash script to fetch data from advent of code'  \
    | tee "$SCRIPT_DIR/data/input$1.prod"

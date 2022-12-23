#!/usr/bin/env bash

set -e

SESSION_FILE='.session'
YEAR='2022'

DAY=$1
if [[ -z $DAY ]]; then
    echo "specify a day (e.g. 3)"
    exit 1
fi

DAY_DIR=$(printf "%02d" $DAY)
mkdir ./$DAY_DIR
echo "Writing python solution template..."
cat <<EOF >$DAY_DIR/solution.py
import sys

def main():
    lines = open(sys.argv[1]).readlines()
    # TODO

if __name__ == '__main__':
    main()
EOF

if [[ -f $SESSION_FILE ]]; then
    echo "Downloading input..."
    curl "https://adventofcode.com/$YEAR/day/$DAY/input" \
      -H "cookie: session=$(cat $SESSION_FILE)" \
      -s \
      >./$DAY_DIR/input.txt
fi

echo "Done."

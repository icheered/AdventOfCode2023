#!/bin/bash
YEAR=2023

# Check if .aoc_session file exists
if [ ! -f .aoc_session ]; then
  echo "Please create a .aoc_session file in the root with your session cookie from adventofcode.com"
  exit 1
fi

AOC_SESSION=$(cat .aoc_session)

max_num=0
for dir in day*/ ; do
  num=$(echo "$dir" | grep -o -E '[0-9]+')
  if (( num > max_num )); then
    max_num=$num
  fi
done

NEW_DAY=$((max_num + 1))

echo "Copying template for day $NEW_DAY"

NEW_DAY="$NEW_DAY"
cp -r template day$NEW_DAY

echo "Updating template"

# Change the project name in template folder Cargo.toml
sed -i 's/^name = "TEMPLATE"$/name = "'day$NEW_DAY'"/' day$NEW_DAY/Cargo.toml

# Change into the src directory
cd day$NEW_DAY/src

# Download the input file for the day
echo "Downloading input file"
curl -s -o input.txt https://adventofcode.com/$YEAR/day/${NEW_DAY}/input --cookie "session=$AOC_SESSION"
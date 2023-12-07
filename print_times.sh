#!/bin/bash

# Loop through directories named 'day1', 'day2', etc.
for dir in day* ; do
    # Check if the directory exists
    if [ -d "$dir" ]; then
        # Change into the directory
        cd "$dir/src"

        # Print the current day and then run the command, printing the last line of the output
        echo -n "$dir: "
        cargo run --quiet part2 input | tail -n 1

        # Go back to the original directory
        cd - > /dev/null
    fi
done

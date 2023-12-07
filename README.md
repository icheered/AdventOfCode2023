# AdventOfCode2023
## How to use template

To use the template you need to to have a file called .aoc_session which has the Cookie Token, which allows downloading
the days input:
```
export AOC_SESSION=<Your Cookie Token here>
```

The you use the following command to create a folder with template for the next day (incremented from the highest dayX folder in the directory) and automatically fetch the input.
editing:
```bash
./newday.sh
```

## Running the solutions
From the Advent Of Code website, 

Navigate to the `dayX/src` folder, copy the example input from the advent of code website to `test.txt`,  and run one of the following commands:
```bash
cargo run --quiet part1 test
cargo run --quiet part1 input
cargo run --quiet part2 test
cargo run --quiet part2 input
```

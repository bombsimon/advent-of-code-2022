# Advent of Code 2022

Let's go, [Advent of Code](https://adventofcode.com) 2022!

## Test

```sh
# Test all days so far
% cargo test

# Test a single day example
% cargo test day01
   Compiling advent-of-code-2022 v0.1.0 (/Users/simon/git/advent-of-code-2022)
    Finished test [unoptimized + debuginfo] target(s) in 0.53s
     Running unittests src/main.rs (target/debug/deps/advent_of_code_2022-c2a0ff035b11f5d4)

running 2 tests
test solutions::day01::tests::part_two ... ok
test solutions::day01::tests::part_one ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Execute

```sh
# Run a specific day
% carog run <day: i32>

# Example
% cargo run 1
   Compiling advent-of-code-2022 v0.1.0 (/Users/simon/git/advent-of-code-2022)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/advent-of-code-2022 1`
Showing solution for day 1

Solution part 1: 72017
Solution part 2: 212520

```

# Advent of Code 2021

Programs to calculate solutions for the [Advent of Code](https://adventofcode.com/2021/about) challenges.

## Build

Each daily solution comes in the form of its own binary, which can be built using
[cargo](https://doc.rust-lang.org/cargo/).

You might need to make very small adjustments to the code, such as calling a different function for part 1 and part 2
of the solution.

The binaries take as their first and only argument the path to the input file to solve the challenge with.

### Examples

*Calculating the solution to the second challenge*
```shell
santa@Holiday-Box:~$ cargo run -p day-02 -- $PATH_TO_INPUT_FILE
```
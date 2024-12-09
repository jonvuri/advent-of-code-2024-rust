mod utils;

mod day1;
mod day2;

aoc_main::main! {
    year 2024;
    day1 : generator => part1, part2;
    day2 : generator => part1, part2;
}

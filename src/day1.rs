use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<(i32, i32)> {
    parser!(lines(i32 "   " i32))
        .parse(&utils::newline_end(input))
        .unwrap()
}

pub fn part1(input: &[(i32, i32)]) -> i32 {
    let first = input.iter().map(|line| line.0);
    let second = input.iter().map(|line| line.1);

    // Sort first numerically:
    let mut first_sorted = first.clone().collect::<Vec<i32>>();
    first_sorted.sort();

    // Sort second numerically:
    let mut second_sorted = second.clone().collect::<Vec<i32>>();
    second_sorted.sort();

    // Zip first and second, subtracting the first from the second:
    let zipped = first_sorted.iter().zip(second_sorted.iter());
    let differences = zipped.map(|(a, b)| (b - a).abs());

    // Sum up the differences:
    differences.sum()
}

pub fn part2(input: &[(i32, i32)]) -> i32 {
    let first = input.iter().map(|line| line.0);
    let second = input.iter().map(|line| line.1);

    // Build a map from second, from the numerical value to the number of times it appears:
    let second_map = second
        .clone()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    // Iterate through first, and for each value, multiply it by the number of times it appears in second (from second_map):
    let mut sum = 0;
    for x in first {
        // Skip if the value is not in second_map:
        if !second_map.contains_key(&x) {
            continue;
        }
        sum += x * second_map.get(&x).unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day1::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parses_example() {
        let example_vec: Vec<(i32, i32)> = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        assert_eq!(generator(EXAMPLE_INPUT), example_vec);
    }

    #[test]
    fn part1_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_sums_top_3() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part2(&input), 31);
    }
}

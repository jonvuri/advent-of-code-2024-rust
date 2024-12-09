use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<Vec<i32>> {
    parser!(lines(repeat_sep(i32, " ")))
        .parse(&utils::newline_end(input))
        .unwrap()
}

pub fn part1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .filter(|row| {
            // Assume the row has at least two elements
            if row.len() < 2 {
                return false;
            }

            // Determine if the row is descending or ascending
            let descending = row[0] > row[1];

            // Compare each pair of elements in the row to see if any or descending or too far apart
            row.iter().zip(row.iter().skip(1)).all(|(a, b)| {
                if descending {
                    *a > *b && *b >= *a - 3
                } else {
                    *a < *b && *b <= *a + 3
                }
            })
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .filter(|row| {
            // Assume the row has at least two elements
            if row.len() < 2 {
                return false;
            }

            // Determine if the row is descending or ascending
            let descending = row[0] > row[1];

            // Compare each pair of elements in the row to see if any or descending or too far apart
            let all_safe = row.iter().zip(row.iter().skip(1)).all(|(a, b)| {
                if descending {
                    *a > *b && *b >= *a - 3
                } else {
                    *a < *b && *b <= *a + 3
                }
            });

            if all_safe {
                return true;
            } else {
                // If the row is not safe, check if it can be made safe by removing an element -
                // Remove elements one by one and then check if the new row is safe
                for i in 0..row.len() {
                    let mut new_row = (*row).clone();
                    new_row.remove(i);

                    let descending = new_row[0] > new_row[1];

                    if new_row.iter().zip(new_row.iter().skip(1)).all(|(a, b)| {
                        if descending {
                            *a > *b && *b >= *a - 3
                        } else {
                            *a < *b && *b <= *a + 3
                        }
                    }) {
                        return true;
                    }
                }
            }

            false
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day2::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parses_example() {
        let example_vec: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(generator(EXAMPLE_INPUT), example_vec);
    }

    #[test]
    fn part1_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part2(&input), 4);
    }
}

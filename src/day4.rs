use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<Vec<char>> {
    parser!(lines(upper+))
        .parse(&utils::newline_end(input))
        .unwrap()
}

pub fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    // For horizontal, vertical, and diagonal traversal:
    // - For each row:
    //     - Keep track of consecutive 'XMAS' characters both forwards and backwards
    //     - For every completed 'XMAS' sequence, increment

    // Horizontal traversal:
    for row in input.iter() {
        let mut forward = 0;
        let mut backward = 0;

        for c in row.iter() {
            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    // Vertical traversal:
    for i in 0..input[0].len() {
        let mut forward = 0;
        let mut backward = 0;

        for j in 0..input.len() {
            let c = input[j][i];

            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    // Diagonal traversal (lower half going down, starting from the top left):
    // X * * *
    // S M * *
    // A S A *
    // M A S S
    for i in 0..input.len() {
        let mut forward = 0;
        let mut backward = 0;

        for j in 0..input[0].len() {
            if i + j >= input.len() {
                break;
            }

            let c = input[i + j][j];

            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    // Diagonal traversal (upper half going down, starting from the top left + 1):
    // * X M S S
    // * * M M A
    // * * * A M
    // * * * * S
    // * * * * *
    for i in 1..input[0].len() {
        let mut forward = 0;
        let mut backward = 0;

        for j in 0..input.len() {
            if j + i >= input[0].len() {
                break;
            }

            let c = input[j][j + i];

            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    // Diagonal traversal (lower half going up, starting from the bottom left):
    // * * * X
    // * * A X
    // * M A A
    // S A S S
    for i in 0..input.len() {
        let mut forward = 0;
        let mut backward = 0;

        for j in 0..input[0].len() {
            if j > i {
                break;
            }

            let c = input[i - j][j];

            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    // Diagonal traversal (upper half going up, starting from the bottom left + 1):
    // X S A X *
    // M A M * *
    // A S * * *
    // S * * * *
    // * * * * *
    for i in 1..input[0].len() {
        let mut forward = 0;
        let mut backward = 0;

        for j in 0..input.len() {
            if j + i >= input[0].len() {
                break;
            }

            let c = input[input.len() - 1 - j][j + i];

            forward = match forward {
                0 => match c {
                    'X' => 1,
                    _ => 0,
                },
                1 => match c {
                    'X' => 1,
                    'M' => 2,
                    _ => 0,
                },
                2 => match c {
                    'X' => 1,
                    'A' => 3,
                    _ => 0,
                },
                3 => match c {
                    'X' => 1,
                    'S' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid forward value");
                }
            };

            if forward == 4 {
                sum += 1;
                forward = 0;
            }

            backward = match backward {
                0 => match c {
                    'S' => 1,
                    _ => 0,
                },
                1 => match c {
                    'S' => 1,
                    'A' => 2,
                    _ => 0,
                },
                2 => match c {
                    'S' => 1,
                    'M' => 3,
                    _ => 0,
                },
                3 => match c {
                    'S' => 1,
                    'X' => 4,
                    _ => 0,
                },
                _ => {
                    panic!("Invalid backward value");
                }
            };

            if backward == 4 {
                sum += 1;
                backward = 0;
            }
        }
    }

    sum
}

pub fn part2(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    // For each non-edge cell:
    // - Check if the cell is 'A'
    //   - If so, check cells to the top left, top right, bottom left, and bottom right:
    //     If the top left and bottom right are 'M' and 'S', or vice versa,
    //     and the top right and bottom left are also 'M' and 'S' or vice versa, increment sum
    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            if input[i][j] == 'A' {
                if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S')
                    || (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
                {
                    if (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S')
                        || (input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
                    {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day4::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn part2_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part2(&input), 9);
    }
}

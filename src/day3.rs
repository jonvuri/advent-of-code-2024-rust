use regex::Regex;

pub fn generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let captures_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.find_iter(input)
        .map(|mult| {
            let captures = captures_re.captures(mult.as_str()).unwrap();
            let left = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let right = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            left * right
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(?<mult>mul\(\d+,\d+\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let captures_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;

    re.captures_iter(input)
        .map(|captures| {
            if let Some(mult) = captures.name("mult") {
                if enabled {
                    let captures = captures_re.captures(mult.as_str()).unwrap();
                    let left = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let right = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    return left * right;
                }
            } else if captures.name("do").is_some() {
                enabled = true;
            } else if captures.name("dont").is_some() {
                enabled = false;
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_finds_sum() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part1(&input), 161);
    }

    static EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part2_finds_sum() {
        let input = generator(EXAMPLE_INPUT2);

        assert_eq!(part2(&input), 48);
    }
}

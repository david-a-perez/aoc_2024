use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use regex::bytes::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    input
        .windows(4)
        .enumerate()
        .filter(|(_, window)| window == b"mul(")
        .map(|(i, _)| {
            let i = i + 4;

            let (a, r) = usize::from_radix_10(&input[i..]);

            let i = i + r;

            if input[i] != b',' {
                return 0;
            }

            let i = i + 1;

            let (b, r) = usize::from_radix_10(&input[i..]);
            let i = i + r;

            if input[i] != b')' {
                return 0;
            }
            a * b
        })
        .sum()
}

#[aoc(day3, part1, Memmem)]
fn part1_memmem(input: &str) -> usize {
    let input = input.as_bytes();

    memchr::memmem::find_iter(input, "mul(")
        .map(|i| {
            let i = i + 4;

            let (a, r) = usize::from_radix_10(&input[i..]);

            let i = i + r;

            if input[i] != b',' {
                return 0;
            }

            let i = i + 1;

            let (b, r) = usize::from_radix_10(&input[i..]);
            let i = i + r;

            if input[i] != b')' {
                return 0;
            }
            a * b
        })
        .sum()
}

#[aoc(day3, part1, MemchrIter)]
fn part1_memchr_iter(input: &str) -> usize {
    let input = input.as_bytes();

    memchr::memchr_iter(b'(', input)
        .map(|i| {
            if &input[i - 3..i] != b"mul" {
                return 0;
            }

            let i = i + 1;

            let (a, r) = usize::from_radix_10(&input[i..]);

            let i = i + r;

            if input[i] != b',' {
                return 0;
            }

            let i = i + 1;

            let (b, r) = usize::from_radix_10(&input[i..]);
            let i = i + r;

            if input[i] != b')' {
                return 0;
            }
            a * b
        })
        .sum()
}

#[aoc(day3, part1, Regex)]
fn part1_regex(input: &str) -> usize {
    let input = input.as_bytes();

    let re = Regex::new(r"mul\(").unwrap();

    re.find_iter(input)
        .map(|i| {
            let i = i.end();

            let (a, r) = usize::from_radix_10(&input[i..]);

            let i = i + r;

            if input[i] != b',' {
                return 0;
            }

            let i = i + 1;

            let (b, r) = usize::from_radix_10(&input[i..]);
            let i = i + r;

            if input[i] != b')' {
                return 0;
            }
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let input = input.as_bytes();

    let mut enabled = true;

    input
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'(')
        .map(|(i, _)| {
            if enabled {
                if &input[i - 3..i] == b"mul" {
                    let i = i + 1;

                    let (a, r) = usize::from_radix_10(&input[i..]);

                    let i = i + r;

                    if input[i] != b',' {
                        return 0;
                    }

                    let i = i + 1;

                    let (b, r) = usize::from_radix_10(&input[i..]);
                    let i = i + r;

                    if input[i] != b')' {
                        return 0;
                    }
                    a * b
                } else if &input[i - 5..i + 2] == b"don't()" {
                    enabled = false;
                    0
                } else {
                    0
                }
            } else if &input[i - 2..i + 2] == b"do()" {
                enabled = true;
                0
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}

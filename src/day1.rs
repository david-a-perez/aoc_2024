use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::{iterator, map, opt},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_num(i: &[u8]) -> IResult<&[u8], usize> {
    map(take(5usize), |digit_str: &[u8]| usize::from_radix_10(digit_str).0)(i)
}

fn parse_line(i: &[u8]) -> IResult<&[u8], (usize, usize)> {
    terminated(
        separated_pair(parse_num, tag("   "), parse_num),
        opt(line_ending),
    )(i)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut it = iterator(input.as_bytes(), parse_line);

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = it.unzip();

    it.finish().unwrap();

    col1.sort();
    col2.sort();

    col1.into_iter()
        .zip(col2.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<usize>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut it = iterator(input.as_bytes(), parse_line);

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = it.unzip();

    it.finish().unwrap();

    col1.sort();
    col2.sort();

    col1.into_iter()
        .dedup_with_count()
        .merge_join_by(col2.into_iter().dedup_with_count(), |(_, a), (_, b)| {
            a.cmp(b)
        })
        .map(|r| match r {
            itertools::EitherOrBoth::Both((a, c), (b, _)) => a * b * c,
            _ => 0,
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n").to_string(),
            "11"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n").to_string(),
            "31"
        );
    }
}

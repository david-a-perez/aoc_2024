use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{iterator, map_res, opt},
    sequence::{separated_pair, terminated},
    IResult,
};

fn parse_num(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |digit_str: &str| digit_str.parse())(i)
}

fn parse_line(i: &str) -> IResult<&str, (usize, usize)> {
    terminated(
        separated_pair(parse_num, tag("   "), parse_num),
        opt(line_ending),
    )(i)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut it = iterator(input, parse_line);

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = it.unzip();

    it.finish().unwrap();

    col1.sort_unstable();
    col2.sort_unstable();

    col1.into_iter()
        .zip(col2.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<usize>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut it = iterator(input, parse_line);

    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = it.unzip();

    it.finish().unwrap();

    col1.sort_unstable();
    col2.sort_unstable();

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
            part1("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n").to_string(),
            "11"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n").to_string(),
            "31"
        );
    }
}

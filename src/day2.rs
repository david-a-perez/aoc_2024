use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::{eof, iterator, map, value, verify},
    sequence::{pair, preceded}, IResult,
};

fn parse_num(i: &[u8]) -> IResult<&[u8], usize> {
    map(digit1, |digit_str: &[u8]| usize::from_radix_10(digit_str).0)(i)
}

fn increasing(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, num) = verify(preceded(tag(" "), parse_num), |&num| {
            matches!(num.wrapping_sub(previous_num), 1..=3)
        })(i)?;

        alt((value((), alt((line_ending, eof))), increasing(num)))(i)
    }
}

fn decreasing(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, num) = verify(preceded(tag(" "), parse_num), |&num| {
            matches!(previous_num.wrapping_sub(num), 1..=3)
        })(i)?;

        alt((value((), alt((line_ending, eof))), decreasing(num)))(i)
    }
}

fn parse_line(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, head) = parse_num(i)?;

    alt((
        value(true, increasing(head)),
        value(true, decreasing(head)),
        value(false, pair(not_line_ending, alt((line_ending, eof)))),
    ))(i)
}

#[aoc(day2, part1)]
fn part1<'a>(input: &str) -> usize {
    let mut it = iterator(input.as_bytes(), parse_line);

    let sum = it.map(|x| if x { 1 } else { 0 }).sum::<usize>();

    assert!(it.finish().unwrap().0.is_empty());

    sum
}

fn skip_and_increasing(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, _) = preceded(tag(" "), digit1)(i)?;

        alt((value((), alt((line_ending, eof))), increasing(previous_num)))(i)
    }
}

fn increasing_with_safety(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, num) = verify(preceded(tag(" "), parse_num), |&num| {
            matches!(num.wrapping_sub(previous_num), 1..=3)
        })(i)?;

        alt((
            value((), alt((line_ending, eof))),
            increasing_with_safety(num),
            skip_and_increasing(num),
        ))(i)
    }
}

fn skip_and_decreasing(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, _) = preceded(tag(" "), digit1)(i)?;

        alt((value((), alt((line_ending, eof))), decreasing(previous_num)))(i)
    }
}

fn decreasing_with_safety(previous_num: usize) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> {
    move |i: &[u8]| {
        let (i, num) = verify(preceded(tag(" "), parse_num), |&num| {
            matches!(previous_num.wrapping_sub(num), 1..=3)
        })(i)?;

        alt((
            value((), alt((line_ending, eof))),
            decreasing_with_safety(num),
            skip_and_decreasing(num),
        ))(i)
    }
}

fn parse_line_with_safety(i: &[u8]) -> IResult<&[u8], bool> {
    let (i, head) = parse_num(i)?;

    alt((
        value(true, increasing_with_safety(head)),
        value(true, decreasing_with_safety(head)),
        value(true, skip_and_increasing(head)),
        value(true, skip_and_decreasing(head)),
        preceded(tag(" "), parse_line),
        value(false, pair(not_line_ending, alt((line_ending, eof)))),
    ))(i)
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut it = iterator(input.as_bytes(), parse_line_with_safety);

    let sum = it.map(|x| if x { 1 } else { 0 }).sum::<usize>();

    assert!(it.finish().unwrap().0.is_empty());

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n"),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n"),
            4
        );
    }
}

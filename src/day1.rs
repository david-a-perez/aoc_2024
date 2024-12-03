use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = input
        .as_bytes()
        .chunks(5 + 3 + 5 + 1)
        .map(|line| {
            (
                usize::from_radix_10(&line[..5]).0,
                usize::from_radix_10(&line[8..13]).0,
            )
        })
        .unzip();

    col1.sort();
    col2.sort();

    col1.into_iter()
        .zip(col2.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<usize>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let (mut col1, mut col2): (Vec<usize>, Vec<usize>) = input
        .as_bytes()
        .chunks(5 + 3 + 5 + 1)
        .map(|line| {
            (
                usize::from_radix_10(&line[..5]).0,
                usize::from_radix_10(&line[8..13]).0,
            )
        })
        .unzip();

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

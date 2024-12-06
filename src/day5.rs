use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::{map, opt, value},
    multi::fold_many1,
    sequence::{preceded, separated_pair, terminated},
    Err, IResult,
};

fn parse_num(i: &[u8]) -> IResult<&[u8], usize> {
    map(digit1, |digit_str: &[u8]| usize::from_radix_10(digit_str).0)(i)
}

fn parse_rules(i: &[u8]) -> IResult<&[u8], [u128; 100]> {
    fold_many1(
        terminated(
            separated_pair(parse_num, take(1usize), parse_num),
            line_ending,
        ),
        || [0u128; 100],
        |mut acc, (before, after)| {
            acc[before] |= 1 << after;
            acc
        },
    )(i)
}

fn parse_update(rules: &[u128; 100]) -> impl FnMut(&[u8]) -> IResult<&[u8], usize> + use<'_> {
    move |mut i| {
        let original_i = i;

        let mut offset_to_middle = 0;

        let (i1, initial_page) = parse_num(i)?;
        i = i1;

        let mut state = 1 << initial_page;

        loop {
            match preceded(tag(","), parse_num)(i) {
                Err(Err::Error(_)) => {
                    let (_, middle) = parse_num(&original_i[offset_to_middle..])?;

                    return Ok((i, middle));
                }
                Err(e) => return Err(e),
                Ok((i1, page)) => {
                    i = i1;
                    let must_be_after = rules[page];

                    if state & must_be_after != 0 {
                        return value(0, not_line_ending)(i);
                    }

                    state |= 1 << page;
                }
            }

            let (i1, page) = preceded(take(1usize), parse_num)(i)?;
            i = i1;
            let must_be_after = rules[page];

            if state & must_be_after != 0 {
                return value(0, not_line_ending)(i);
            }

            state |= 1 << page;

            offset_to_middle += 3;
        }
    }
}

fn parse_updates<'a, 'b>(i: &'a [u8], rules: &'b [u128; 100]) -> IResult<&'a [u8], usize> {
    fold_many1(
        terminated(parse_update(rules), opt(line_ending)),
        || 0,
        |mut acc, middle| {
            acc += middle;
            acc
        },
    )(i)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    let (input, rules) = parse_rules(input).unwrap();

    let (_, total) = parse_updates(&input[1..], &rules).unwrap();

    total
}

fn bubble_sort(list: &mut [usize], rules: &[u128; 100]) -> bool {
    let mut n = list.len();
    let mut swapped = false;
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            if rules[list[i]] & (1 << list[i - 1]) != 0 {
                list.swap(i, i - 1);
                new_n = i;
                swapped = true;
            }
        }
        n = new_n
    }
    swapped
}

pub fn comma_separated_nums(
    res: &mut Vec<usize>,
) -> impl FnMut(&[u8]) -> IResult<&[u8], ()> + use<'_> {
    move |mut i| {
        res.clear();

        // Parse the first element
        match parse_num(i) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            match tag(",")(i) {
                Err(Err::Error(_)) => return Ok((i, ())),
                Err(e) => return Err(e),
                Ok((i1, _)) => match parse_num(i1) {
                    Err(Err::Error(_)) => return Ok((i, ())),
                    Err(e) => return Err(e),
                    Ok((i2, o)) => {
                        res.push(o);
                        i = i2;
                    }
                },
            }
        }
    }
}

fn parse_incorrect_update<'a, 'b>(
    rules: &'a [u128; 100],
) -> impl FnMut(&[u8]) -> IResult<&[u8], usize> + use<'a, 'b> {
    let mut list = Vec::new();
    move |i| {
        let (i, ()) = comma_separated_nums(&mut list)(i)?;

        let swapped = bubble_sort(&mut list, rules);

        if swapped {
            Ok((i, list[list.len() / 2]))
        } else {
            Ok((i, 0))
        }
    }
}

fn parse_incorrect_updates<'a, 'b>(
    i: &'a [u8],
    rules: &'b [u128; 100],
) -> IResult<&'a [u8], usize> {
    fold_many1(
        terminated(parse_incorrect_update(rules), opt(line_ending)),
        || 0,
        |mut acc, middle| {
            acc += middle;
            acc
        },
    )(i)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let (input, rules) = parse_rules(input).unwrap();

    let (_, total) = parse_incorrect_updates(&input[1..], &rules).unwrap();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47"), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47"), 123);
    }
}

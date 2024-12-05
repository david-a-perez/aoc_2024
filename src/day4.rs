use aoc_runner_derive::aoc;
use itertools::izip;

fn index(input: &[u8], row: Option<usize>, col: Option<usize>, cols: usize) -> Option<u8> {
    if let (Some(col), Some(row)) = (col, row) {
        if col < cols {
            input.get(row * cols + col).copied()
        } else {
            None
        }
    } else {
        None
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let offsets2: [(isize, isize); 8] = [
        (-2, -2),
        (-2, 0),
        (-2, 2),
        (0, -2),
        (0, 2),
        (2, -2),
        (2, 0),
        (2, 2),
    ];

    let offsets3: [(isize, isize); 8] = [
        (-3, -3),
        (-3, 0),
        (-3, 3),
        (0, -3),
        (0, 3),
        (3, -3),
        (3, 0),
        (3, 3),
    ];

    let mut total = 0;

    let cols = input.iter().position(|&c| c == b'\n').unwrap() + 1;

    let rows = input.len().div_ceil(cols);

    for row in 0..rows {
        for col in 0..cols - 1 {
            if index(input, Some(row), Some(col), cols) == Some(b'X') {
                for ((row_offset, col_offset), (row_offset2, col_offset2), (row_offset3, col_offset3)) in izip!(offsets, offsets2, offsets3) {
                    if index(
                        input,
                        row.checked_add_signed(row_offset3),
                        col.checked_add_signed(col_offset3),
                        cols,
                    ) == Some(b'S')
                    {
                        if index(
                            input,
                            row.checked_add_signed(row_offset2),
                            col.checked_add_signed(col_offset2),
                            cols,
                        ) == Some(b'A')
                        {
                            if index(
                                input,
                                row.checked_add_signed(row_offset),
                                col.checked_add_signed(col_offset),
                                cols,
                            ) == Some(b'M')
                            {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    total
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();

    let mut total = 0;

    let cols = input.iter().position(|&c| c == b'\n').unwrap() + 1;

    let rows = input.len().div_ceil(cols);

    for row in 1..rows - 1 {
        for col in 1..cols - 2 {
            if index(input, Some(row), Some(col), cols) == Some(b'A') {
                let top_left = index(
                    input,
                    row.checked_add_signed(-1),
                    col.checked_add_signed(-1),
                    cols,
                );
                let bottom_right = index(
                    input,
                    row.checked_add_signed(1),
                    col.checked_add_signed(1),
                    cols,
                );
                if (top_left == Some(b'M') && bottom_right == Some(b'S'))
                    || (top_left == Some(b'S') && bottom_right == Some(b'M'))
                {
                    let top_right = index(
                        input,
                        row.checked_add_signed(-1),
                        col.checked_add_signed(1),
                        cols,
                    );
                    let bottom_left = index(
                        input,
                        row.checked_add_signed(1),
                        col.checked_add_signed(-1),
                        cols,
                    );
                    if (top_right == Some(b'M') && bottom_left == Some(b'S'))
                        || (top_right == Some(b'S') && bottom_left == Some(b'M'))
                    {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n"), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n"), 9);
    }
}

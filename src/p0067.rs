// Maximum path sum II
//
// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
// the maximum total from top to bottom is 23.
//
// 3
// 7 4
// 2 4 6
// 8 5 9 3
//
// That is, 3 + 7 + 4 + 9 = 23.
//
// Find the maximum total from top to bottom in triangle.txt,
// a 15K text file containing a triangle with one-hundred rows.
//
// NOTE: This is a much more difficult version of Problem 18.
// It is not possible to try every route to solve this problem, as there are 2^99 altogether!
// If you could check one trillion (10^12) routes every second it would take over twenty
// billion years to check them all. There is an efficient algorithm to solve it. ;o)

use std::fs;

fn compute(input: Vec<u32>) -> u32 {
    let mut curr_row_len = num_rows(&input);
    let mut curr_row_idx = input.len() - curr_row_len;
    let mut scratch = vec![0; curr_row_len + 1];

    while curr_row_len != 0 {
        for i in 0..curr_row_len {
            scratch[i] = input[curr_row_idx + i] + scratch[i].max(scratch[i + 1]);
        }

        curr_row_len -= 1;
        curr_row_idx -= curr_row_len;
    }

    scratch[0]
}

// I sure had to Google to remember this gem.
// Takes the total count of numbers in the input (triangle) and determines how many rows
// there are, which is also the length of the last row.
fn num_rows(input: &Vec<u32>) -> usize {
    ((((input.len() * 8 + 1) as f64).sqrt() as usize) - 1) / 2
}

fn parse_input(input: String) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

#[test]
fn example() {
    assert_eq!(23, compute(vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3]));
}

#[test]
fn problem() {
    let input = fs::read_to_string("./resources/p0067_triangle.txt").expect("Unable to read file");

    assert_eq!(7273, compute(parse_input(input)));
}

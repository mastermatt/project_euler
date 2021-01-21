// Maximum path sum I
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
// Find the maximum total from top to bottom of the triangle below:
//
// 75
// 95 64
// 17 47 82
// 18 35 87 10
// 20 04 82 47 65
// 19 01 23 75 03 34
// 88 02 77 73 07 63 67
// 99 65 04 28 06 16 70 92
// 41 41 26 56 83 40 80 70 33
// 41 48 72 33 47 32 37 16 94 29
// 53 71 44 65 25 43 91 52 97 51 14
// 70 11 33 28 77 73 17 78 39 68 17 57
// 91 71 52 38 17 14 91 43 58 50 27 29 48
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
//
// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
// However, Problem 67, is the same challenge with a triangle containing one-hundred rows;
// it cannot be solved by brute force, and requires a clever method! ;o)

fn compute(input: Vec<u32>) -> u32 {
    let mut curr_row_len = num_rows(&input);
    let mut curr_row_idx = input.len() - curr_row_len;
    let mut scratch = vec![0; curr_row_len + 1];
    let mut vectors = vec![0; input.len()];

    while curr_row_len != 0 {
        for i in 0..curr_row_len {
            vectors[curr_row_idx + i] = if scratch[i] > scratch[i + 1] { 0 } else { 1 }; // 0 if the final trace goes left, 1 if right
            scratch[i] = input[curr_row_idx + i] + scratch[i].max(scratch[i + 1]);
        }

        curr_row_len -= 1;
        curr_row_idx -= curr_row_len;
    }

    println!("{:?}", vectors);
    // [1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

    let mut path = vec![0; num_rows(&input)];
    let mut idx = 0;
    for row in 0..num_rows(&input) {
        path[row] = input[idx];
        idx += row + 1 + vectors[idx];
    }
    println!("{:?}", path);
    // [75, 64, 82, 87, 82, 75, 73, 28, 83, 32, 91, 78, 58, 73, 93]

    scratch[0]
}

// I sure had to Google to remember this gem.
// Takes the total count of numbers in the input (triangle) and determines how many rows
// there are, which is also the length of the last row.
fn num_rows(input: &Vec<u32>) -> usize {
    ((((input.len() * 8 + 1) as f64).sqrt() as usize) - 1) / 2
}

#[test]
fn example() {
    assert_eq!(23, compute(vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3]));
}

#[test]
fn problem() {
    let v = vec![
        75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34, 88, 02,
        77, 73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40, 80, 70, 33, 41,
        48, 72, 33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 70, 11, 33,
        28, 77, 73, 17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 63,
        66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 04, 62, 98, 27, 23, 09, 70, 98, 73, 93,
        38, 53, 60, 04, 23,
    ];

    assert_eq!(1074, compute(v));
}

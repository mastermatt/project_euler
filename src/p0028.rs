// Number spiral diagonals
//
// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
//
// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13
//
// It can be verified that the sum of the numbers on the diagonals is 101.
//
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

fn compute(limit: usize) -> usize {
    let mut idx = 1;
    let mut result = 1;

    for i in (1..limit).step_by(2) {
        for _ in 0..4 {
            idx += i + 1;
            result += idx;
        }
    }

    result
}

#[test]
fn example() {
    assert_eq!(101, compute(5));
}

#[test]
fn problem() {
    assert_eq!(669171001, compute(1001));
}

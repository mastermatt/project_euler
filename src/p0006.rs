// Sum square difference
//
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers
// and the square of the sum is 3025 - 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

fn compute(bound: u32) -> u32 {
    square_of_sum(bound) - sum_of_squares(bound)
}

fn sum_of_squares(bound: u32) -> u32 {
    (1..=bound).map(|n| n.pow(2)).sum()
}

fn square_of_sum(bound: u32) -> u32 {
    (1..=bound).sum::<u32>().pow(2)
}

#[test]
fn example() {
    assert_eq!(2640, compute(10));
}

#[test]
fn problem() {
    assert_eq!(25164150, compute(100));
}

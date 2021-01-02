// Multiples of 3 and 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

fn compute(bound: u32) -> u32 {
    (1..bound).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

#[test]
fn example() {
    assert_eq!(23, compute(10));
}

#[test]
fn problem() {
    assert_eq!(233168, compute(1000));
}

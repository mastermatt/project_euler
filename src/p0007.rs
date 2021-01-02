// 10001st prime
//
// By listing the first six prime numbers:
// 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10,001st prime number?

fn compute(n: usize) -> u32 {
    (1..).filter(|i| is_prime(i)).nth(n).unwrap()
}

fn is_prime(n: &u32) -> bool {
    let s = (*n as f64).sqrt() as u32;
    (2..=s).all(|i| n % i != 0)
}

#[test]
fn example() {
    assert_eq!(13, compute(6));
}

#[test]
fn problem() {
    assert_eq!(104743, compute(10_001));
}

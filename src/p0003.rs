// Largest prime factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143?

fn compute(num: u64) -> u64 {
    let mut divisor = 2;
    let mut n = num;

    while n >= 2 {
        if n % divisor == 0 {
            n /= divisor;
        } else {
            divisor += 1;
        }
    }

    divisor
}

#[test]
fn example() {
    assert_eq!(29, compute(13195));
}

#[test]
fn problem() {
    assert_eq!(6857, compute(600851475143));
}

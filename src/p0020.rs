// Factorial digit sum
//
// n! means n × (n − 1) × ... × 3 × 2 × 1
//
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//
// Find the sum of the digits in the number 100!

use num::bigint::{BigUint, ToBigUint};
use num::integer::Integer;
use num::traits::{One, ToPrimitive, Zero};

fn compute(start: u32) -> u32 {
    let mut factorial = One::one();

    for i in 2..=start {
        factorial *= i;
    }

    sum_digits(factorial)
}

fn sum_digits(num: BigUint) -> u32 {
    let mut n = num;
    let mut result: u32 = 0;
    let ten = 10.to_biguint().unwrap();
    let zero = Zero::zero();

    while n != zero {
        let (d, m) = n.div_mod_floor(&ten);

        result += m.to_u32().unwrap();
        n = d;
    }

    result
}

#[test]
fn example() {
    assert_eq!(27, compute(10));
}

#[test]
fn problem() {
    assert_eq!(648, compute(100));
}

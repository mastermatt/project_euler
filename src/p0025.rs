// 1000-digit Fibonacci number
//
// The Fibonacci sequence is defined by the recurrence relation:
//
// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:
//
// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.
//
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use num::bigint::BigUint;
use num::traits::One;
use std::mem::replace;

fn compute(digits: usize) -> usize {
    let mut a: BigUint = One::one();
    let mut b: BigUint = One::one();
    let ten_log2 = 10f64.log2();

    for i in 3.. {
        let c = a + &b;

        // I found that string conversion of the BigUint was pretty slow and caused the problem to
        // take ~600ms. But by digging around the implementation of BigUint, I found this size
        // approximation in `to_radix_digits_le` which dropped the runtime for the problem to ~8ms.
        let approx_len = ((c.bits() as f64) / ten_log2).ceil() as usize;
        if approx_len >= digits && c.to_str_radix(10).len() >= digits {
            return i;
        }

        // This is a low cost way of swapping a with b and b with c.
        a = replace(&mut b, c);
    }
    0
}

#[test]
fn example() {
    assert_eq!(12, compute(3));
}

#[test]
fn problem() {
    assert_eq!(4782, compute(1000));
}

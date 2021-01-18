// Non-abundant sums
//
// A perfect number is a number for which the sum of its proper divisors is exactly
// equal to the number. For example, the sum of the proper divisors of 28 would
// be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
//
// A number n is called deficient if the sum of its proper divisors is less than n and it
// is called abundant if this sum exceeds n.
//
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
// upper limit cannot be reduced any further by analysis even though it is known that the greatest
// number that cannot be expressed as the sum of two abundant numbers is less than this limit.
//
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

use crate::util::factors;

fn compute() -> usize {
    let limit = 28123usize;
    let abundant_nums: Vec<usize> = (1..limit).filter(|&n| is_abundant(n)).collect();
    let mut sumable = vec![false; limit + 1];
    let mut result = 0;

    for (i, &a) in abundant_nums.iter().enumerate() {
        for &b in &abundant_nums[i..] {
            let s = a + b;
            if s > limit {
                break;
            }
            sumable[s] = true;
        }
    }

    for i in 1..limit {
        if !sumable[i] {
            result += i;
        }
    }

    result
}

fn is_abundant(num: usize) -> bool {
    let mut divisors = factors::divisors(num as u64);
    divisors.pop(); // remove the actual num from the end of the list
    let sum: u64 = divisors.iter().sum();
    num < (sum as usize)
}

#[test]
fn problem() {
    assert_eq!(4179871, compute());
}

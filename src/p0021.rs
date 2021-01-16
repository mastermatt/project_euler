// Amicable numbers
//
// Let d(n) be defined as the sum of proper divisors of n
// (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and
// each of a and b are called amicable numbers.
//
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//
// Evaluate the sum of all the amicable numbers under 10000.

use crate::util::factors;

fn compute(limit: usize) -> usize {
    let mut result = 0;
    let mut sums = vec![0; limit];

    for i in 1..limit {
        let sum = sum_proper_divisors(i);
        sums[i] = sum;

        if i != sum && sum < limit && sums[sum] == i {
            result += i + sum;
        }
    }

    result
}

fn sum_proper_divisors(num: usize) -> usize {
    let mut divisors = factors::divisors(num as u64);
    divisors.pop(); // remove the actual num from the end of the list
    divisors.iter().map(|&n| n as usize).sum()
}

#[test]
fn example_sums() {
    assert_eq!(284, sum_proper_divisors(220));
    assert_eq!(220, sum_proper_divisors(284));
}

#[test]
fn problem() {
    assert_eq!(31626, compute(10_000));
}

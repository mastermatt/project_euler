// https://codereview.stackexchange.com/a/120646
// https://en.wikipedia.org/wiki/Divisor_function
// this is stupid fast
pub fn count_divisors(num: u64) -> u32 {
    let mut num_divisors = 1;
    let mut factor = 2; // Candidate for prime factor of `n`
    let mut n = num;
    let mut exponent;

    // If `n` is not a prime number then it must have one factor
    // which is <= `sqrt(n)`, so we try these first:
    while factor * factor <= n {
        if n % factor == 0 {
            // `factor` is a prime factor of `n`, determine the exponent:
            exponent = 0;
            loop {
                n /= factor;
                exponent += 1;
                if n % factor != 0 {
                    break;
                }
            }
            // `factor^exponent` is one term in the prime factorization of n,
            // this contributes as factor `exponent + 1`:
            num_divisors *= exponent + 1;
        }
        // Next possible prime factor:
        factor = if factor == 2 { 3 } else { factor + 2 }
    }

    // Now `n` is either 1 or a prime number. In the latter case,
    // it contributes a factor 2:
    if n > 1 {
        num_divisors *= 2;
    }

    num_divisors
}

#[test]
fn check_count_divisors() {
    assert_eq!(6, count_divisors(28));
    assert_eq!(90, count_divisors(25200));
    assert_eq!(576, count_divisors(76576500));
}

pub fn divisors(num: u64) -> Vec<u64> {
    if num == 1 {
        return vec![1];
    }

    let mut lower = Vec::new();
    let mut upper = Vec::new();
    let sqrt = (num as f64).sqrt() as u64;
    let is_perfect_square = sqrt * sqrt == num;
    let loop_to = if is_perfect_square { sqrt - 1 } else { sqrt };

    for i in 1..=loop_to {
        if num % i == 0 {
            lower.push(i);
            upper.push(num / i);
        }
    }

    if is_perfect_square {
        lower.push(sqrt);
    }

    upper.reverse();
    lower.append(&mut upper);
    lower
}

#[test]
fn check_divisors() {
    assert_eq!(divisors(1), [1]);
    assert_eq!(divisors(6), [1, 2, 3, 6]);
    assert_eq!(divisors(16), [1, 2, 4, 8, 16]); // (perfect square)
    assert_eq!(divisors(17), [1, 17]); // (prime)
}

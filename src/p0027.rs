// Quadratic primes
//
// Euler discovered the remarkable quadratic formula: n^2 + n + 41
//
// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39.
// However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when
// n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
//
// The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the
// consecutive values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form: n^2 + an + b, where |a| < 1000 and |b| <= 1000
// where |n| is the modulus/absolute value of n. e.g. |11| = 11 and |-4| = 4
//
// Find the product of the coefficients, a and b, for the quadratic expression that produces the
// maximum number of primes for consecutive values of n, starting with n = 0.

use primal::Sieve;

fn compute(limit: usize) -> isize {
    let sieve = primal::Sieve::new(100_000);

    // collect all the primes under the limit and their negatives
    let mut coefficients = vec![1isize, -1];
    for n in sieve.primes_from(0).take_while(|x| *x < limit) {
        coefficients.push(n as isize);
        coefficients.push(-1 * n as isize);
    }

    let mut max_count = 0;
    let mut max_prod = 0;

    for &a in &coefficients {
        for &b in &coefficients {
            let count = count_consecutive_primes(a, b, &sieve);
            if count > max_count {
                max_count = count;
                max_prod = a * b;
                // println!("### {} {} {}", a, b, count);
            }
        }
    }

    max_prod
}

fn count_consecutive_primes(a: isize, b: isize, sieve: &Sieve) -> usize {
    (0..)
        .map(|n| (n * n) + (a * n) + b)
        .take_while(|x| (*x > 1) && sieve.is_prime(*x as usize))
        .count()
}

#[test]
fn example_count_forty() {
    let sieve = primal::Sieve::new(2_000);
    assert_eq!(40, count_consecutive_primes(1, 41, &sieve));
}

#[test]
fn example_count_eighty() {
    let sieve = primal::Sieve::new(2_000);
    assert_eq!(80, count_consecutive_primes(-79, 1601, &sieve));
}

#[test]
fn problem() {
    assert_eq!(-59231, compute(1000));
}

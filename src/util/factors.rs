// https://codereview.stackexchange.com/a/120646
// https://en.wikipedia.org/wiki/Divisor_function
// this is stupid fast
pub fn count_divisors(num: u64) -> u32 {
    let mut num_divisors = 1;
    let mut factor = 2; // Candidate for prime factor of `n`
    let mut n = num;

    // If `n` is not a prime number then it must have one factor
    // which is <= `sqrt(n)`, so we try these first:
    while factor * factor <= n {
        if n % factor == 0 {
            // `factor` is a prime factor of `n`, determine the exponent:
            let mut exponent = 0;
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

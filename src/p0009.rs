// Special Pythagorean triplet
//
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2.
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn compute(n: u32) -> u32 {
    for a in 1.. {
        for b in a..n / 2 {
            let c = ((a * a + b * b) as f64).sqrt();
            let cc = c as u32;
            if c != cc as f64 {
                // skip when `c` isn't a whole number
                continue;
            }

            let sum = a + b + cc;
            if sum == n {
                return a * b * cc;
            }
        }
    }
    0
}

#[test]
fn check_one() {
    // (33, 56, 65)
    // 33 + 56 + 65 = 154
    // 33 * 56 * 65 = 120120
    assert_eq!(120120, compute(154));
}

#[test]
fn problem() {
    assert_eq!(31875000, compute(1000));
}

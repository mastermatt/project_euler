// Smallest multiple
//
// 2520 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//
// wink wink: Least Common Multiple

fn compute(bound: u32) -> u32 {
    (1..=bound).fold(1, lcm)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    a * (b / gcd(a, b))
}

#[test]
fn example() {
    assert_eq!(2520, compute(10));
}

#[test]
fn problem() {
    assert_eq!(232792560, compute(20));
}

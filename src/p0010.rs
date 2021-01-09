// Summation of primes
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

// this works, but it takes almost 8 secs
// fn compute(n: usize) -> usize {
//     (2..n).filter(|i| is_prime(i)).sum()
// }
//
// fn is_prime(n: &usize) -> bool {
//     let s = (*n as f64).sqrt() as usize;
//     (2..=s).all(|i| n % i != 0)
// }

// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
// ~450ms
// a segmented sieve would use less memory but wouldn't necessarily be faster,
// I might come back to that idea later.
fn compute(n: usize) -> usize {
    let mut result = 0;
    // let bound = ((n as f64).sqrt() as u64) + 1;
    let mut mark = vec![false; (n + 1) as usize];
    // (2..n).filter(|i| is_prime(i)).sum()

    for i in 2..n {
        if mark[i] == false {
            result += i;
            // println!("{}", i);

            for j in (i..n).step_by(i) {
                mark[j] = true;
            }
        }
    }

    result
}

#[test]
fn example() {
    assert_eq!(17, compute(10));
}

#[test]
fn problem() {
    assert_eq!(142913828922, compute(2_000_000));
}

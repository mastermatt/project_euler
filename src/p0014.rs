// Longest Collatz sequence
//
// The following iterative sequence is defined for the set of positive integers:
//
// n → n/2 (when n is even)
// n → 3n + 1 (when n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence:
//
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

fn compute() -> u32 {
    let mut max_num = 0;
    let mut max_len = 0;

    for i in 2..1_000_000 {
        let len = get_chain_len(i);
        if len > max_len {
            max_num = i;
            max_len = len;
        }
    }

    max_num
}

fn get_chain_len(num: u32) -> u32 {
    let mut n = num as u64;
    let mut c = 1;

    while n != 1 {
        c += 1;
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }

    return c;
}

#[test]
fn example() {
    assert_eq!(10, get_chain_len(13));
}

#[test]
fn problem() {
    assert_eq!(837799, compute());
}

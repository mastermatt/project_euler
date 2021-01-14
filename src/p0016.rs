// Power digit sum
//
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 2^1000?
//
// --------------
//
// One could just use an OSS lib that handled arbitrarily large numbers, but what fun is that?

fn compute(pow: usize) -> u32 {
    // not sure if there is anything special with 1/3 the pow, but it's a little more than needed
    let mut arr = [0u8; 333];
    arr[0] = 2;

    for _ in 1..pow {
        let mut carry = 0;
        for num in arr.iter_mut() {
            let x = (*num * 2) + carry;
            *num = x % 10;
            carry = x / 10;
        }
    }

    arr.iter().map(|&n| n as u32).sum()
}

#[test]
fn example() {
    assert_eq!(26, compute(15));
}

#[test]
fn problem() {
    assert_eq!(1366, compute(1000));
}

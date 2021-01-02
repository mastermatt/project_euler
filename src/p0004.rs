// Largest palindrome product
//
// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

const TEN: u32 = 10;

fn compute(num: u32) -> u32 {
    let mut largest = 0;
    let min = TEN.pow(num - 1);
    let max = TEN.pow(num);

    for a in min..max {
        for b in min..max {
            let prod = a * b;
            if prod > largest && is_palindrome(prod) {
                largest = prod
            }
        }
    }

    largest
}

fn is_palindrome(num: u32) -> bool {
    let str = num.to_string();
    let rev = str.chars().rev().collect::<String>();
    str == rev
}

#[test]
fn example() {
    assert_eq!(9009, compute(2));
}

#[test]
fn problem() {
    assert_eq!(906609, compute(3));
}

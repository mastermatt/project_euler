// Lexicographic permutations
//
// A permutation is an ordered arrangement of objects.
// For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
// If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
// The lexicographic permutations of 0, 1 and 2 are:
//
// 012   021   102   120   201   210
//
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
//
// ----------------------------------
//
// https://medium.com/@aiswaryamathur/find-the-n-th-permutation-of-an-ordered-string-using-factorial-number-system-9c81e34ab0c8
// https://en.wikipedia.org/wiki/Factorial_number_system

fn compute(collection: &str, nth: usize) -> String {
    let mut chars: Vec<char> = collection.chars().collect();
    let mut permutation = Vec::new();

    for &i in factoradic(nth).iter() {
        permutation.push(chars.remove(i));
    }

    permutation.into_iter().collect()
}

fn factoradic(decimal: usize) -> Vec<usize> {
    let mut quotient = decimal;
    let mut coefficients = vec![0];

    for i in 2.. {
        coefficients.push(quotient % i);
        quotient /= i;
        if quotient == 0 {
            break;
        }
    }

    coefficients.reverse();
    coefficients
}

#[test]
fn sample_factoradic() {
    assert_eq!(vec![2, 4, 2, 0, 1, 0], factoradic(349));
}

#[test]
fn sample_factoradic_large() {
    assert_eq!(vec![2, 6, 6, 2, 5, 1, 2, 2, 0, 0], factoradic(1_000_000));
}

#[test]
fn example_abcd() {
    assert_eq!("cbad", compute("abcd", 14));
}

#[test]
fn problem() {
    assert_eq!("2783915460", compute("0123456789", 1_000_000 - 1));
}

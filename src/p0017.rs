// Number letter counts
//
// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
// how many letters would be used?
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two)
// contains 23 letters and 115 (one hundred and fifteen) contains 20 letters.
// The use of "and" when writing out numbers is in compliance with British usage.

const LOWER: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn num_to_str(num: usize) -> String {
    let mut v = vec![];
    let thousands = num / 1000;
    let hundreds = (num % 1000) / 100;
    let lower = num % 100;

    if thousands != 0 {
        v.push(LOWER[thousands]);
        v.push("thousand");
    }

    if hundreds != 0 {
        v.push(LOWER[hundreds]);
        v.push("hundred");

        if lower != 0 {
            v.push("and");
        }
    }

    if lower >= 20 {
        v.push(TENS[lower / 10]);
        v.push(LOWER[lower % 10]);
    }

    if lower > 0 && lower < 20 {
        v.push(LOWER[lower]);
    }

    v.join(" ")
}

fn compute(n: usize) -> usize {
    let s = (1..=n).map(num_to_str).collect::<Vec<String>>().join(" ");

    // println!("{}", s);
    s.replace(" ", "").len()
}

#[test]
fn example() {
    assert_eq!(19, compute(5));
}

#[test]
fn problem() {
    assert_eq!(21124, compute(1000));
}

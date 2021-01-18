// Names scores
//
// Using names.txt, a 46K text file containing over five-thousand first names,
// begin by sorting it into alphabetical order. Then working out the alphabetical value for
// each name, multiply this value by its alphabetical position in the list to obtain a name score.
//
// For example, when the list is sorted into alphabetical order, COLIN,
// which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
// So, COLIN would obtain a score of 938 × 53 = 49714.
//
// What is the total of all the name scores in the file?

use std::fs;

fn compute(mut list: Vec<String>) -> usize {
    list.sort_unstable();
    let mut result = 0;

    for (i, name) in list.iter().enumerate() {
        result += (i + 1) * alphabetical_value(name)
    }

    result
}

fn alphabetical_value(name: &str) -> usize {
    name.chars().map(|c| (c as usize) - 64).sum()
}

fn parse_input(input: String) -> Vec<String> {
    input
        .trim()
        .replace('"', "")
        .split(",")
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn example_colin() {
    assert_eq!(53, alphabetical_value("COLIN"));
}

#[test]
fn problem() {
    let input = fs::read_to_string("./resources/p0022_names.txt").expect("Unable to read file");

    assert_eq!(871198282, compute(parse_input(input)));
}

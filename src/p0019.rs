// Counting Sundays
//
// You are given the following information, but you may prefer to do some research for yourself.
//
// - 1 Jan 1900 was a Monday.
//
// - Thirty days has September,
//   April, June and November.
//   All the rest have thirty-one,
//   Saving February alone,
//   Which has twenty-eight, rain or shine.
//   And on leap years, twenty-nine.
//
// - A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
//
// ----------------------------
//
// Ok, let's think about this.
// If Jan 1 is considered to start on the 0 dow (day of week) index then on non-leap year years
// the dow index of first of each month is: 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5 and the next year starts on 1.
//       For leap years the index would be: 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6 and the next year starts on 2.
//
// So if there year starts on a Monday, there are two Sunday first of the month regardless of leap year, hmmmm interesting.
//
// At this point, I figured I could code the answer faster than thinking about it. I was right.

const MONTH: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const MONTH_L: [u16; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn compute() -> u16 {
    let mut count = 0;
    // day of week. Monday: 0 .. Sunday: 6
    // 1 Jan 1901 was a Tuesday
    let mut dow = 1;

    for year in 1901..=2000 {
        let month_lens = if is_leap_year(year) { MONTH_L } else { MONTH };

        for mo in month_lens.iter() {
            if dow == 6 {
                // a Sunday!
                count += 1;
            }

            dow = (dow + mo) % 7;
        }
    }

    count
}

// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
// https://stackoverflow.com/a/11595914/5535820
fn is_leap_year(year: u16) -> bool {
    (year & 3) == 0 && ((year % 25) != 0 || (year & 15) == 0)
}

#[test]
fn problem() {
    assert_eq!(171, compute());
}

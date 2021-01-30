// Reciprocal cycles
//
// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
//
// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
//
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

fn compute(limit: usize) -> usize {
    let mut max_d = 0;
    let mut max_l = 0;

    // we only really need to check prime nums, but we're doing all 1K in less than a ms so... eh.
    for i in 2..limit {
        let l = cycle_len(i);

        if l > max_l {
            max_l = l;
            max_d = i;
        }
    }

    max_d
}

// https://mathworld.wolfram.com/DecimalExpansion.html
fn cycle_len(num: usize) -> usize {
    if num % 2 == 0 || num % 5 == 0 {
        return 0;
    }

    let mut a = 1;
    let mut t = 0;
    loop {
        a = (a * 10) % num;
        t += 1;
        if a == 1 {
            break;
        }
    }
    t
}

#[test]
fn example() {
    assert_eq!(7, compute(10));
}

#[test]
fn problem() {
    assert_eq!(983, compute(1000));
}

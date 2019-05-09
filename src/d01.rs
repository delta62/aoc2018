use std::collections::HashSet;

pub fn freq(frequencies: &str) -> i32 {
    let ops = frequencies.lines();
    ops.fold(0, |acc, x| {
        acc + x.parse::<i32>().unwrap()
    })
}

pub fn dup(frequencies: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut current = 0;
    frequencies
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .cycle()
        .find(|x| {
            if seen.contains(&current) {
                true
            } else {
                seen.insert(current);
                current += x;
                false
            }
        });
    current
}

#[test]
fn freq_ex1() {
    let res = freq("+1\n+1\n+1");
    assert_eq!(res, 3);
}

#[test]
fn freq_ex2() {
    let res = freq("+1\n+1\n-2");
    assert_eq!(res, 0);
}

#[test]
fn freq_ex3() {
    let res = freq("-1\n-2\n-3");
    assert_eq!(res, -6);
}

#[test]
fn dup_ex1() {
    let res = dup("+1\n-1");
    assert_eq!(res, 0);
}

// #[test]
// fn dup_ex2() {
//     let res = dup("+3\n+3\n+4\n-2\n-4");
//     assert_eq!(res, 10);
// }
//
// #[test]
// fn dup_ex3() {
//     let res = dup("-6\n+3\n+8\n+5\n-6");
//     assert_eq!(res, 5);
// }
//
// #[test]
// fn dup_ex4() {
//     let res = dup("+7\n+7\n-2\n-7\n-4");
//     assert_eq!(res, 14);
// }

#![allow(dead_code)]

fn d01_freq(frequencies: &str) -> i32 {
    let ops = frequencies.split(", ");
    ops.fold(0, |acc, x| {
        acc + x.parse::<i32>().unwrap()
    })
}

#[test]
fn ex1() {
    let res = d01_freq("+1, +1, +1");
    assert_eq!(res, 3);
}

#[test]
fn ex2() {
    let res = d01_freq("+1, +1, -2");
    assert_eq!(res, 0);
}

#[test]
fn ex3() {
    let res = d01_freq("-1, -2, -3");
    assert_eq!(res, -6);
}

// #[test]
// fn blah() {
//     let res = d01_freq("");
//     println!("{}", res);
// }

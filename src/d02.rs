use std::collections::HashMap;

pub fn checksum(input: &str) -> u32 {
    let (d, t) = input
        .lines()
        .map(|x| sum_line(x))
        .fold((0, 0), |(d, t), x| {
            match x {
                (false, false) => (d, t),
                (false, true)  => (d, t + 1),
                (true, false)  => (d + 1, t),
                (true, true)   => (d + 1, t + 1),
            }
        });
    d * t
}

fn sum_line(input: &str) -> (bool, bool) {
    let mut map = HashMap::new();
    for c in input.chars() {
        match map.get(&c) {
            Some(v) => map.insert(c, v + 1),
            None    => map.insert(c, 1),
        };
    }
    let mut double = false;
    let mut triple = false;
    for (_, v) in map {
        if v == 2 {
            double = true;
        } else if v == 3 {
            triple = true;
        }
    }
    (double, triple)
}

#[test]
fn checksum_ex1() {
    let lines = vec! [
        "abcdef",
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab",
    ];
    let input = lines.join("\n");
    let res = checksum(&input);
    assert_eq!(res, 12);
}

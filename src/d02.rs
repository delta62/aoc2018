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

pub fn char_seq(input: &str) -> Option<String> {
    let mut seen: Vec<&str> = Vec::new();
    let mut ret: Option<String> = None;
    input.lines().for_each(|x| {
        let s = seen.iter().find(|s| is_seq(s, x));
        match s {
            Some(s) => {
                ret = Some(common(s, x));
                return
            },
            None => seen.push(x),
        }
    });
    None
}

fn is_seq(a: &str, b: &str) -> bool {
    let mut diff = 0;
    for (c1, c2) in a.chars().zip(b.chars()) {
        let d = i64::from(u32::from(c1)) - i64::from(u32::from(c2));
        diff += d.abs();
    }
    diff == 1
}

fn common(a: &str, b: &str) -> String {
    let char_pairs = a.chars().zip(b.chars());
    let v = char_pairs.fold(Vec::new(), |mut acc: Vec<char>, (x, y)| {
        if x == y {
            acc.push(x);
        }
        acc
    });
    v.iter().collect()
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

#[test]
fn char_seq_ex1() {
    let lines = vec! [
        "abcde",
        "fghij",
        "klmno",
        "pqrst",
        "fguij",
        "axcye",
        "wvxyz",
    ];
    let input = lines.join("\n");
    let res = char_seq(&input);
    assert_eq!(res, Some("fgij".to_string()));
}

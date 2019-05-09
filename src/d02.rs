use std::collections::HashMap;
use std::iter::Zip;
use std::str::Chars;

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
    for x in input.lines() {
        let s = seen.iter().find(|s| is_seq(s, x));
        match s {
            Some(s) => {
                return Some(common(s, x));
            },
            None => seen.push(x),
        }
    };
    None
}

fn is_seq(a: &str, b: &str) -> bool {
    let mut diff = 0;
    for (c1, c2) in char_pairs(a, b) {
        if c1 != c2 {
            diff += 1
        }
    }
    diff == 1
}

fn common(a: &str, b: &str) -> String {
    let v = char_pairs(a, b).fold(Vec::new(), |mut acc: Vec<char>, (x, y)| {
        if x == y {
            acc.push(x);
        }
        acc
    });
    v.iter().collect()
}

fn char_pairs<'a>(a: &'a str, b: &'a str) -> Zip<Chars<'a>, Chars<'a>> {
    a.chars().zip(b.chars())
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

extern crate regex;

use regex::Regex;
use std::str::FromStr;

struct Area {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

struct ParseError { }

impl FromStr for Area {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#^(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let x = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                let y = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                let width = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let height = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();
                Ok(Area { id, x, y, width, height })
            },
            None => Err(ParseError { }),
        }
    }
}

pub fn overlap_count(input: &str) -> u32 {
    let areas = input.lines().map(|x| x.parse::<Area>());
    42
}

use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead, Error};
use std::fs::File;

fn read_input() -> Result<Vec<String>, Error> {
    let file = File::open("input/day02.txt").unwrap();
    BufReader::new(file).lines().collect::<_>()
}

fn count(s: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for char in s.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }
    counts
}

fn solve1(lines: &Vec<String>) -> u32 {
    let mut count2 = 0;
    let mut count3 = 0;
    for line in lines {
        let mut has_2 = false;
        let mut has_3 = false;
        for count in count(&line).values() {
            match count {
                2 => has_2 = true,
                3 => has_3 = true,
                _ => ()
            }
        }
        count2 += has_2 as u32;
        count3 += has_3 as u32;
    }
    count2 * count3
}

fn solve2(lines: &Vec<String>) -> String {
    let mut seen = HashSet::new();
    for line in lines {
        let mut char_vec = line.chars().collect::<Vec<char>>();
        for i in 0..char_vec.len() {
            let one_missing = char_vec[..i].iter().chain(char_vec[i + 1..].iter()).collect::<String>();
            if seen.contains(&(i, one_missing.clone())) {
                return one_missing;
            }
            seen.insert((i, one_missing));
        }
    }
    "".to_string()
}

pub fn solve() {
    let lines = read_input().unwrap();

    println!("Checksum: {}", solve1(&lines));
    println!("Common letters: {}", solve2(&lines));
}

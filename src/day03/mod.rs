extern crate regex;

use std::io::{BufReader, BufRead};
use std::fs::File;
use self::regex::Regex;

struct Claim {
    id: i32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}


fn read_input() -> Vec<Claim> {
    let file = File::open("input/day03.txt").unwrap();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut result = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();
        result.push(
            Claim{id: captures[1].parse().unwrap(),
                  x: captures[2].parse().unwrap(),
                  y: captures[3].parse().unwrap(),
                  w: captures[4].parse().unwrap(),
                  h: captures[5].parse().unwrap(),
            });
    }
    result
}

pub fn solve() {
    let mut fabric = Vec::<Vec<u32>>::new();
    for _ in 0..1000 {
        fabric.push(vec![0; 1000]);
    }
    let claims = read_input();
    for claim in &claims {
        for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {
                fabric[x][y] += 1;
            }
        }
    }
    let sum: u32 = fabric.iter().map(|row| row.iter().fold(0, |count, val| count + (*val >= 2) as u32)).sum();
    println!("Contested square inches: {}", sum);

    let mut nonoverlapping = None;
    for claim in &claims {
        let mut valid = true;
        'outer: for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {
                if fabric[x][y] != 1 {
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            match nonoverlapping {
                Some(id) => panic!("Two valid claims: {} {}", id, claim.id),
                None => nonoverlapping = Some(claim.id)
            };
        }
    }
    println!("Remaining claim: {}", nonoverlapping.unwrap());
}

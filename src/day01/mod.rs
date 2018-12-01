use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_input() -> Vec<i32> {
    let file = File::open("input/day01.txt").unwrap();

    let mut ints = vec![];
    for line in BufReader::new(file).lines() {
        ints.push(line.unwrap().parse::<i32>().unwrap());
    }
    ints
}

pub fn solve() {
    let mut frequency = 0;
    let mut visited = HashSet::new();
    visited.insert(frequency);
    let ints = read_input();
    println!("Final frequency: {}", ints.iter().sum::<i32>());
    loop {
        for delta in ints.iter() {
            frequency += delta;
            if visited.contains(&frequency) {
                println!("Duplicate frequency: {}", frequency);
                return
            }
            visited.insert(frequency);
        }
    }
}

use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;
use chrono::{NaiveDateTime, Timelike};

enum GuardEvent {
    Guard(u32),
    FallsAsleep,
    WakesUp
}

struct TimedEvent {
    timestamp: NaiveDateTime,
    event: GuardEvent
}

impl TimedEvent {
    pub fn new(s: &str) -> TimedEvent {
        lazy_static! {
            static ref regex_guard: Regex = Regex::new(
                r"\[([^\]]+)\] Guard #(\d+) begins shift").unwrap();
            static ref regex_falls_asleep: Regex = Regex::new(
                r"\[([^\]]+)\] falls asleep").unwrap();
            static ref regex_wakes_up: Regex = Regex::new(
                r"\[([^\]]+)\] wakes up").unwrap();
        }
        match regex_guard.captures(s) {
            Some(captures) => return TimedEvent{
                timestamp: NaiveDateTime::parse_from_str(&captures[1], "%Y-%m-%d %H:%M").unwrap(),
                event: GuardEvent::Guard(captures[2].parse().unwrap())
            },
            None => ()
        };
        match regex_falls_asleep.captures(s) {
            Some(captures) => return TimedEvent{
                timestamp: NaiveDateTime::parse_from_str(&captures[1], "%Y-%m-%d %H:%M").unwrap(),
                event: GuardEvent::FallsAsleep
            },
            None => ()
        };
        match regex_wakes_up.captures(s) {
            Some(captures) => return TimedEvent{
                timestamp: NaiveDateTime::parse_from_str(&captures[1], "%Y-%m-%d %H:%M").unwrap(),
                event: GuardEvent::WakesUp
            },
            None => ()
        };
        panic!();
    }
}

fn read_input() -> Vec<TimedEvent> {
    let file = File::open("input/day04.txt").unwrap();
    let mut lines = BufReader::new(file).lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    lines.sort();
    lines.iter().map(|line| TimedEvent::new(line)).collect::<_>()
}


pub fn solve() {
    let events = read_input();
    let mut active_guard = None;
    let mut start = None;
    let mut guard_minutes = HashMap::new();
    for event in &events {
        match event.event {
            GuardEvent::Guard(id) => active_guard = Some(id),
            GuardEvent::FallsAsleep => start = Some(event.timestamp),
            GuardEvent::WakesUp => {
                let guard_id = active_guard.unwrap();
                let mut minute_counter = guard_minutes.entry(guard_id)
                    .or_insert(vec![0; 60]);
                for i in start.unwrap().minute()..event.timestamp.minute() {
                    minute_counter[i as usize] += 1;
                }
            }
        }
    }
    let (guard_id, minute_counts) = guard_minutes.iter().max_by_key(|x| x.1.iter().sum::<i32>()).unwrap();
    let max_minute = minute_counts.iter().enumerate().max_by_key(|x| x.1).unwrap().0 as u32;
    println!("Strategy 2: {}", *guard_id as u32 * max_minute);

    let mut max_guard = 0;
    let mut max_minute = 0;
    let mut max_count = 0;
    for (guard_id, minute_counts) in &guard_minutes {
        let (guard_max_minute, guard_max_count) = minute_counts.iter().enumerate().max_by_key(|x| x.1).unwrap();
        if guard_max_count > &max_count {
            max_guard = *guard_id;
            max_minute = guard_max_minute;
            max_count = *guard_max_count;
        }

    }
    println!("Strategy 2: {}", max_guard * max_minute as u32);
}

#[macro_use]
extern crate clap;
extern crate advent2018;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code 2018")
        .arg(Arg::with_name("day")
             .short("d")
             .long("day")
             .takes_value(true))
        .get_matches();

    let day = value_t!(matches.value_of("day"), i32).unwrap_or(-1);
    match day {
        1 => advent2018::day01::solve(),
        2 => advent2018::day02::solve(),
        3 => advent2018::day03::solve(),
        4 | -1 => advent2018::day04::solve(),
        _ => {
            println!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
}

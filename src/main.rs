pub mod common;

mod day1;
mod day2;
mod day3;

use std::{borrow::Cow, env, fs};

use common::Day;

fn run_day<'a, D: Day<'a>>(part1: bool, part2: bool, content: &'a str) {
    let parsed = D::parse(content);

    if part1 {
        let part1_result = D::part1(&parsed);
        println!("Part 1: {:?}", part1_result);
    }

    if part2 {
        let part2_result = D::part2(&parsed);
        println!("Part 2: {:?}", part2_result);
    }
}

fn file(day: u16) -> Cow<'static, str> {
    Cow::from(format!("input/day{}.txt", day))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u16 = args.get(1).and_then(|x| x.parse::<u16>().ok()).unwrap_or(0);
    let part: u16 = args.get(2).and_then(|x| x.parse::<u16>().ok()).unwrap_or(3);
    let file = args.get(3).map(Cow::from).unwrap_or_else(|| file(day));

    let content = fs::read_to_string(file.as_ref()).expect("Error while reading input file");
    let part1 = part == 1 || part == 3;
    let part2 = part == 2 || part == 3;

    match day {
        1 => run_day::<day1::Day1>(part1, part2, &content),
        2 => run_day::<day2::Day2>(part1, part2, &content),
        3 => run_day::<day3::Day3>(part1, part2, &content),
        _ => {}
    }
}

mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use std::{collections::HashMap, env};

fn main() {
    let mut mods: HashMap<&str, Box<dyn Fn() -> String>> = HashMap::new();
    mods.insert("day1part1", Box::new(day01::part1::run));
    mods.insert("day1part2", Box::new(day01::part2::run));
    mods.insert("day2part1", Box::new(day02::part1::run));
    mods.insert("day2part2", Box::new(day02::part2::run));
    mods.insert("day3part1", Box::new(day03::part1::run));
    mods.insert("day3part2", Box::new(day03::part2::run));
    mods.insert("day4part1", Box::new(day04::part1::run));
    mods.insert("day4part2", Box::new(day04::part2::run));
    mods.insert("day5part1", Box::new(day05::part1::run));
    mods.insert("day5part2", Box::new(day05::part2::run));
    mods.insert("day6part1", Box::new(day06::part1::run));
    mods.insert("day6part2", Box::new(day06::part2::run));
    mods.insert("day7part1", Box::new(day07::part1::run));
    mods.insert("day7part2", Box::new(day07::part2::run));
    mods.insert("day8part1", Box::new(day08::part1::run));
    mods.insert("day8part2", Box::new(day08::part2::run));
    mods.insert("day9part1", Box::new(day09::part1::run));
    mods.insert("day9part2", Box::new(day09::part2::run));
    mods.insert("day10part1", Box::new(day10::part1::run));
    mods.insert("day10part2", Box::new(day10::part2::run));
    mods.insert("day11part1", Box::new(day11::part1::run));
    mods.insert("day11part2", Box::new(day11::part2::run));

    let args: Vec<String> = env::args().collect();
    println!("{}", mods[args[1].as_str()]());
}

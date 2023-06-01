mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::{collections::HashMap, env};

fn main() {
    let mut mods: HashMap<&str, Box<dyn Fn() -> String>> = HashMap::new();
    mods.insert("day1part1", Box::new(day1::part1::run));
    mods.insert("day1part2", Box::new(day1::part2::run));
    mods.insert("day2part1", Box::new(day2::part1::run));
    mods.insert("day2part2", Box::new(day2::part2::run));
    mods.insert("day3part1", Box::new(day3::part1::run));
    mods.insert("day3part2", Box::new(day3::part2::run));
    mods.insert("day4part1", Box::new(day4::part1::run));
    mods.insert("day4part2", Box::new(day4::part2::run));
    mods.insert("day5part1", Box::new(day5::part1::run));
    mods.insert("day5part2", Box::new(day5::part2::run));
    mods.insert("day6part1", Box::new(day6::part1::run));
    mods.insert("day6part2", Box::new(day6::part2::run));
    mods.insert("day7part1", Box::new(day7::part1::run));
    mods.insert("day7part2", Box::new(day7::part2::run));

    let args: Vec<String> = env::args().collect();
    println!("{}", mods[args[1].as_str()]());
}

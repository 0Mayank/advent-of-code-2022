mod day1;
mod day2;
mod day3;

use std::{collections::HashMap, env, println};

fn main() {
    let mut mods: HashMap<&str, Box<dyn Fn() -> i32>> = HashMap::new();
    mods.insert("day1part1", Box::new(day1::part1::run));
    mods.insert("day1part2", Box::new(day1::part2::run));
    mods.insert("day2part1", Box::new(day2::part1::run));
    mods.insert("day2part2", Box::new(day2::part2::run));
    mods.insert("day3part1", Box::new(day3::part1::run));
    mods.insert("day3part2", Box::new(day3::part2::run));
    //mods.insert("day4part1", Box::new(day4::part1::run));

    let args: Vec<String> = env::args().collect();
    println!("{}", mods[args[1].as_str()]());
}

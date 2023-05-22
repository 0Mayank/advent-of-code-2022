mod day1;
mod day2;

use std::{collections::HashMap, env, println};

fn main() {
    let mods = HashMap::from([
        ("day1part1", day1::part1::run()),
        ("day1part2", day1::part2::run()),
        ("day2part1", day2::part1::run()),
        ("day2part2", day2::part2::run()),
    ]);

    let args: Vec<String> = env::args().collect();
    println!("{}", mods[args[1].as_str()]);
}

use advent_of_code_2023::*;

use std::env::args;

fn main() -> std::io::Result<()> {
    let args = args();

    match args.skip(1).collect::<String>().as_str() {
        "" => println!("Usage: cargo run [--release] [day<n>]"),
        "day1" => {
            day_01_part1();
            day_01_part2();
        }
        "day2" => {
            day_02_part1();
            day_02_part2();
        }
        "day3" => {
            day_03_part1();
        }
        _ => {}
    }

    Ok(())
}

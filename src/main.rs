use advent_of_code_2023::*;

use std::env::args;

fn main() -> std::io::Result<()> {
    let args = args();

    match args.skip(1).collect::<String>().as_str() {
        "day1" => {
            day_01_part1();
            day_01_part2();
        }
        "day2" => {
            day_02_part1();
        }
        _ => {}
    }

    Ok(())
}

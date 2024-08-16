static INPUT: &str = include_str!("input.txt");

fn parse_part_01(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let token = l.chars();
            let num = token.filter(|c| c.is_numeric()).collect::<Vec<_>>();

            let first = num.first().unwrap();
            let last = num.last().unwrap();

            let res = format!("{}{}", first, last);
            res.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn day_01_part1() {
    let start = std::time::Instant::now();
    let nums = parse_part_01(INPUT);

    let res = nums.iter().sum::<usize>();

    println!(
        "day 01 part 1: {}, time to process: {:?}",
        res,
        start.elapsed()
    )
}

fn parse_part_02(input: &str) -> Vec<usize> {
    #[rustfmt::skip]
    let keywords = [
        ("1",     1),
        ("2",     2),
        ("3",     3),
        ("4",     4),
        ("5",     5),
        ("6",     6),
        ("7",     7),
        ("8",     8),
        ("9",     9),
        ("one",   1),
        ("two",   2),
        ("three", 3),
        ("four",  4),
        ("five",  5),
        ("six",   6),
        ("seven", 7),
        ("eight", 8),
        ("nine",  9),
    ];

    input
        .lines()
        .map(|line| {
            let mut digits: Vec<Option<usize>> = vec![None; line.len()];
            for i in 0..line.len() {
                for (s, val) in keywords.iter() {
                    if line[i..].starts_with(s) {
                        digits[i] = Some(*val);
                        break;
                    }
                }
            }

            let token = digits.iter().flatten().cloned().collect::<Vec<_>>();

            let first = token.first().unwrap() * 10;
            let last = token.last().unwrap();

            first + last
        })
        .collect()
}

pub fn day_01_part2() {
    let start = std::time::Instant::now();
    let parsed_input = parse_part_02(INPUT);

    let res = parsed_input.iter().sum::<usize>(); // 54980

    println!(
        "day 01 part 2: {:?}, time to process: {:?}",
        res,
        start.elapsed()
    );
}

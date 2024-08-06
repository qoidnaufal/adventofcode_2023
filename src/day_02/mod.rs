// #![allow(dead_code, unused_variables)]

use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
const LIMIT: [Cube; 3] = [Cube::Red(12), Cube::Green(13), Cube::Blue(14)];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let comp = |n, m| {
            if n > m {
                Some(std::cmp::Ordering::Greater)
            } else if n < m {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        };

        match (self, other) {
            (Self::Red(n), Self::Red(m)) => comp(n, m),
            (Self::Blue(n), Self::Blue(m)) => comp(n, m),
            (Self::Green(n), Self::Green(m)) => comp(n, m),
            _ => None,
        }
    }
}

impl FromStr for Cube {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut token = s.split_whitespace();
        let num = token.next().unwrap().parse::<usize>().unwrap();
        let color = token.next().unwrap();

        match color {
            "red" => Ok(Self::Red(num)),
            "blue" => Ok(Self::Blue(num)),
            "green" => Ok(Self::Green(num)),
            _ => Err(()),
        }
    }
}

fn convert_into_set(input: &str) -> Vec<Cube> {
    let input = input.trim();

    if !input.contains(',') {
        let set = input.parse::<Cube>().unwrap();
        vec![set]
    } else {
        input
            .split(',')
            .map(|s| s.parse::<Cube>().unwrap())
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Game {
    game_id: usize,
    sets: Vec<Vec<Cube>>,
}

fn parse_input1(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut token = line.split([':', ';']);

            let game_id = token
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let mut sets = Vec::new();

            while let Some(set) = token.next() {
                let set = convert_into_set(set);
                sets.push(set)
            }

            Game { game_id, sets }
        })
        .collect()
}

fn find_game(games: Vec<Game>) -> Vec<usize> {
    games
        .iter()
        .filter(|g| {
            let sets = g.sets.as_slice();

            sets.iter().all(|set| {
                set.iter().all(|cube| {
                    LIMIT
                        .iter()
                        .any(|n| cube.partial_cmp(n).is_some_and(|r| !r.is_gt()))
                })
            })
        })
        .map(|g| {
            println!("{:?}", g);
            g.game_id
        })
        .collect::<Vec<_>>()
}

pub fn day_02_part1() {
    let start = std::time::Instant::now();

    let _test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let games = parse_input1(INPUT);

    let res = find_game(games).iter().sum::<usize>();

    println!(
        "day 02 part 1: {:?}, time to process: {:?}",
        res,
        start.elapsed()
    )
}

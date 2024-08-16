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
        let comp = |n: &usize, m: &usize| match n.cmp(m) {
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
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

impl Game {
    fn find_max_cubes(&self) -> usize {
        let mut red = Vec::new();
        let mut green = Vec::new();
        let mut blue = Vec::new();

        self.sets.iter().for_each(|set| {
            set.iter().for_each(|cube| match *cube {
                Cube::Red(r) => red.push(r),
                Cube::Green(g) => green.push(g),
                Cube::Blue(b) => blue.push(b),
            });
        });

        let max_red = red.iter().max().unwrap();
        let max_green = green.iter().max().unwrap();
        let max_blue = blue.iter().max().unwrap();

        max_red * max_green * max_blue
    }
}

fn parse_input(input: &str) -> Vec<Game> {
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

            for set in token {
                let set = convert_into_set(set);
                sets.push(set)
            }

            Game { game_id, sets }
        })
        .collect()
}

fn find_games(games: Vec<Game>) -> Vec<usize> {
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
        .map(|g| g.game_id)
        .collect::<Vec<_>>()
}

fn calculate_max_cubes(games: Vec<Game>) -> usize {
    games.iter().map(|game| game.find_max_cubes()).sum()
}

pub fn day_02_part1() {
    let start = std::time::Instant::now();
    let games = parse_input(INPUT);
    let res = find_games(games).iter().sum::<usize>();

    println!(
        "day 02 part 1: {:?}, time to process: {:?}",
        res,
        start.elapsed()
    )
}

pub fn day_02_part2() {
    let start = std::time::Instant::now();
    let games = parse_input(INPUT);

    let res = calculate_max_cubes(games);

    // attempt 1: answer (2699) too low --> i forgot in game.find_max_cubes I didnt power the result, instead I added them

    println!(
        "day 02 part 2: {:?}, time to process: {:?}",
        res,
        start.elapsed()
    )
}

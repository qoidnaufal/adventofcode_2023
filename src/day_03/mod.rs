// #![allow(dead_code, unused_variables, unused_mut)]

static INPUT: &str = include_str!("input.txt");

fn symbol_index(input: &str) -> Vec<Vec<usize>> {
    let lines = input.lines();

    let mut sym_idx = Vec::new();

    for line in lines {
        let token = line.chars().collect::<Vec<_>>();
        let mut id = Vec::new();
        for i in 0..token.len() {
            if token
                .get(i)
                .is_some_and(|c: &char| !c.is_numeric() && *c != '.')
            {
                id.push(i);
            }
        }
        sym_idx.push(id);
    }

    sym_idx
}

fn parse_input(input: &str) -> Vec<usize> {
    let symbol_index = symbol_index(input);

    let lines = input.lines().enumerate();

    let mut numbers = Vec::new();

    for (row, line) in lines {
        let mut token = line.char_indices();

        if let Some(s_idx) = symbol_index.get(row - 1) {
            if !s_idx.is_empty() {
                s_idx.iter().for_each(|n| {
                    let mut buf = String::new();
                    for (c_idx, ch) in token.by_ref() {
                        if !ch.is_numeric() {
                            if buf.is_empty() && c_idx > *n {
                                break;
                            } else if buf.is_empty() {
                                continue;
                            } else if !buf.is_empty() && c_idx > *n {
                                break;
                            } else if !buf.is_empty() && c_idx < *n {
                                buf.clear();
                                continue;
                            } else {
                                buf.push(ch);
                                continue;
                            }
                        } else {
                            buf.push(ch);
                        }
                    }

                    if !buf.is_empty() {
                        let val = buf.split(|c: char| !c.is_numeric()).collect::<Vec<_>>();
                        for v in val {
                            if let Ok(number) = v.parse::<usize>() {
                                println!("row: {row}, n: {n}, {number:?} [-1]");
                                numbers.push(number)
                            }
                        }
                    }
                })
            }
        }

        let mut token = line.char_indices();

        if let Some(s_idx) = symbol_index.get(row) {
            if !s_idx.is_empty() {
                s_idx.iter().for_each(|n| {
                    let mut buf = String::new();
                    for (c_idx, ch) in token.by_ref() {
                        if !ch.is_numeric() {
                            if buf.is_empty() && c_idx - 1 == *n {
                                break;
                            } else if buf.is_empty() {
                                continue;
                            } else if !buf.is_empty() && (c_idx > *n || c_idx == line.len() - 1) {
                                let val = buf.split(|c: char| !c.is_numeric()).collect::<Vec<_>>();
                                for v in val {
                                    if let Ok(number) = v.parse::<usize>() {
                                        println!("row: {row}, n: {n}, {number:?} [0]");
                                        numbers.push(number);
                                    }
                                }
                                break;
                            } else if !buf.is_empty() && c_idx == *n {
                                buf.push(ch);
                                continue;
                            } else {
                                buf.clear();
                            }
                        } else {
                            buf.push(ch);
                            if c_idx == line.len() - 1 {
                                let val = buf.split(|c: char| !c.is_numeric()).collect::<Vec<_>>();
                                for v in val {
                                    if let Ok(number) = v.parse::<usize>() {
                                        println!("row: {row}, n: {n}, {number:?} [0]");
                                        numbers.push(number);
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }

        let mut token = line.char_indices();

        if let Some(s_idx) = symbol_index.get(row + 1) {
            if !s_idx.is_empty() {
                s_idx.iter().for_each(|n| {
                    let mut buf = String::new();
                    for (c_idx, ch) in token.by_ref() {
                        if !ch.is_numeric() {
                            if buf.is_empty() && c_idx > *n {
                                break;
                            } else if buf.is_empty() {
                                continue;
                            } else if !buf.is_empty() && c_idx > *n {
                                break;
                            } else if !buf.is_empty() && c_idx < *n {
                                buf.clear();
                                continue;
                            } else {
                                buf.push(ch);
                                continue;
                            }
                        } else {
                            buf.push(ch);
                        }
                    }

                    if !buf.is_empty() {
                        let val = buf.split(|c: char| !c.is_numeric()).collect::<Vec<_>>();
                        for v in val {
                            if let Ok(number) = v.parse::<usize>() {
                                println!("row: {row}, n: {n}, {number:?} [+1]");
                                numbers.push(number)
                            }
                        }
                    }
                })
            }
        }
    }

    numbers
}

pub fn day_03_part1() {
    let start = std::time::Instant::now();

    //     #[rustfmt::skip]
    //     let test_input =
    // "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    let parsed_input = parse_input(INPUT);

    let res = parsed_input.iter().sum::<usize>(); // 543867

    println!(
        "day 03 part 1: {res}, time to process: {:?}",
        start.elapsed()
    )
}

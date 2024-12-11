use std::collections::HashMap;
use std::io::stdin;

pub fn part1() {
    let mut stones: Vec<i64> = Vec::new();

    stdin().lines()
        .for_each(|l| {
            let line = l.unwrap();
            stones = line.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect();
        });

    let mut map : HashMap<(i64, usize), i64> = HashMap::new();
    let mut result = 0;
    for stone in stones {
        result += parse_stone(stone, 75, &mut map)
    }

    println!("{result:?}");
}

fn parse_stone(stone: i64, iteration: usize, map: &mut HashMap<(i64, usize), i64>) -> i64 {
    if iteration == 0 {
        return 1;
    }
    if map.contains_key(&(stone, iteration)) {
        return map[&(stone, iteration)]
    }
    let new_iteration = iteration - 1;

    let size = is_even_digits(stone);
    let res = match size % 2 == 0 {
        true => {
            let new = parse_even(stone, size);
            parse_stone(new.0, new_iteration, map) + parse_stone(new.1, new_iteration, map)
        }
        false => {
            parse_stone(parse_odd(stone), new_iteration, map)
        }
    };
    
    map.insert((stone, iteration), res);
    res
}

fn parse_even(number: i64, length: usize) -> (i64, i64) {
    let a = (length / 2) as u32;
    let divider = (10i64).pow(a);
    (number / divider, number % divider)
}

fn parse_odd(number: i64) -> i64 {
    if number == 0 {
        return 1;
    }

    number * 2024
}

fn is_even_digits(number: i64) -> usize {
    number.to_string().len()
}

pub fn part2() {}

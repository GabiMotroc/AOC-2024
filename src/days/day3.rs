use std::io::stdin;
use regex::Regex;

pub fn part1() {
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    stdin().lines()
        .for_each(|line|
            {
                for (_, [first, second]) in re.captures_iter(&line.unwrap()).map(|caps| caps.extract()) {
                    let a: i32 = first.parse().unwrap();
                    let b: i32 = second.parse().unwrap();

                    result += a * b;
                }
            }
        );

    println!("{result:?}");
}

pub fn part2() {}

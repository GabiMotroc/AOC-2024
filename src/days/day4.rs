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

pub fn part2() {
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let line: String = stdin().lines().map(|l| l.unwrap().to_string())
        .collect::<Vec<String>>()
        .join("");

    let split: Vec<&str> = line.split("don't()").collect();

    println!("{split:?}");
    for (_, [first, second]) in re.captures_iter(&split[0]).map(|caps| caps.extract()) {
        let a: i32 = first.parse().unwrap();
        let b: i32 = second.parse().unwrap();

        result += a * b;
    }

    for x in split.iter().skip(1) {
        let a: Vec<&str> = x.split("do()").collect();

        println!("do split {x:?}");

        for y in a.iter().skip(1) {
            for (_, [first, second]) in re.captures_iter(y).map(|caps| caps.extract()) {
                let a: i32 = first.parse().unwrap();
                let b: i32 = second.parse().unwrap();

                result += a * b;
            }
        }
    }

    println!("{result:?}");
}

use std::io::stdin;

pub fn part1() {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut prints: Vec<Vec<i32>> = Vec::new();

    let mut switch_to_second_part = false;
    stdin().lines()
        .for_each(|l| {
            let line = l.unwrap();
            if line.is_empty() {
                switch_to_second_part = true;
            } else {
                match switch_to_second_part {
                    false => {
                        let split: Vec<i32> = line.split('|').map(|c| c.parse().unwrap()).collect();
                        let rule: (i32, i32) = (split[0], split[1]);
                        rules.push(rule);
                    }
                    true => {
                        prints.push(line.split(',').map(|c| c.parse().unwrap()).collect());
                    }
                }
            }
        });
    println!("{rules:?}");
    println!("{prints:?}");
}

pub fn part2() {}

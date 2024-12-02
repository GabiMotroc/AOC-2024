use std::collections::HashMap;
use std::io::stdin;

pub fn part1() {
    let mut left_col = vec![];
    let mut right_col = vec![];

    stdin().lines()
        .for_each(
            |x| {
                let a: Vec<i32> = x
                    .unwrap()
                    .split_whitespace()
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect();

                left_col.push(a[0]);
                right_col.push(a[1]);
            }
        );

    left_col.sort();
    right_col.sort();

    let mut result = 0;
    for x in 0..right_col.len()
    {
        result += (left_col[x] - right_col[x]).abs();
    }

    println!("{result:?}");
}

pub fn part2() {
    let mut left_col = vec![];
    let mut right_col = vec![];
    stdin().lines()
        .for_each(
            |x| {
                let a: Vec<i32> = x
                    .unwrap()
                    .split_whitespace()
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect();

                left_col.push(a[0]);
                right_col.push(a[1]);
            }
        );

    let mut hm = HashMap::new();
    for x in right_col {
        hm.entry(x).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut result = 0;

    for x in left_col {
        match hm.get(&x) {
            None => {}
            Some(value) => {
                result += x * value;
            }
        }
    }

    println!("{result:?}");
}

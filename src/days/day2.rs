use std::io::stdin;

pub fn part1() {
    let result: usize =
        stdin().lines()
            .map(
                |line| {
                    let arr: Vec<usize> = line
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    let res_asc = is_arr_safe_asc(arr.clone());
                    let res_desc = is_arr_safe_desc(arr);
                    let result = res_asc || res_desc;
                    match result {
                        true => { 1 }
                        false => { 0 }
                    }
                }
            )
            .sum();

    println!("{result:?}");
}

fn is_arr_safe_asc(arr: Vec<usize>) -> bool {
    let mut it = arr.into_iter();
    match it.next() {
        None => { false }
        Some(first) => it.scan(first, |prev, next| {
            let mut cmp = *prev <= next;

            let abs_diff = prev.abs_diff(next);

            cmp &= abs_diff >= 1;
            cmp &= abs_diff <= 3;

            *prev = next;

            Some(cmp)
        }).all(|b| b),
    }
}

fn is_arr_safe_desc(arr: Vec<usize>) -> bool {
    let mut it = arr.into_iter();
    match it.next() {
        None => { false }
        Some(first) => it.scan(first, |prev, next| {
            let mut cmp = *prev >= next;

            let abs_diff = prev.abs_diff(next);

            cmp &= abs_diff >= 1;
            cmp &= abs_diff <= 3;

            *prev = next;

            Some(cmp)
        }).all(|b| b),
    }
}

pub fn part2() {
    let result: usize =
        stdin().lines()
            .map(
                |line| {
                    let arr: Vec<usize> = line
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();

                    for index in 0..arr.len() {
                        let mut new_arr = arr.clone();
                        new_arr.remove(index);
                        let res_asc = is_arr_safe_asc(new_arr.clone());
                        let res_desc = is_arr_safe_desc(new_arr);
                        let result = res_asc || res_desc;
                        match result {
                            true => { return 1; }
                            false => {}
                        }
                    }

                    0
                }
            )
            .sum();

    println!("{result:?}");
}

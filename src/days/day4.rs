use std::io::stdin;

pub fn part1() {
    let rows: Vec<Vec<char>> = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let height = rows.len();
    let width = rows[0].len();

    let mut result = 0;

    result += reverse_and_count(&rows);

    let mut transposed = vec![vec![' '; height]; width];

    for i in 0..height {
        for y in 0..width{
            transposed[y][i] = rows[i][y];
        }
    }
    result += reverse_and_count(&transposed);
    
    let primary_diagonal = primary_diagonals(&rows);
    result += reverse_and_count(&primary_diagonal);

    let secondary_diagonal = secondary_diagonals(&rows);
    result += reverse_and_count(&secondary_diagonal);

    println!("{rows:?}");
    println!("{transposed:?}");
    println!("{primary_diagonal:?}");
    println!("{secondary_diagonal:?}");
    println!("{result:?}");
}

fn primary_diagonals(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = rows.len();
    let width: i32 = rows[0].len() as i32;
    let mut diagonals: Vec<Vec<char>> = Vec::new();

    for col in 0..width {
        let mut diagonal = Vec::new();
        let mut i = 0;
        let mut j: i32 = col;
        while i < height && j >= 0 {
            diagonal.push(rows[i][j as usize]);
            i += 1;
            j -= 1;
        }
        diagonals.push(diagonal);
    }

    for row in 1..height {
        let mut diagonal = Vec::new();
        let mut i = row;
        let mut j = width - 1;
        while i < height && j as isize >= 0 {
            diagonal.push(rows[i][j as usize]);
            i += 1;
            j -= 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn secondary_diagonals(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = rows.len();
    let width = rows[0].len();
    let mut diagonals: Vec<Vec<char>> = Vec::new();

    for col in (0..width).rev() {
        let mut diagonal = Vec::new();
        let mut i = 0;
        let mut j = col;
        while i < height && j < width {
            diagonal.push(rows[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    for row in 1..height {
        let mut diagonal = Vec::new();
        let mut i = row;
        let mut j = 0;
        while i < height && j < width {
            diagonal.push(rows[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn reverse_and_count(rows: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    for row in rows.clone() {
        result += count(row)
    }

    for row in rows.clone() {
        result += count(row.into_iter().rev().collect::<Vec<char>>())
    }
    result
}

fn count_matrix(m: Vec<Vec<char>>) -> i32 {
    m.into_iter().map(count)
        .sum()
}

fn count(s: Vec<char>) -> i32 {
    s.windows(4)
        .filter(|&w| w.iter().collect::<String>() == *"XMAS")
        .count() as i32
}

pub fn part2() {}

/*
S..S..S
.A.A.A.
..MMM..
SAMXMAS
.AMMM..
.A.A.A.
S..S..S
 */
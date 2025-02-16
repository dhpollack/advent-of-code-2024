use itertools::iproduct;
use log::{debug, info};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const TARGET: &[char] = &['X', 'M', 'A', 'S'];
const TRUE_FALSE: &[bool] = &[true, false];

fn check_xmas(
    matrix: &HashMap<(usize, usize), char>,
    x: usize,
    y: usize,
    i: usize,
    j: usize,
    i_add: bool,
    j_add: bool,
    pos: usize,
) -> bool {
    let k = {
        match (i_add, j_add) {
            (true, true) => (x + i, y + j),
            (false, true) => (x - i, y + j),
            (true, false) => (x + i, y - j),
            (false, false) => (x - i, y - j),
        }
    };
    match matrix.get(&k) {
        Some(v) => *v == TARGET[pos],
        _ => false,
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let matrix: HashMap<(usize, usize), char> = lines
            .map_while(Result::ok)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x, y), c))
                    .collect::<Vec<((usize, usize), char)>>()
                    .into_iter()
            })
            .collect();
        let mut answer = 0;
        matrix.clone().into_iter().for_each(|((x, y), c)| match c {
            'X' => {
                debug!("Found an X, doing something @ ({x}, {y})");
                // i_add = left or right
                // j_add = up or down
                // use_x = move along x-axis
                // use_y = move along y-axis
                // true, true, true, true = diagonal right
                // true, false, true, true = diagonal left
                // true, true, false, true = straight up
                // false, true, false, true = straight up
                // _, _, false, false = go nowhere
                // true, true, true, false = right
                // true, false, true, false = right
                // true, _, false, _ = up or down
                // _, true, _, false = left or right
                for (&i_add, &j_add, &use_x, &use_y) in
                    iproduct!(TRUE_FALSE, TRUE_FALSE, TRUE_FALSE, TRUE_FALSE)
                {
                    if (3 > x && !i_add && use_x) || (3 > y && !j_add && use_y) {
                        continue;
                    }
                    match (i_add, j_add, use_x, use_y) {
                        // n (true, true, false, false) = nowhere
                        // n (true, false, false, false) = nowhere
                        // n (false, true, false, false) = nowhere
                        // n (false, false, false, false) = nowhere
                        //
                        // n (true, false, false, true) = down
                        // n (true, true, false, true) = up
                        //
                        // n (true, true, true, false) = right
                        // n (false, true, true, false) = left
                        (_, _, false, false) | (true, _, false, _) | (_, true, _, false) => {
                            continue
                        }
                        // y (false, true, false, true) = up
                        // y (false, false, false, true) = down
                        // y (true, false, true, false) = right
                        // y (false, false, true, false) = left
                        // y (true, true, true, true) = diagonal up and right
                        // y (false, true, true, true) = diagonal up and left
                        // y (true, false, true, true) = diagonal down and right
                        // y (false, false, true, true) = diagonal down and left
                        _ => {}
                    }
                    let mut is_good = true;
                    for pos in 1..4 {
                        let mut i = pos;
                        let mut j = pos;
                        match (use_x, use_y) {
                            (true, false) => j = 0,
                            (false, true) => i = 0,
                            _ => {}
                        };
                        if check_xmas(&matrix, x, y, i, j, i_add, j_add, pos) {
                            continue;
                        } else {
                            is_good = false;
                            break;
                        }
                    }
                    if is_good {
                        info!("Found XMAS @ ({x}, {y})");
                        answer += 1;
                    }
                }
            }
            _ => {
                debug!("Not an X");
            }
        });
        // 2495 is too low
        info!("Answer: {answer}");
    }
    Ok(())
}

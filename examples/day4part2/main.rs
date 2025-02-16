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

const TRUE_FALSE: &[bool] = &[true, false];

fn check_x_mas(
    matrix: &HashMap<(usize, usize), char>,
    x: usize,
    y: usize,
    check_axis: bool,
    check_direction: bool,
) -> bool {
    debug!("{x} {y} {check_axis} {check_direction}");
    if (2 > x && !check_axis && !check_direction) || (2 > y && check_axis && check_direction) {
        debug!("Kill: {x} {y} {check_axis} {check_direction}");
        return false;
    }
    let mut is_good = true;
    for (c, rel_x, rel_y) in [('M', 2, 0), ('A', 1, 1), ('S', 0, 2), ('S', 2, 2)].into_iter() {
        let k = {
            match (check_axis, check_direction) {
                // true, true = x-axis, up
                // true, false = x_axis, down
                // false, true = y-axis, right
                // false, false = y-axis, left
                (true, true) => (x + rel_x, y - rel_y),
                (true, false) => (x + rel_x, y + rel_y),
                (false, true) => (x + rel_y, y + rel_x),
                (false, false) => (x - rel_y, y + rel_x),
            }
        };
        is_good = match matrix.get(&k) {
            Some(v) => {
                debug!("{v} {c} {k:?}");
                *v == c
            }
            _ => false,
        };
        if !is_good {
            break;
        }
    }
    is_good
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
            'M' => {
                debug!("Found an M, doing something @ ({x}, {y})");
                // true, true = x-axis, up
                // true, false = x_axis, down
                // false, true = y-axis, right
                // false, false = y-axis, left
                for (&check_axis, &check_direction) in iproduct!(TRUE_FALSE, TRUE_FALSE) {
                    if check_x_mas(&matrix, x, y, check_axis, check_direction) {
                        info!("Found X-MAS @ ({x}, {y})");
                        answer += 1;
                    }
                }
            }
            _ => {
                debug!("Not an M");
            }
        });
        // 2495 is too low
        info!("Answer: {answer}");
    }
    Ok(())
}

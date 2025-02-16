use itertools::Itertools;
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

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

fn main() -> anyhow::Result<()> {
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let answer = lines
            .map_while(Result::ok)
            .filter_map(|line| {
                let mut ascending: Option<bool> = None;
                line.split_whitespace()
                    .tuple_windows()
                    .try_for_each(|(a, b)| {
                        let (col1, col2) =
                            (a.parse::<i32>().unwrap(), b.trim().parse::<i32>().unwrap());
                        let is_ascending = col1 < col2;
                        let direction = ascending.unwrap_or(is_ascending);
                        let diff = (col1 - col2).abs();
                        if is_ascending != direction || !(MIN_DIFF..=MAX_DIFF).contains(&diff) {
                            Err(())
                        } else {
                            ascending = Some(direction);
                            Ok(())
                        }
                    })
                    .ok()
            })
            .count();
        println!("Answer: {answer}");
    }
    Ok(())
}

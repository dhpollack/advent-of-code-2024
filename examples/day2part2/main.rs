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

fn filter_fn(line: String, skip: bool, skip_i: usize, lines_vec: &mut Vec<String>) -> Option<()> {
    let mut ascending: Option<bool> = None;
    let mut prev_item: Option<i32> = None;
    line.trim()
        .split(" ")
        .enumerate()
        .try_for_each(|(i, a)| {
            if skip && i == skip_i {
                return Ok(());
            }
            let curr = a.parse::<i32>().expect("this is a number");
            match prev_item {
                Some(prev) => {
                    let is_ascending = prev < curr;
                    let direction = ascending.unwrap_or(is_ascending);
                    let cond1 = is_ascending != direction;
                    let diff = (prev - curr).abs();
                    let cond2 = !(MIN_DIFF..=MAX_DIFF).contains(&diff);
                    if cond1 || cond2 {
                        // if already found bad and bad condition again fail
                        Err(())
                    } else {
                        ascending = Some(direction);
                        prev_item = Some(curr);
                        Ok(())
                    }
                }
                // First iteration we set the prev_item to the current item
                None => {
                    prev_item = Some(curr);
                    Ok(())
                }
            }
        })
        .inspect_err(|_| {
            lines_vec.push(line);
        })
        .ok()
}

fn main() -> anyhow::Result<()> {
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let mut lines_vec: Vec<String> = Vec::new();
        let mut answer = lines
            .map_while(Result::ok)
            .filter_map(|line| filter_fn(line.clone(), false, 0, &mut lines_vec))
            .count();
        // naively remove an item
        for skip_i in 0..20 {
            let lines_iter = lines_vec.clone();
            lines_vec = Vec::new();
            answer += lines_iter
                .clone()
                .into_iter()
                .filter_map(|line| filter_fn(line.clone(), true, skip_i, &mut lines_vec))
                .count();
            if lines_vec.is_empty() {
                println!("{skip_i}");
                break;
            }
        }
        println!("Answer: {answer}");
    }
    Ok(())
}

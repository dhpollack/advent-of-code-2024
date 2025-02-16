use regex::Regex;
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

const MIN_NUM_LEN: i32 = 1;
const MAX_NUM_LEN: i32 = 3;

fn main() -> anyhow::Result<()> {
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let re = Regex::new(r"mul\(([0-9]{1, 3}),([0-9]{1, 3})\)").expect("valid regex");
        let answer: i32 = lines
            .map_while(Result::ok)
            .flat_map(|line| {
                let matches: Vec<i32> = re
                    .captures_iter(&line)
                    .map(|c| c.extract())
                    .map(|(_, [s1, s2])| {
                        let n1 = s1.parse::<i32>().unwrap();
                        let n2 = s2.parse::<i32>().unwrap();
                        n1 * n2
                    })
                    .collect();
                matches.into_iter()
            })
            .sum();
        println!("Answer: {answer}");
    }
    Ok(())
}

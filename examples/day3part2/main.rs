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

fn add_numbers(s1: &str, s2: &str, is_do: bool) -> i32 {
    if is_do {
        let n1 = s1.parse::<i32>().unwrap();
        let n2 = s2.parse::<i32>().unwrap();
        n1 * n2
    } else {
        0
    }
}

fn main() -> anyhow::Result<()> {
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let re =
            Regex::new(r".*?((?:do\(\))|(?:don\'t\(\)))?.*?mul\(([0-9]{1, 3}),([0-9]{1, 3})\).*?")
                .expect("valid regex");
        let mut is_do = true;
        let answer: i32 = lines
            .map_while(Result::ok)
            .flat_map(|line| {
                let matches: Vec<i32> = re
                    .captures_iter(&line)
                    .map(|c| match (c.get(1), c.get(2), c.get(3)) {
                        (None, Some(s1), Some(s2)) => {
                            let prefix = c.get(0).unwrap().as_str();
                            if prefix.contains("do()") {
                                is_do = true;
                            } else if prefix.contains("don't()") {
                                is_do = false;
                            }
                            println!("None ({is_do}): {}", c.get(0).unwrap().as_str());
                            add_numbers(s1.as_str(), s2.as_str(), is_do)
                        }
                        (Some(do_or_dont), Some(s1), Some(s2)) => {
                            println!("do or don't: {}\n", do_or_dont.as_str());
                            is_do = do_or_dont.as_str() == "do()";
                            println!("Some ({is_do}): {}", c.get(0).unwrap().as_str());
                            add_numbers(s1.as_str(), s2.as_str(), is_do)
                        }
                        _ => {
                            println!("{}", c.get(0).unwrap().as_str());
                            0
                        }
                    })
                    .collect();
                matches.into_iter()
            })
            .sum();
        println!("Answer: {answer}");
    }
    Ok(())
}

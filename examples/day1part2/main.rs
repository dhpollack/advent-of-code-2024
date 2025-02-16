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

fn main() -> anyhow::Result<()> {
    let input_path = Path::new(file!())
        .parent()
        .ok_or(0)
        .expect("wrong path")
        .join("input");

    if let Ok(lines) = read_lines(input_path) {
        let (col1, col2): (Vec<i32>, Vec<i32>) = lines
            .map_while(Result::ok)
            .filter_map(|line| {
                if let Some((a, b)) = line.trim().split_once(" ") {
                    Some((a.parse::<i32>().unwrap(), b.trim().parse::<i32>().unwrap()))
                } else {
                    None
                }
            })
            .unzip();
        let col2_map: HashMap<i32, i32> = col2.into_iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        let similarity: i32 = col1
            .iter()
            .map(|x| col2_map.get(x).map_or(0, |v| x * v))
            .sum();
        println!("Answer: {similarity}");
    }
    Ok(())
}

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
        let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = lines
            .map_while(Result::ok)
            .filter_map(|line| {
                if let Some((a, b)) = line.trim().split_once(" ") {
                    Some((a.parse::<i32>().unwrap(), b.trim().parse::<i32>().unwrap()))
                } else {
                    None
                }
            })
            .unzip();
        col1.sort();
        col2.sort();
        let total_diff: i32 = col1
            .iter()
            .zip(col2.iter())
            .map(|tuple| {
                let diff = tuple.0 - tuple.1;
                diff.abs()
            })
            .sum();
        println!("Total Difference: {total_diff}");
    }
    Ok(())
}

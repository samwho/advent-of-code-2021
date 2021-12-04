use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    let mut prev: i32 = 0;

    let lines: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    for window in lines.windows(3) {
        let curr: i32 = window.iter().sum();

        if prev != 0 && curr > prev {
            total += 1;
        }

        prev = curr;
    }

    println!("{}", total);
    Ok(())
}

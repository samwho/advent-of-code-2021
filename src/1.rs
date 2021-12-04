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

    for result in reader.lines() {
        let line = result?;
        let curr: i32 = line.parse()?;

        if prev != 0 && curr > prev {
            total += 1;
        }

        prev = curr;
    }

    println!("{}", total);
    Ok(())
}

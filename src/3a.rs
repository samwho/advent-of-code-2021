use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/3.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut ones: Vec<i32> = Vec::with_capacity(12);
    for _ in 0..12 {
        ones.push(0);
    }

    for result in reader.lines() {
        let line = result?;
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
        total += 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for j in 0..12 {
        let i = 11 - j;
        let mut mcb = 0;
        if ones[i] > total / 2 {
            mcb = 1;
        }
        gamma += (2 as i32).pow(j as u32) * mcb;
        epsilon += (2 as i32).pow(j as u32) * (1 - mcb);
    }

    println!("{}", gamma * epsilon);
    Ok(())
}

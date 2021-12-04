use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result::Result,
};

fn most_common_bit(lines: &[String], pos: usize) -> char {
    let mut zeroes = 0;
    let mut ones = 0;

    for line in lines {
        if line.chars().nth(pos).unwrap() == '1' {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }

    if ones >= zeroes {
        return '1';
    }
    return '0';
}

fn least_common_bit(lines: &[String], pos: usize) -> char {
    let mcb = most_common_bit(lines, pos);
    if mcb == '1' {
        return '0';
    } else {
        return '1';
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/3.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();

    let mut o2_lines = lines.clone();
    let mut pos = 0;
    loop {
        let mcb = most_common_bit(o2_lines.as_slice(), pos);

        o2_lines = o2_lines
            .into_iter()
            .filter(|line| line.chars().nth(pos).unwrap() == mcb)
            .collect();

        if o2_lines.len() <= 1 {
            break;
        }

        pos += 1;
    }

    let o2_val = i32::from_str_radix(&o2_lines[0], 2).unwrap();

    let mut co2_lines = lines.clone();
    let mut pos = 0;
    loop {
        let lcb = least_common_bit(co2_lines.as_slice(), pos);

        co2_lines = co2_lines
            .into_iter()
            .filter(|line| line.chars().nth(pos).unwrap() == lcb)
            .collect();

        if co2_lines.len() <= 1 {
            break;
        }

        pos += 1;
    }

    let co2_val = i32::from_str_radix(&co2_lines[0], 2).unwrap();
    println!("{}", o2_val * co2_val);
    Ok(())
}

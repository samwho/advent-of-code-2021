use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/2.txt")?;
    let reader = BufReader::new(file);

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for result in reader.lines() {
        let line = result?;

        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts.as_slice() {
            [command, amount] => {
                let num: i32 = amount.parse()?;
                match *command {
                    "forward" => x += num,
                    "up" => y -= num,
                    "down" => y += num,
                    _ => panic!("invalid command"),
                }
            }
            _ => panic!("invalid line"),
        }
    }

    println!("{}", x * y);
    Ok(())
}

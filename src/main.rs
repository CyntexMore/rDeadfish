use std::env::args;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Error, Write};
use std::process;

fn interpret(input: &str) -> Option<Vec<i32>> {
    let mut accumulator: i32 = 0;
    let mut result = Vec::new();

    for exp in input.chars() {
        if accumulator == 256 || accumulator < 0 {
            accumulator = 0;
        }

        match exp {
            'i' => accumulator += 1,
            'd' => accumulator -= 1,
            's' => accumulator *= accumulator,
            'o' => result.push(accumulator),
            'q' => return None,
            _ => (),
        }
    }
    Some(result)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if !(args.len() > 1) {
        loop {
            let mut input = String::new();
            print!(">> ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read input!");

            let mut output = String::new();
            for i in match interpret(&input) {
                Some(o) => o,
                None => process::exit(0),
            } {
                output = format!("{}{}", output, i);
            }
            println!("{}", output);
        }
    } else {
        let file_path = &args[1];
        let file = File::open(file_path)?;
        let buffer = BufReader::new(file);

        for instructions in buffer.lines() {
            let mut output = String::from(">> ");
            for i in match interpret(&instructions?) {
                Some(o) => o,
                None => process::exit(0),
            } {
                output = format!("{}{}", output, i);
            }
            println!("{}", output);
        }
    }

    Ok(())
}

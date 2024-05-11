use std::io;
use std::io::Write;
use std::process;

fn interpret(input: &str) -> Option<Vec<u8>> {
    let mut accumulator: u8 = 0;
    let mut result = Vec::new();

    for exp in input.chars() {
        match exp {
            'i' => accumulator = accumulator.overflowing_add(1).0,
            'd' => accumulator = accumulator.overflowing_sub(1).0,
            's' => accumulator *= accumulator,
            'o' => result.push(accumulator),
            'q' => return None,
            _ => (),
        }
    }
    Some(result)
}

fn main() {
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input!");

        let output = match interpret(&input) {
            Some(o) => o,
            None => process::exit(0),
        };
        println!("{:?}", output);
    }
}

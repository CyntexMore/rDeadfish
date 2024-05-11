use std::io;
use std::process;
use std::io::Write;

fn interpret(input: String) -> Vec<u8> {
    let mut accumulator = 0;
    let mut result = Vec::new();

    // go through all of the characters one by one and make them do their thing
    for exp in input.chars() {
        match exp {
            'i' => accumulator += 1,           // +1 to the accumulator
            'd' => accumulator -= 1,           // -1 from the accumulator
            's' => accumulator *= accumulator, // squares the value of the accumulator
            'o' => result.push(accumulator),   // prints the value of the accumulator
            'q' => process::exit(1),           // quits the program
            _ => (),
        }
    }
    result
}

fn main() {
    loop {
        // get user input
        let mut input = String::new();
        print!(">> ");
        io::stdout()
            .flush()
            .unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        // print the results
        let output = interpret(input);
        println!("{:?}\n", output);
    }
}

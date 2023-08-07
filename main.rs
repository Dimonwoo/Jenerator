use std::io::{self, Write};
use std::time::{Duration, Instant};

mod generator;
mod file_manager;

fn main() {
    let mut input: String = String::new();

    print!("Enter the number of tokens to generate: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_tokens_to_generate: usize = input.trim().parse().expect("Invalid input");

    let start_time: Instant = Instant::now();
    let file_name: &str = "tokens.txt";
    let tokens: Vec<String> = generator::generate_tokens_multithreaded(num_tokens_to_generate);

    match file_manager::write_tokens_to_file(file_name, &*tokens) {
        Ok(tokens_written) => {
            let end_time: Instant = Instant::now();
            let duration: Duration = end_time - start_time;

            println!("Tokens have been saved in {}.", file_name);
            println!("Number of tokens written to the file: {}", tokens_written);
            println!("Time taken to generate tokens: {:?}", duration);

            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
        }
        Err(e) => eprintln!("Failed to write tokens to file: {}", e),
    }
}
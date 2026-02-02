use std::io::{self};
use tiberias::{calculate_median, calculate_mode, convert_sentence};

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let numbers1 = [1, 3, 3, 4, 5, 5, 7, 8, 9, 5];

    if let Some(rex) = numbers.get(5) {
        println!("value: {rex}")
    } else {
        println!("couldnt get value")
    }

    //let median = calculate_median(&numbers);
    if let Some(median) = calculate_median(&numbers) {
        println!("median: {}", median);
    }
    // match median {
    //     Some(median) => println!("median: {}", median),
    //     None => println!("Median not obtainable"),
    // }
    let mode = calculate_mode(&numbers1);
    match mode {
        Some(mode) => println!("mode: {}", mode),
        None => println!("Mode not obtainable"),
    }

    println!("=== Pig Latin Converter ===\n");
    loop {
        println!("Enter a word or sentence (or 'quit' to exit): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        match input {
            "" => continue,
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                let result = convert_sentence(input);
                println!("Pig Latin: {}\n", result);
            }
        }
    }
}

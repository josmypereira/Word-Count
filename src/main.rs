// Count the words in any file passed using command line
// Created by Josmy Pereira October 13, 2024


use std::env; // Import the env module for handling command line arguments
use std::fs; // Import the fs module for file system operations
use std::collections::HashMap; // Import the HashMap collection for storing word counts

fn main() {
    // Attempt to read the file specified as a command line argument
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            // If the file is successfully read, convert contents to lowercase
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                // If an error occurs, print the error message and exit the program
                eprintln!("Could not read file: {}", e);
                std::process::exit(1); // Exit with error code 1
            }
        },
        None => {
            // If no argument is provided, print a usage message and exit
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2); // Exit with error code 2
        }
    };

    // Split the contents into individual words and collect them into a vector
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // Create a HashMap to count occurrences of each unique word
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        // Insert the word into the HashMap and increment its count
        *word_counts.entry(word).or_insert(1) += 1; // Default count is 1
    }
    
    // Initialize variables to track the highest word count and the corresponding words
    let mut top_count = 0u32; // Variable to store the highest count
    let mut top_words: Vec<&str> = Vec::new(); // Vector to store the top words

    // Iterate through the word counts to determine the most commonly used word(s)
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            // If the current word's count exceeds the top count, update top count and reset top words
            top_count = val;
            top_words.clear();
            top_words.push(key); // Add the new top word
        } else if val == top_count {
            // If the current word's count matches the top count, add it to the top words
            top_words.push(key);
        }
    }

    // Display the results: the top word(s) and their occurrence count
    println!("Top word(s) occurred {} times:", top_count);
    for word in top_words.iter() {
        println!("{}", word); // Print each top word
    }
}

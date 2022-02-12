use std::io;
// use std::fs;

// Emojis
// Black square U+2B1B or U+25FC
// Yellow square U+1F7E8
// Green square U+1F7E9

fn main() {
    // let words_list = fs::read_to_string("./words.txt")
    //     .expect("Something went wrong reading the file");
    
    let wordle = String::from("pizza");

    let inplace = "\u{1F7E9}";
    let contains = "\u{1F7E8}";
    let missing = "\u{2B1B}";

    let mut hint_vec: Vec<&str>;
    let mut guess: String;
    
    println!("Terminal Wordle. 5 letter word.");
    
    for i in 0..5 {
        let mut c = 0;

        loop {
            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: String = match guess.trim().parse() {
                Ok(string) => string,
                Err(_) => continue,
            };
            
            // TODO: Change to match statement
            if guess.chars().count() > 5 {
                println!("Word too long.");
            } else if guess.chars().count() < 5 {
                println!("Word too short.")
            } else {
                break;
            }
        }

        hint_vec = guess
            .trim()
            .chars()
            .map(|letter| {
                let wletter = wordle.as_bytes()[c] as char;
                c += 1;
                if letter == wletter {
                    inplace
                } 
                else if wordle.contains(letter) {
                    contains
                }
                else {
                    missing
                }
            })
            .collect();
        
        let hint: String = hint_vec.into_iter().collect();
        
        println!("{}  ({}/5)", hint, i+1);

        if guess.trim() == wordle.trim() {
            println!("You win!");
            break;
        }
    }
}
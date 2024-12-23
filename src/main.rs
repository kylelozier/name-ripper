use std::io;
use std::fs::File;
use std::io::BufReader;
use std::{collections::HashSet, env, io::{BufRead, Write}};

fn main() -> io::Result<()> {
    println!("Input the file name: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input.");

    let path = env::current_dir()?;
    let path = path.join(input.trim());

    println!("You entered {}", input);
    println!("Looking for {}", path.display());

    let f = File::open(path)
        .expect("Failed to open file.");

    println!("Found: {input}");

    let reader = BufReader::new(f);
    let mut copied_words: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let mut copy_word = String::new();
                let mut copying: bool = false;
                for ch in l.chars() {
                    if ch.is_ascii_uppercase() {
                        copying = true;
                        copy_word.push(ch);
                    }
                    else if ch.is_ascii_lowercase() && copying == true {
                        copy_word.push(ch);
                    }
                    else if !ch.is_ascii_alphabetic() {
                        copying = false;
                        if copy_word.len() > 1 {
                            copied_words.push(copy_word.clone());
                            copy_word.clear();
                        }
                        else {
                            copy_word.clear();
                        }
                    }
                }
            },
            Err(e) => eprintln!("Error reading line: {e}, e"),
        }
    }

       // Use a HashSet to remove duplicates
    let unique_copied_words: HashSet<String> = copied_words.into_iter().collect();

    // Convert the HashSet back to a Vec<String>
    let mut unique_copied_words_vec: Vec<String> = unique_copied_words.into_iter().collect();

    unique_copied_words_vec.sort();

    println!("{} unique capitalized words found.", unique_copied_words_vec.len());

    let mut file = File::create("unique_words.txt")?;

    for item in unique_copied_words_vec {
        writeln!(file, "{}", item)?;
    }

    Ok(())
}

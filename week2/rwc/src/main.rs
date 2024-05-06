use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename)?;
    let mut content_line :Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines(){
        let line = line?;
        content_line.push(line);
    }
    let mut content_word :Vec<String> =Vec::new();
    println!("lines:{}",content_line.len());
    for line in content_line.iter(){
        for word in line.split_whitespace(){
            content_word.push(word.to_string());
        }
    }
    println!("words:{}",content_word.len());
    let mut content_char :Vec<char> = Vec::new();
    for word in content_word.iter(){
        for word_char in word.chars(){
            content_char.push(word_char);
        }
    }
    println!("chars:{}",content_char.len());
}

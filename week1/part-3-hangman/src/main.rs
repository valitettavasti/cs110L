// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut rust_guess_times = NUM_INCORRECT_GUESSES;
    let mut output_words :Vec<char>=Vec::new();
    let mut guessed_words:Vec<char>=Vec::new();
    for _ in 0..secret_word_chars.len(){
        output_words.push("-".parse().unwrap());
    }
    let mut count_number = 0;
    while rust_guess_times > 0 {
        let output_words_string:String = output_words.iter().collect();
        println!("The word so far is {}",output_words_string);
        let guessed_words_string:String = guessed_words.iter().collect();
        println!("You have guessed the following letters:{}",guessed_words_string);
        println!("You have {} guesses left",rust_guess_times);
        print!("please guess a letter:");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_char = guess.chars().next().unwrap();
        guessed_words.push(guess_char);
        let mut judge = false;
        'counting_up:for i in 0..secret_word_chars.len(){
            if secret_word_chars[i] == guess_char{
                output_words[i] = guess_char;
                judge = true;
                count_number += 1;
                //break 'counting_up;
            }
        }
        if judge==false {
            rust_guess_times-=1;
        }
        if count_number==secret_word_chars.len(){
            break;
        }
    }
    if count_number==secret_word_chars.len(){
        println!("Congratulations you guessed the secret word:{}!",secret_word);
    }else {
        println!("Sorry, you ran out of guesses!");
    }
}

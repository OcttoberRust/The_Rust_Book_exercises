use std::io;

fn main() {
    //Take in a string
    let mut input = String::new();
    
    println!("Please type in a phrase to convert to Pig Latin. ");
    
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    println!("You typed: {}", input);

    for word in input.split_whitespace() {
        println!("{}", word);
    }

    //if word starts with a vowel, add hay to the end
    //add first consonant to end of word and add "ay"
    //keep in mind UTF-8
}

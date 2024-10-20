use std::io;

fn main() {
    //Take in a string
    let mut input = String::new();
    
    println!("Please type in a phrase to convert to Pig Latin. ");
    
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    println!("You typed: {}", input);

    let mut final_string: String = String::new();

    for word in input.split_whitespace() {
        println!("{}", word);
        if let Some(first_char) = word.chars().next() {
            if "aeiouAEIOU".contains(first_char) {
                final_string.push_str(&vowel_rule(word));
            } else {
                final_string.push_str(&consonant_rule(word));   
            }
        }
    }

    println!("final outcome: {}", final_string);
    //if word starts with a vowel, add hay to the end
    //add first consonant to end of word and add "ay"
    //keep in mind UTF-8
}

fn vowel_rule(word: &str) -> String {
    let mut new_string = String::new();
    new_string.push_str(&word[..]);
    new_string.push_str("ay ");
    new_string
}

fn consonant_rule(word: &str) -> String {
    let mut new_string = String::new();
    new_string.push_str(&word[1..]);
    new_string.push_str(&word[..1]);
    new_string.push_str("ay ");
    new_string
}

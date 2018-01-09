// Vigenere Cipher
#![feature(inclusive_range_syntax)]

fn main() {
    encrypt("Universum", "Earth");
}

fn char_to_pos(input: char) -> usize {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().position(|c| { input == c } ).unwrap()
}

fn encrypt(input: &str, key: &str) -> String {
    let w = input.to_string().to_uppercase();
    let k = key.to_string().to_uppercase();

    let key_alphabet_pos = k.chars().map(|c|{ char_to_pos(c) }).collect::<Vec<usize>>();

    // The amount of chars possible at key length
    let chunk_amount: f32 = w.len() as f32 / k.len() as f32;

    // Create ranges from where to where each group/chunk goes
    let split_word: Vec<Vec<char>> = (0..chunk_amount.ceil() as usize).map(|chunk| {
        let lowerbound = chunk * key.len();
        let upperbound = {
            // Last chunk/group upperbound
            if chunk == (chunk_amount.ceil() as usize) - 1 { w.len() - 1 }
            // General chunk/group upperbound
            else { lowerbound + key.len() - 1 }
        };
        // Collect the chars
        (lowerbound..=upperbound).map(|i| { w.chars().nth(i).unwrap()}).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();


    println!("{:?}", split_word);

    "Test word".to_string()
}

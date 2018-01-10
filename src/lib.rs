#![feature(inclusive_range_syntax)]

pub enum Method {
    Encipher,
    Decipher,
}

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_to_pos(input: char) -> usize {
    ALPHABET.chars().position(|c| input == c).unwrap()
}

pub fn cipher(input: &str, key: &str, method: Method) -> String {
    let w = input.to_string().to_uppercase();
    let k = key.to_string().to_uppercase();

    let alphabet_chars_pos = k.chars().map(|c| char_to_pos(c)).collect::<Vec<usize>>();

    // The amount of chars possible at key length
    let chunk_amount = w.len() as f32 / k.len() as f32;

    // Create ranges from where to where each group/chunk goes
    //let split_word: Vec<Vec<char>>
    (0..chunk_amount.ceil() as usize)
        .map(|chunk| {
            let lowerbound = chunk * key.len();
            let maximum = (chunk_amount.ceil() as usize) - 1;

            let upperbound = if chunk == maximum {
                // Last chunk/group upperbound
                w.len() - 1
            } else {
                // General chunk/group upperbound
                lowerbound + key.len() - 1
            };

            // Collect the char positions
            (lowerbound..=upperbound)
                .map(|i| char_to_pos(w.chars().nth(i).unwrap()))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
        .into_iter()
        .map(|char_chunk| {
            char_chunk
                .into_iter()
                .enumerate()
                .map(|(i, pos)| pos_to_moved_letter(pos, &method, &alphabet_chars_pos, i))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .concat()
}

// Moves position by key and returns the corresponding char
fn pos_to_moved_letter(
    letter_pos: usize,
    method: &Method,
    alphabet_char_pos: &Vec<usize>,
    index: usize,
) -> char {
    let moved_char_pos = match method {
        &Method::Encipher => (letter_pos + alphabet_char_pos[index]) % ALPHABET.len(),
        &Method::Decipher => {
            let new_pos = letter_pos as i8 - alphabet_char_pos[index] as i8;
            // If the position is negative, start from the end of the alphabet
            if new_pos < 0 {
                (ALPHABET.len() as i8 + new_pos) as usize
            } else {
                new_pos as usize
            }
        }
    };

    ALPHABET.chars().nth(moved_char_pos).unwrap()
}

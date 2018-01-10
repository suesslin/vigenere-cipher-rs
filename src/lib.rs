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

    let key_alphabet_pos = k.chars().map(|c| char_to_pos(c)).collect::<Vec<usize>>();

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
                .map(|(i, single)| {
                    let encrypted_char_pos = match method {
                        Method::Encipher => (single + key_alphabet_pos[i]) % ALPHABET.len(),
                        Method::Decipher => {
                            let pos = single as i8 - key_alphabet_pos[i] as i8;
                            // If the position is negative, start from the end of the alphabet
                            if pos < 0 {
                                (ALPHABET.len() as i8 + pos) as usize
                            } else {
                                pos as usize
                            }
                        }
                    };
                    ALPHABET.chars().nth(encrypted_char_pos).unwrap()
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .concat()
}

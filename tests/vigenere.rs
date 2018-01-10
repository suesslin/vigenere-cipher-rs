extern crate vigenere_cipher;

use vigenere_cipher::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enciphered_correct() {
        assert_eq!(cipher("VIGENERE", "DE", Method::Encipher), "YMJIQIUI");
    }

    #[test]
    fn deciphered_correct() {
        assert_eq!(cipher("YMJIQIUI", "DE", Method::Decipher), "VIGENERE");
    }

    #[test]
    fn decipher_enciphered_correct() {
        let word = "VIGENERE";
        let key = "DE";
        assert_eq!(
            cipher(&cipher(word, key, Method::Encipher), key, Method::Decipher),
            word
        );
    }
}

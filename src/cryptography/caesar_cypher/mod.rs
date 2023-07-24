pub fn caesar_cipher(input: &str, shift: i32) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let shift = shift % 26; // Make sure shift is within 0-25 range

    let cipher: String = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base_alphabet = if c.is_ascii_uppercase() {
                    alphabet
                } else {
                    lowercase_alphabet
                };

                let idx = base_alphabet
                    .find(c)
                    .expect("Failed to find the character in the alphabet.");

                let shifted_idx = (idx as i32 + shift + 26) % 26;
                base_alphabet.chars().nth(shifted_idx as usize).unwrap()
            } else {
                c
            }
        })
        .collect();

    cipher
}

pub fn caesar_decrypt(ciphertext: &str, shift: i32) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let shift = shift % 26; // Make sure shift is within 0-25 range

    let decrypted_text: String = ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base_alphabet = if c.is_ascii_uppercase() {
                    alphabet
                } else {
                    lowercase_alphabet
                };

                let idx = base_alphabet
                    .find(c)
                    .expect("Failed to find the character in the alphabet.");

                let shifted_idx = (idx as i32 - shift + 26) % 26;
                base_alphabet.chars().nth(shifted_idx as usize).unwrap()
            } else {
                c
            }
        })
        .collect();

    decrypted_text
}

// fn crack_caesar_cipher(ciphertext: &str) {
//     for shift in 0..26 {
//         let decrypted_text = caesar_decrypt(ciphertext, shift);
//         println!("Shift: {}, Decrypted Text: {}", shift, decrypted_text);
//     }
// }

#[cfg(test)]
mod test {

    use super::caesar_cipher;

    #[test]
    fn test_cct() {
        let plaintext = "HELLO";
        let shift = 3;
        let ciphertext = caesar_cipher(plaintext, shift);
        println!("Ciphertext: {}", ciphertext);
    
        // Decrypting the ciphertext
        let decrypted_text = caesar_cipher(&ciphertext, -shift);
        println!("Decrypted Text: {}", decrypted_text);
    }
}

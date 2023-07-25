// This function generates the key in
// a cyclic manner until it's length isn't
// equal to the length of original text
pub fn generate_key(plaintext: &str, key: &str) -> String {
    let sample: Vec<&str> = key.split("").collect();
    let mut new_key: Vec<&str> = Vec::new();
    new_key.push(key);

    for i in 0..=plaintext.len() - key.len() {
        new_key.push(sample[i%key.len()]);
    }

    new_key.join("")
}

// This function returns the encrypted text
// generated with the help of the key
pub fn vigenere_encrypt(plaintext: &str, key: &str) -> String {
    let plaintext_chars = plaintext.chars().collect::<Vec<char>>();
    let key_chars = key.chars().collect::<Vec<char>>();
    let mut new_key = String::new();

    for i in 0..plaintext.len() {
        let _x = plaintext_chars[i] as u8 + key_chars[i] as u8;
        let _y = _x % 26;
        let x = _y + 'A' as u8;
        new_key.push(x as char);
    }

    new_key
}

// This function decrypts the encrypted text
// and returns the original text
pub fn vigenere_decrypt(plaintext: &str, key: &str) -> String {
    let plaintext_chars = plaintext.chars().collect::<Vec<char>>();
    let key_chars = key.chars().collect::<Vec<char>>();
    let mut new_key = String::new();

    for i in 0..plaintext.len() {
        let x = (plaintext_chars[i] as u8 - key_chars[i] as u8 + 26) % 26 + 'A' as u8;
        new_key.push(x as char);
    }

    new_key

}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_vignere() {

        let plaintext = "HELLO";
        let key = "KEY";
    
        // Encryption
        let encrypted_text = vigenere_encrypt(plaintext, key);
        println!("Encrypted: {}", encrypted_text);
    
        // Decryption
        let decrypted_text = vigenere_decrypt(&encrypted_text, key);
        println!("Decrypted: {}", decrypted_text);
    }

}

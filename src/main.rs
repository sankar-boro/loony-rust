mod cryptography;
use cryptography::vignere::{vigenere_decrypt, vigenere_encrypt, generate_key};

fn main() {
    let plaintext = "HELLO";
    let keyword = "KEY";
    let key = generate_key(plaintext, keyword);

    
    // Encryption
    let encrypted_text = vigenere_encrypt(plaintext, &key);
    println!("Encrypted: {}", encrypted_text);
    
    // Decryption
    let decrypted_text = vigenere_decrypt(&encrypted_text, &key);
    println!("Decrypted: {}", decrypted_text);
    
    let xx = generate_key(plaintext, &key);
    println!("{}", xx);
}
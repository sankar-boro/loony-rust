use rand::Rng;
use std::io;

pub fn get_otp_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn generate_random_key_otp(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let key: String = (0..length)
        .map(|_| (rng.gen_range(b'A'..b'Z' + 1)) as char)
        .collect();
    key
}

pub fn encrypt_otp(plaintext: &str, key: &str) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();

    let encrypted_bytes: Vec<u8> = plaintext_bytes
        .iter()
        .zip(key_bytes.iter())
        .map(|(&p, &k)| p ^ k)
        .collect();

    String::from_utf8(encrypted_bytes).expect("Invalid UTF-8 characters")
}

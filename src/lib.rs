mod cryptography;

pub use cryptography::vignere::{vigenere_decrypt, vigenere_encrypt, generate_key};
pub use cryptography::caesar_cypher::{caesar_cipher, caesar_decrypt};
pub use cryptography::otpad::{encrypt_otp, generate_random_key_otp, get_otp_input};
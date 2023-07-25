use loony_rust::{get_otp_input, encrypt_otp, generate_random_key_otp};
fn main() {
    // Get the plaintext message from the user
    let plaintext = get_otp_input("Enter the plaintext message: ");

    // Generate a random key of the same length as the plaintext
    let key = generate_random_key_otp(plaintext.len());

    // Encrypt the plaintext using the key
    let ciphertext = encrypt_otp(&plaintext, &key);

    // Print the results
    println!("Plaintext: {}", plaintext);
    println!("Key: {}", key);
    println!("Ciphertext: {}", ciphertext);

    // Decrypt the ciphertext using the key to get back the original plaintext
    let decrypted_message = encrypt_otp(&ciphertext, &key);
    println!("Decrypted message: {}", decrypted_message);
}
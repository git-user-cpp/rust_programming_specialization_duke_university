use caesar_cipher::{encrypt, decrypt};

fn main() {
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let shift: u8 = 3;
    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Decrypted: {}", decrypted_text);
}

/*
   This program is a basic Vernam (One-time pad)
   cipher implemented in Rust.
*/

use rand::distributions::Alphanumeric;
use rand::prelude::*;

fn main() {
    // Let text be the input
    let text: String = "hello world".to_owned();
    let key: String = random_string(text.len());

    // Convert to a u8 representation
    let text_bin: Vec<u8> = to_binary(text.clone());
    let key_bin: Vec<u8> = to_binary(key.clone());

    // Xor the key and text
    let text_xor_key: Vec<u8> = binary_xor(&text_bin, &key_bin);

    // Convert back to string
    let ciphertext: String = vecu8_to_string(&text_xor_key);

    println!("Encrypted {} with key {}:", text, key);
    // Print the text such that it shows unicode sequences
    // That it cannot represent
    println!("{:?}", ciphertext);

    // Print what it can represent
    println!("{}", ciphertext);

    // Decrypt
    let cipher_bin: Vec<u8> = to_binary(ciphertext.clone());
    let cipher_xor_key: Vec<u8> = binary_xor(&cipher_bin, &key_bin);
    let plaintext: String = vecu8_to_string(&cipher_xor_key);

    println!("Decrypted {} with key {}:", ciphertext, key);
    println!("{:?}", plaintext);
    println!("{}", plaintext);
}

fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

fn to_binary(plaintext: String) -> Vec<u8> {
    /*
       Returns a Vec of u8s that represent the binary
       representation of every char in plaintext.
    */
    plaintext.into_bytes()
}

fn binary_xor(bin_text: &Vec<u8>, bin_key: &Vec<u8>) -> Vec<u8> {
    /*
       Binary XOR each item in a Vec<u8> with another Vec<u8>
    */
    bin_text
        .iter()
        .zip(bin_key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>()
}

fn vecu8_to_string(bin_text: &Vec<u8>) -> String {
    /*
       Convert a vec of u8 to a string
    */
    bin_text
        .iter()
        .map(|x| match std::char::from_u32((*x).into()) {
            Some(c) => c,
            None => '�',
        })
        .map(char::from)
        .collect()
}

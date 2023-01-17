/*
    This program is a basic Vernam (One-time pad)
    cipher implemented in Rust.
 */

fn main() {
    let text: &str = "1234567890";
    let key: &str = "qwertyuiop";
    
    // Convert to a u8 representation
    let text_bin: Vec<u32> = to_binary(text);
    let key_bin: Vec<u32> = to_binary(key);

    // Xor the key and text
    let text_xor_key: Vec<u32> = binary_xor(text_bin, key_bin);

    // Convert back to string
    let ciphertext: String = to_string(text_xor_key);

    // Print the text such that it shows unicode sequences
    // That it cannot represent
    println!("{:?}", ciphertext);

    // Print what it can represent
    println!("{}", ciphertext);
}

fn to_binary(plaintext: &str) -> Vec<u32> {
    /*
        Returns a Vec of u32s that represent the binary 
        representation of every char in plaintext.
     */
    let mut binary_vec: Vec<u32> = Vec::new();
    for c in plaintext.chars() {
        binary_vec.push(c as u32);
    }
    return binary_vec;
}

fn binary_xor(bin_text: Vec<u32>, bin_key: Vec<u32>) -> Vec<u32> {
    bin_text
        .iter()
        .zip(bin_key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u32>>()
}

fn to_string(bin_text: Vec<u32>) -> String {
    bin_text
        .iter()
        .map(|x| match std::char::from_u32(*x) {
            Some(c) => c,
            None => '�',
        })
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

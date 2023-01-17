/*
    This program is a basic Vernam (One-time pad)
    cipher implemented in Rust.
 */

fn main() {
    let text: String = "1234567890".to_owned();
    let key: String = "qwertyuiop".to_owned();
    
    // Convert to a u8 representation
    let text_bin: Vec<u8> = to_binary(text);
    let key_bin: Vec<u8> = to_binary(key);

    // Xor the key and text
    let text_xor_key: Vec<u8> = binary_xor(text_bin, key_bin);

    // Convert back to string
    let ciphertext: String = to_string(text_xor_key);

    // Print the text such that it shows unicode sequences
    // That it cannot represent
    println!("{:?}", ciphertext);

    // Print what it can represent
    println!("{}", ciphertext);
}

fn to_binary(plaintext: String) -> Vec<u8> {
    /*
        Returns a Vec of u8s that represent the binary 
        representation of every char in plaintext.
     */
    plaintext.into_bytes()
}

fn binary_xor(bin_text: Vec<u8>, bin_key: Vec<u8>) -> Vec<u8> {
    bin_text
        .iter()
        .zip(bin_key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>()
}

fn to_string(bin_text: Vec<u8>) -> String {
    bin_text
        .iter()
        .map(|x| match std::char::from_u32((*x).into()) {
            Some(c) => c,
            None => '�',
        })
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

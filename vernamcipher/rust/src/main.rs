use ascii_converter::*;

fn main() {
    let text: &str = "Hello World!";
    let key: &str = "abcdefghijkl";
    
    let text_bin: Vec<u8> = to_binary(text);
    let key_bin: Vec<u8> = to_binary(key);

    println!("{:?}", binary_xor(text_bin, key_bin));
}

fn to_binary(plaintext: &str) -> Vec<u32> {
    /*
        May return a Vec of binary representations of 
        each char in the &str plaintext.
     */
    match string_to_binary(plaintext) {
        Ok(binary) => binary,
        Err(_) => vec![0],
    }
}

fn binary_xor(plainbinary: Vec<u32>, keybinary: Vec<u32>) -> Vec<u32> {
    plainbinary
        .iter()
        .zip(keybinary.iter())
        .map(|(&a, &b)| a ^ b)  // XOR each element
        .collect::<Vec<u32>>()
}
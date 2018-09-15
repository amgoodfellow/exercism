use std::collections::HashMap;
const ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

const KEY: [char; 36] = [
    'z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h',
    'g', 'f', 'e', 'd', 'c', 'b', 'a','0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = plain.to_lowercase();
    let map: HashMap<_, _> = ALPHABET.iter().zip(KEY.iter()).collect();
    let mut result = String::new();
    let mut word_length = 0;
    for c in plain.chars() {
        if let Some(val) = map.get(&c) {
            result.push(*val.clone());
            word_length += 1;
            if word_length == 5 {
                result.push(' ');
                word_length = 0;
            }
        }
    }

    result.trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let map: HashMap<_, _> = KEY.iter().zip(ALPHABET.iter()).collect();
    let mut result = String::new();
    for c in cipher.chars() {
        if let Some(val) = map.get(&c) {
            result.push(val.clone().clone());
        }
    }

    result 
}

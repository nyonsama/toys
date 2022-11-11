//pub mod vigenere {

pub fn decode(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut plain: Vec<u8> = Vec::new();
    plain.reserve(text.len());
    let mut k = key.iter().cycle();
    for c in text {
        plain.push((c + 26 - k.next().unwrap()) % 26 + 0x61);
    }
    plain
}

pub fn encode(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cipher: Vec<u8> = Vec::new();
    cipher.reserve(text.len());
    let mut k = key.iter().cycle();
    for c in text {
        cipher.push((c - 0x61 + k.next().unwrap() - 0x61) % 26 + 0x61);
    }
    cipher
}
//}

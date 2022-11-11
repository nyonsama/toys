//pub mod column {
use std::collections::HashMap;

pub fn decode(text: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if text.len() % key.len() != 0 {
        return Err(String::from("Illegal length of text"));
    }

    let mut key_sorted = Vec::from(key);
    key_sorted.sort_unstable();
    let key_map: HashMap<_, _> = key_sorted.iter().zip(0..key.len()).collect();

    // let mut plain: Vec<u8> = (0..text.len() / key.len())
    //     .map(|offset| {
    //         text.get(
    //             (key.iter().map(|c| key_map.get(c).unwrap()).cycle())
    //                 .next()
    //                 .unwrap()
    //                 * height
    //                 + offset,
    //         )
    //         .unwrap()
    //         .clone()
    //     })
    //     .collect();

    let mut plain: Vec<u8> = Vec::new();
    plain.reserve(text.len());
    for i in 0..text.len() / key.len() {
        for k in key {
            plain.push(
                text.get(key_map.get(k).unwrap() * text.len() / key.len() + i)
                    .unwrap()
                    .clone(),
            );
        }
    }
    Ok(plain)
}

pub fn encode(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut text = Vec::from(text);
    if text.len() % key.len() != 0 {
        text.resize((text.len() / key.len() + 1) * key.len(), b'x');
    }

    let mut key_sorted = Vec::from(key);
    key_sorted.sort_unstable();
    let key_map: HashMap<_, _> = key_sorted.iter().zip(0..key.len()).collect();

    let mut cipher: Vec<u8> = Vec::new();
    for k in key {
        cipher.extend(
            text.iter()
                .skip(*key_map.get(k).unwrap())
                .step_by(key.len()),
        );
    }
    cipher
}
//}

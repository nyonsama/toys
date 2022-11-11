pub fn calc_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for byte in data {
        counts[*byte as usize] += 1;
    }
    counts.iter().fold(0f64, |acc, &count| {
        let p = count as f64 / data.len() as f64;
        match count {
            0 => acc,
            _ => acc - p * p.log2(),
        }
    })
}

/// Return a slice that no trailling '\0'
pub fn strip_0(buf: &[u8]) -> &[u8] {
    let mut i = buf.len();
    for &byte in buf.iter().rev() {
        if byte == b'\0' {
            i -= 1;
        } else {
            break;
        }
    }
    return &buf[..i];
}

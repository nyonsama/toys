#[test]
fn test_check() {
    println!("{}", brackets_match("a+(2+[2+2])"));
    println!("{}", brackets_match("a+[2+(2+2)]"));
    println!("{}", brackets_match(")(1+2)"));
    println!("{}", brackets_match("[8*9+(10+10)"));
}

pub fn brackets_match(exp: &str) -> bool {
    let is_pair = |l, r| match l {
        '(' => r == ')',
        '[' => r == ']',
        '{' => r == '}',
        _ => false,
    };
    let mut stack = Vec::new();
    let mut chars = exp.chars();
    loop {
        match chars.next() {
            Some(c) => match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => match stack.pop() {
                    Some(l) => match is_pair(l, c) {
                        false => return false,
                        true => {}
                    },
                    None => return false,
                },
                _ => {}
            },
            None => match stack.len() {
                0 => break,
                _ => return false,
            },
        }
    }
    true
}

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Json {}

#[derive(Debug)]
pub enum JsonType {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<JsonType>),
    Object(HashMap<String, JsonType>),
    Null,
}

#[derive(Debug, Clone, Copy)]
enum TokenType {
    LeftSquare,
    LeftCurly,
    RightSquare,
    RightCurly,
    Colon,
    Comma,
    Null,
    True,
    False,
    Number,
    String,
    Whitespace,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

lazy_static! {
    static ref TOKEN_REGEX: Vec<(Regex,TokenType)> = vec![
        (r"\A\[",TokenType::LeftSquare),
        (r"\A\]",TokenType::RightSquare),
        (r"\A\{",TokenType::LeftCurly),
        (r"\A\}",TokenType::RightCurly),
        (r"\A:",TokenType::Colon),
        (r"\A,",TokenType::Comma),
        (r"\Anull",TokenType::Null),
        (r"\Atrue",TokenType::True),
        (r"\Afalse",TokenType::False),
        (r"\A-?(0|([1-9][0-9]*))(\.[0-9]+)?((E|e)(-|\+)?[0-9]+)?",TokenType::Number), // number
        (r##"\A"(((.*?[^\\])?(\\{2})*(\\")?)|.*?[^\\"])""##,TokenType::String), // string
        (r##"\A[ \n\r\t]+"##,TokenType::Whitespace), // whitespace
    ]
    .iter()
    .map(|t| (Regex::new(t.0).unwrap(),t.1))
    .collect();
}

#[test]
fn test_regex() {
    let re = &TOKEN_REGEX[TOKEN_REGEX.len() - 2].0;
    println!(
        "{:?}",
        re.find(r##""aa\\\\\"\\\"asdf\\"\"z\\xc"r\\\"ed","value": "#f00"}"##)
    );
}

#[test]
fn test_tokenize() {
    let result = tokenize(
        r##"[
	{
		"color": "r\\\"ed",
		"value": "#f00"
	},
	{
		"color": "green",
		"value": "#0f0"
	},
	{
		"color": "blue",
		"value": "#00f"
	},
	{
		"color": "cyan",
		"value": "#0ff"
	},
	{
		"color": "mag\\\\\"enta",
		"value": "#f0f"
	},
	{
		"color": "yellow",
		"value": "#ff0"
	},
	{
		"color": "black",
		"value": "#000"
	}
]"##,
    )
    .unwrap();
    for r in result {
        println!("{:?}", r);
    }
}

fn tokenize(s: &str) -> Result<Vec<Token>, usize> {
    let mut result = Vec::new();
    let mut index = 0usize;
    while index < s.len() {
        let mut ok = false;
        for (regex, token_type) in TOKEN_REGEX.iter() {
            if let Some(i) = regex.shortest_match(&s[index..]) {
                match *token_type {
                    TokenType::Whitespace => (),
                    _ => {
                        result.push(Token {
                            token_type: *token_type,
                            value: Some(String::from(&s[index..index + i])),
                        });
                    }
                }
                index += i;
                ok = true;
                break;
            }
        }
        if !ok {
            return Err(index);
        }
    }
    Ok(result)
}

struct Parser<'a> {
    index: usize,
    tokens: &'a [Token],
}
impl<'a> Parser<'a> {
    fn parse_number(&mut self) -> Result<JsonType, String> {
        self.index += 1;
        let f = f64::from_str(self.tokens[self.index].value.as_ref().unwrap().as_str()).unwrap();
        Ok(JsonType::Number(f))
    }
    fn parse_string(&mut self) -> Result<JsonType, String> {
        self.index += 1;
        Ok(JsonType::String(format!("asdf")))
    }
    fn parse_true(&mut self) -> Result<JsonType, String> {
        self.index += 1;
        Ok(JsonType::Boolean(true))
    }
    fn parse_false(&mut self) -> Result<JsonType, String> {
        self.index += 1;
        Ok(JsonType::Boolean(false))
    }
    fn parse_null(&mut self) -> Result<JsonType, String> {
        self.index += 1;
        Ok(JsonType::Null)
    }
    fn parse_array(&mut self) -> Result<JsonType, String> {
        let mut array = Vec::new();
        // self.tokens[self.index] shuold be '['
        self.index += 1;
        while self.index < self.tokens.len() {
            match self.tokens[self.index].token_type {
                TokenType::String => array.push(self.parse_string()?),
                TokenType::Number => array.push(self.parse_number()?),
                TokenType::True => array.push(self.parse_true()?),
                TokenType::False => array.push(self.parse_false()?),
                TokenType::Null => array.push(self.parse_null()?),
                TokenType::LeftSquare => array.push(self.parse_array()?),
                TokenType::LeftCurly => array.push(self.parse_object()?),
                TokenType::RightSquare => {
                    self.index += 1;
                    return Ok(JsonType::Array(array));
                }
                _ => return Err(format!("Unexpected token{:?}", self.tokens[self.index])),
            }
            match self.tokens[self.index].token_type {
                TokenType::Comma => self.index += 1,
                TokenType::RightSquare => (),
                _ => return Err(format!("Unexpected token{:?}", self.tokens[self.index])),
            }
        }
        Err(format!("Unexpected EOF"))
    }
    fn parse_object(&mut self) -> Result<JsonType, String> {
        Ok(JsonType::Null)
    }

    pub fn parse(&mut self, tokens: &'a [Token]) -> Result<JsonType, String> {
        // status: start,singlevalue,
        self.tokens = tokens;
        let mut result = match tokens[self.index].token_type {
            TokenType::LeftSquare => self.parse_array()?,
            TokenType::LeftCurly => self.parse_object()?,
            TokenType::Number => self.parse_number()?,
            TokenType::String => self.parse_string()?,
            TokenType::True => JsonType::Boolean(true),
            TokenType::False => JsonType::Boolean(false),
            TokenType::Null => JsonType::Null,
            _ => {
                return Err(format!(
                    "Unexpected token {:?} at {}",
                    tokens[self.index], self.index
                ))
            }
        };

        if self.index < tokens.len() - 1 {
            return Err(format!(
                "Unexpected token {:?} at {}",
                tokens[self.index], self.index
            ));
        }
        Ok(result)
    }
}

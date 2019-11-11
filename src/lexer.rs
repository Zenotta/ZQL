use regex::Regex;
use crate::grammar::{ HeapKeyword, StackKeyword, get_heap_keyword, get_stack_keyword };

#[derive(Debug, Clone)]
pub enum LexToken {
    Paren(char),
    Op(String),
    Number(f64),
    Value(String),
    HeapKeyword(HeapKeyword),
    StackKeyword(StackKeyword)
}

#[derive(Debug, Clone)]
pub struct LexOutput {
    success: bool,
    tokens: Vec<LexToken>
}

/// Lex input string into a vector of lexical tokens
pub fn lex(input: &String) -> LexOutput {
    let input_vec = input.chars().collect::<Vec<char>>();
    let mut tokens = Vec::with_capacity(input_vec.len());
    let mut current_atom = Vec::with_capacity(input_vec.len());

    for character in input_vec {
        match character {
            ' ' => {
                // Take a space character as a break in token
                if current_atom.len() > 0 {
                    let next_token = lex_token(&current_atom);

                    tokens.push(next_token);
                    current_atom.clear();
                }
            }
            _ => {
                current_atom.push(character);
            }
        }
    }

    if !current_atom.is_empty() {
        let next_token = lex_token(&current_atom);

        tokens.push(next_token);
        current_atom.clear();
    }

    LexOutput::new(true, tokens)
}

/// Parse a single token in vector format to return a lexed output
fn lex_token(input: &Vec<char>) -> LexToken {
    let input_to_check: String = input.iter().collect();

    // Do a character check if it's a single character
    if input_to_check.len() == 1 {
        let char_to_check = input_to_check.chars().next().unwrap();

        match char_to_check {
            '*' | '-' | '+' | '/' | '^' | '<' | '>' | '=' => {
                return LexToken::Op(char_to_check.to_string());
            },
            '(' | ')' | '[' | ']' | '{' | '}' => {
                return LexToken::Paren(char_to_check);
            },
            '0'...'9' => {
                return LexToken::Number(char_to_check.to_digit(10).unwrap_or(0) as f64);
            },
            _ => {
                return LexToken::Value(char_to_check.to_string());
            }
        }
    }

    // Check for heap keyword
    if let Some(heap_keyword) = get_heap_keyword(&input_to_check) {
        return LexToken::HeapKeyword(heap_keyword);
    }

    // Check for stack keyword
    if let Some(stack_keyword) = get_stack_keyword(&input_to_check) {
        return LexToken::StackKeyword(stack_keyword);
    }
    
    // Check for float value
    if Regex::new(r"[0-9]+\.[0-9]+").unwrap().is_match(&input_to_check) {
        return LexToken::Number(input_to_check.parse::<f64>().unwrap());
    }

    // TODO: Check for keyword matches

    LexToken::Value(input_to_check)
}


/// Inserts breaks in parentheses
pub fn insert_breaks(script: &String) -> String {
    let script_chars = script.chars().collect::<Vec<char>>();
    let mut script_with_breaks = Vec::new();

    for c in script_chars {
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' => {
                script_with_breaks.push(' ');
                script_with_breaks.push(c);
                script_with_breaks.push(' ');
            },
            _ => {
                script_with_breaks.push(c);
            }
        }
    }

    script_with_breaks.into_iter().collect()
}



impl LexOutput {
    /// Creates a new instance of a LexOutput
    pub fn new(success: bool, tokens: Vec<LexToken>) -> LexOutput {
        LexOutput {
            success: success,
            tokens: tokens
        }
    }
}
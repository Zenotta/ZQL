use regex::Regex;
use crate::grammar::{ HeapKeyword, StackKeyword, get_heap_keyword, get_stack_keyword };

/// A data structure for the types of lexical tokens
#[derive(Debug, Clone, PartialEq)]
pub enum LexToken {
    Punc(char),
    Op(char),
    Number(f64),
    Value(String),
    HeapKeyword(HeapKeyword),
    StackKeyword(StackKeyword)
}

/// A builder for lexical token representations
#[derive(Debug, Clone)]
pub struct Lexer {
    pub tokens: Vec<LexToken>
}

impl Lexer {

    /// Generates a new Lexer instance for lexing ZQL scripts
    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new()
        }
    }

    /// Lex input string into a vector of lexical tokens
    /// 
    /// ### Arguments
    /// 
    /// * `input`   - The input script to lex
    pub fn lex(&mut self, input: &String) {
        let input_with_breaks = self.insert_breaks(input);
        let input_vec = input_with_breaks.chars().collect::<Vec<char>>();
        let mut tokens = Vec::with_capacity(input_vec.len());
        let mut current_atom = Vec::with_capacity(input_vec.len());

        for character in input_vec {
            match character {
                ' ' => {
                    // Take a space character as a break in token
                    if current_atom.len() > 0 {
                        let next_token = self.lex_token(&current_atom);

                        tokens.push(next_token);
                        current_atom.clear();
                    }
                }
                _ => {
                    current_atom.push(character);
                }
            }
        }

        // Assume the remaining atom is a full token
        if !current_atom.is_empty() {
            tokens.push(self.lex_token(&current_atom));
        }

        self.tokens = tokens;
    }

    /// Parse a single token in vector format to return a lexed output
    /// 
    /// ### Arguments
    /// 
    /// * `input`   - The set of characters representing a single token
    fn lex_token(&self, input: &Vec<char>) -> LexToken {
        let input_to_check: String = input.iter().collect();

        // Do a character check if it's a single character
        if input_to_check.len() == 1 {
            let char_to_check = input_to_check.chars().next().unwrap();

            match char_to_check {
                '*' | '-' | '+' | '/' | '^' | '<' | '>' | '=' => {
                    return LexToken::Op(char_to_check);
                },
                '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    return LexToken::Punc(char_to_check);
                },
                '0'...'9' => {
                    return LexToken::Number(char_to_check.to_digit(10).unwrap_or(0) as f64);
                },
                _ => {
                    return LexToken::Value(char_to_check.to_string());
                }
            }
        }

        // TODO: Abstract this correctly

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

        LexToken::Value(input_to_check)
    }

    /// Inserts breaks in parentheses and other punctuation. This is needed 
    /// to parse punctuation separately from script content.
    /// 
    /// ### Arguments
    /// 
    /// * `script`  - The script to insert breaks into
    fn insert_breaks(&self, script: &String) -> String {
        let script_chars = script.chars().collect::<Vec<char>>();
        let mut script_with_breaks = Vec::new();

        for c in script_chars {
            match c {
                '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    script_with_breaks.append(&mut vec![' ', c, ' ']);
                },
                '\n' => {
                    continue;
                }
                _ => {
                    script_with_breaks.push(c);
                }
            }
        }

        script_with_breaks.into_iter().collect()
    }
}
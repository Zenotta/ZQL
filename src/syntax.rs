use crate::lexer::{ Lexer, LexToken };
use crate::grammar::GrammarAtom;

/// A representation of a token node in a syntax tree
#[derive(Debug, Clone)]
struct ParseNode {
    pub entry: GrammarAtom,
    pub children: Vec<ParseNode>
}

/// A builder for parsing lexed language
#[derive(Debug, Clone)]
pub struct Parser;


impl ParseNode {
    /// Creates a new node for parsing
    pub fn new() -> ParseNode {
        ParseNode {
            entry: GrammarAtom::Value(String::from("")),
            children: Vec::new()
        }
    }
}

impl Parser {
    /// Parses a full script
    /// 
    /// ### Arguments
    /// 
    /// * `input`   - The input script to parse
    pub fn parse_script(&self, input: &String) {
        let mut lex_inst = Lexer::new();
        lex_inst.lex(input);

        let tokens = lex_inst.tokens;
        // self.parse_expression(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        //     Ok(n)
        // } else {
        //     Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
        // })
    }

    /// Parses a heap expression
    /// 
    /// ### Arguments
    /// 
    /// * `tokens`      - Tokens of the expression to parse
    /// * `position`    - Index position to parse
    fn parse_heap_expression(&self, tokens: &Vec<LexToken>, position: usize) -> Result<(ParseNode, usize), String> {
        let c: &LexToken = tokens.get(position).unwrap();

        match c {
            LexToken::Number(n) => {
                let mut node = ParseNode::new();
                node.entry = GrammarAtom::Number(n.clone());
                Ok((node, position + 1))
            }
            LexToken::HeapKeyword(k) => {
                let mut node = ParseNode::new();
                node.entry = GrammarAtom::HeapKeyword(k.clone());
                Ok((node, position + 1))
            }
            LexToken::StackKeyword(k) => {
                let mut node = ParseNode::new();
                node.entry = GrammarAtom::StackKeyword(k.clone());
                Ok((node, position + 1))
            }
            LexToken::Value(v) => {
                let mut node = ParseNode::new();
                node.entry = GrammarAtom::Value(v.clone());
                Ok((node, position + 1))
            }
            LexToken::Punc(p) => {
                match p {
                    '(' | '[' | '{' => {
                        let mut paren = ParseNode::new();
                        Ok((paren, position + 1))
                    }
                    _ => Err(format!("Expected paren at {} but found {:?}", position, p)),
                }
            }
            _ => {
                Err(format!("Unexpected token {:?}, expected paren or number", {
                    c
                }))
            }
        }
    }

    /// Parses an expression
    /// 
    /// ### Arguments
    /// 
    /// * `tokens`      - Tokens of the expression to parse
    /// * `position`    - Index position
    fn parse_expression(&self, tokens: &Vec<LexToken>, position: usize) {
        //let (node_term, next_pos) = try!(self.parse_term(tokens, position));
        let c = tokens.get(position); // should be next_pos

        match c {
            Some( LexToken::Op(';') ) => {
                let mut sum = ParseNode::new();
                sum.entry = GrammarAtom::HeapExpression;

            }
            _ => {
                // we have just the summand production, nothing more.
                //Ok((node_term, next_pos))
            }
        }
    }
}

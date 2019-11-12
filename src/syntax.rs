#![allow(dead_code)]

use crate::grammar::{GrammarAtom, HeapKeyword, StackKeyword};
use crate::lexer::{LexToken, Lexer};

/// A representation of a token node in a syntax tree
#[derive(Debug, Clone)]
pub struct ParseNode {
    pub entry: GrammarAtom,
    pub children: Vec<ParseNode>,
}

/// A builder for parsing lexed language
#[derive(Debug, Clone)]
pub struct Parser;

impl ParseNode {
    /// Creates a new node for parsing
    pub fn new() -> ParseNode {
        ParseNode {
            entry: GrammarAtom::Value(String::from("")),
            children: Vec::new(),
        }
    }
}

impl Parser {
    /// Parses a full script
    ///
    /// ### Arguments
    ///
    /// * `input`   - The input script to parse
    pub fn parse_script(&self, input: &String) -> Result<(ParseNode), String> {
        let mut lex_inst = Lexer::new();
        lex_inst.lex(input);

        let tokens = lex_inst.tokens;
        let token_len = tokens.len();

        let mut position = 0;
        let mut syntax_tree = ParseNode::new();
        syntax_tree.entry = GrammarAtom::HeapExpression;

        while position < token_len {
            let c: &LexToken = tokens
                .get(position)
                .ok_or(
                    String::from("Tried to access a lexical token, but the index requested doesn't exist in the list")
                )?;

            match c {
                LexToken::HeapKeyword(k) => {
                    let (node, next_position) = if k == &HeapKeyword::Stack {
                        self.parse_stack_expression(&tokens, position + 1)?
                    } else {
                        self.parse_heap_expression(&tokens, position + 1)?
                    };

                    syntax_tree.children.push(node);
                    position = next_position;
                }
                LexToken::StackKeyword(_k) => {
                    return Err(format!(
                        "The command or value '{:?}' is a Stack keyword and needs to be declared in a Stack",
                        { c }
                    ));
                }
                _ => {
                    return Err(format!(
                        "The command or value '{:?}' at position {} is syntactically invalid",
                        { c },
                        position
                    ));
                }
            }
        }

        Ok(syntax_tree)
    }

    /// Generic parse method for expressions
    ///
    /// ### Arguments
    ///
    ///
    ///
    fn parse_expression(
        &self,
        tokens: &Vec<LexToken>,
        position: usize,
    ) -> Result<(ParseNode, usize), String> {
        let mut node = ParseNode::new();
        let c: &LexToken = tokens.get(position).ok_or(String::from(
            "Tried to access a lexical token, but the index requested doesn't exist in the list",
        ))?;

        match c {
            LexToken::Number(n) => {
                node.entry = GrammarAtom::Number(n.clone());
            }
            LexToken::StackKeyword(k) => {
                node.entry = GrammarAtom::StackKeyword(k.clone());
            }
            LexToken::Value(v) => {
                node.entry = GrammarAtom::Value(v.clone());
            }
            LexToken::HeapKeyword(k) => {
                node.entry = GrammarAtom::HeapKeyword(k.clone());
            }
            _ => {
                return Err(format!(
                    "The command or value '{:?}' is not valid in ZQL",
                    { c }
                ));
            }
        };

        Ok((node, position + 1))
    }

    /// Parses a heap expression
    ///
    /// ### Arguments
    ///
    /// * `tokens`              - Tokens of the expression to parse
    /// * `position`            - Index position to parse
    /// * `start_expression`    - Whether this is a new expression or not
    fn parse_heap_expression(
        &self,
        tokens: &Vec<LexToken>,
        position: usize,
    ) -> Result<(ParseNode, usize), String> {
        let mut heap_expression = ParseNode::new();
        let mut mut_position = position;
        let mut expression_complete = false;

        heap_expression.entry = GrammarAtom::HeapExpression;

        while !expression_complete {
            let c: &LexToken = tokens.get(mut_position).ok_or(String::from(
                "Tried to access a lexical token, but the index requested doesn't exist in the list",
            ))?;

            match c {
                LexToken::StackKeyword(k) => {
                    return Err(format!(
                        "INVALID: The command or value '{:?}' at position {} is a Stack keyword, but used in a ZQL Heap",
                        { k },
                        mut_position
                    ));
                }
                LexToken::Punc(p) => {
                    let mut node = ParseNode::new();
                    node.entry = GrammarAtom::Punc(*p);
                    heap_expression.children.push(node);
                    mut_position += 1;

                    // TODO: Handle bracket punctuation

                    if p == &';' {
                        expression_complete = true;
                    }
                }
                _ => {
                    let (node, next_position) = self.parse_expression(tokens, mut_position)?;
                    heap_expression.children.push(node);
                    mut_position = next_position;
                }
            };
        }

        Ok((heap_expression, mut_position))
    }

    /// Parses a stack expression
    ///
    /// ### Arguments
    ///
    /// * `tokens`      - Tokens of the expression to parse
    /// * `position`    - Index position
    /// * `start_expression`    - Whether this is a new expression or not
    fn parse_stack_expression(
        &self,
        tokens: &Vec<LexToken>,
        position: usize,
    ) -> Result<(ParseNode, usize), String> {
        let mut stack_expression = ParseNode::new();
        let mut mut_position = position;
        let mut expression_complete = false;

        stack_expression.entry = GrammarAtom::StackExpression;

        while !expression_complete {
            let c: &LexToken = tokens.get(mut_position).ok_or(String::from(
                "Tried to access a lexical token, but the index requested doesn't exist in the list",
            ))?;

            match c {
                LexToken::HeapKeyword(k) => {
                    return Err(format!(
                        "INVALID: The command or value '{:?}' at position {} is a Heap keyword, but used in a ZQL Stack",
                        { k },
                        mut_position
                    ));
                }
                LexToken::Punc(p) => {
                    match p {
                        '[' | ']' => {
                            let mut node = ParseNode::new();
                            node.entry = GrammarAtom::Punc(*p);
                            stack_expression.children.push(node);
                            mut_position += 1;

                            // Stack expression is considered complete
                            if p == &']' {
                                expression_complete = true;
                            }
                        }
                        _ => {
                            return Err(format!(
                                "INVALID: Only '[' and ']' are valid in a Stack. The syntax '{:?}' at position {} is invalid",
                                { p },
                                mut_position
                            ));
                        }
                    }
                }
                _ => {
                    let (node, next_position) = self.parse_expression(tokens, mut_position)?;
                    stack_expression.children.push(node);
                    mut_position = next_position;
                }
            };
        }

        Ok((stack_expression, mut_position))
    }
}

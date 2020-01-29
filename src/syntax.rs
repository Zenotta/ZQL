#![allow(dead_code)]

use crate::grammar::{GrammarAtom, HeapKeyword, OpAtom, find_op_grammar_atom};
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

                    let mut heap_keyword = ParseNode::new();
                    heap_keyword.entry = GrammarAtom::HeapKeyword(k.clone());
                    heap_keyword.children.push(node);

                    syntax_tree.children.push(heap_keyword);

                    position = next_position;
                }
                LexToken::StackKeyword(_k) => {
                    return Err(format!(
                        "The command or value {:?} is a Stack keyword and needs to be declared in a Stack",
                        { c }
                    ));
                }
                _ => {
                    return Err(format!(
                        "The command or value {:?} at position {} is syntactically invalid",
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
    /// * `tokens`              - Tokens of the expression to parse
    /// * `position`            - Index position to parse
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
                return Err(format!("The command or value {:?} is not valid in ZQL", {
                    c
                }));
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
    fn parse_heap_expression(
        &self,
        tokens: &Vec<LexToken>,
        position: usize,
    ) -> Result<(ParseNode, usize), String> {
        let mut heap_expression = ParseNode::new();
        let mut mut_position = position;

        heap_expression.entry = GrammarAtom::HeapExpression;

        while mut_position < tokens.len() {
            let c: &LexToken = tokens.get(mut_position).ok_or(format!(
                "Tried to access a lexical token at position {} in a HEAP EXPR, but the index requested doesn't exist in the list", mut_position
            ))?;

            match c {
                LexToken::StackKeyword(k) => {
                    return Err(format!(
                        "INVALID: The command or value {:?} at position {} is a Stack keyword, but is being used in a ZQL Heap",
                        { k },
                        mut_position
                    ));
                }
                LexToken::Punc(p) => {
                    let mut node = ParseNode::new();
                    node.entry = GrammarAtom::Punc(*p);

                    // TODO: Handle bracket punctuation for child expressions
                    match p {
                        &'{' | &'}' | &'[' | &']' | &'(' | &')' | &';' => {
                            mut_position += 1;

                            if p == &';' {
                                break;
                            }
                        }
                        _ => {
                            return Err(format!(
                                "INVALID: The syntax {:?} at position {} is invalid syntax in a Heap",
                                { p },
                                mut_position
                            ));
                        }
                    };

                    heap_expression.children.push(node);
                }
                LexToken::HeapKeyword(k) => {
                    let (node, next_position) = if k == &HeapKeyword::Stack {
                        self.parse_stack_expression(&tokens, mut_position + 1)?
                    } else {
                        self.parse_heap_expression(&tokens, mut_position + 1)?
                    };

                    let mut heap_keyword = ParseNode::new();
                    heap_keyword.entry = GrammarAtom::HeapKeyword(k.clone());
                    heap_keyword.children.push(node);
                    heap_expression.children.push(heap_keyword);

                    mut_position = next_position;
                }
                LexToken::Op(o) => {
                    let node = find_op_grammar_atom(o);
                    
                    mut_position += 1;
                    heap_expression.children.push(node);
                }
                _ => {
                    // Assume anything else can be generically parsed
                    // Errors and invalid tokens will be caught in the generic parsing method
                    let (node, next_position) = self.parse_expression(tokens, mut_position)?;
                    heap_expression.children.push(node);
                    mut_position = next_position;
                }
            };
        }

        Ok((heap_expression, mut_position))
    }

    /// Parses a stack expression. Stack expressions cannot nest, so a simple
    /// check on the closing bracket is enough to conclude the parsing.
    ///
    /// ### Arguments
    ///
    /// * `tokens`              - Tokens of the expression to parse
    /// * `position`            - Index position
    fn parse_stack_expression(
        &self,
        tokens: &Vec<LexToken>,
        position: usize,
    ) -> Result<(ParseNode, usize), String> {
        let mut stack_expression = ParseNode::new();
        let mut mut_position = position;

        stack_expression.entry = GrammarAtom::StackExpression;

        loop {
            let c: &LexToken = tokens.get(mut_position).ok_or(String::from(
                "Tried to access a lexical token in a STACK EXPR, but the index requested doesn't exist in the list",
            ))?;

            match c {
                LexToken::HeapKeyword(k) => {
                    return Err(format!(
                        "INVALID: The command or value {:?} at position {} is a Heap keyword, but is being used in a ZQL Stack",
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
                                break;
                            }
                        }
                        _ => {
                            return Err(format!(
                                "INVALID: '[' and ']' are the only valid brackets in a Stack. The syntax {:?} at position {} is invalid",
                                { p },
                                mut_position
                            ));
                        }
                    }
                }
                LexToken::Op(o) => {
                    let node = find_op_grammar_atom(o);
                    
                    mut_position += 1;
                    stack_expression.children.push(node);
                }
                _ => {
                    let (node, next_position) = self.parse_expression(tokens, mut_position)?;
                    stack_expression.children.push(node);
                    mut_position = next_position;
                }
            };
        }

        println!("");
        println!("STACK EXPRESSION: {:?}", stack_expression);
        println!("");

        Ok((stack_expression, mut_position))
    }
}

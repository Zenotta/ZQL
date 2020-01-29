use crate::syntax::ParseNode;

/// Definitions of grammar items
#[derive(Debug, Clone, PartialEq)]
pub enum GrammarAtom {
    Number(f64),
    Value(String),
    Punc(char),
    Op(OpAtom),
    HeapExpression,
    StackExpression,
    StackKeyword(StackKeyword),
    HeapKeyword(HeapKeyword)
}

/// Definitions of operation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpAtom {
    To,
    Multiply,
    Add,
    Subtract,
    Divide,
    LessThan,
    GreaterThan,
    EqualTo
}

/// Definitions of stack keywords
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackKeyword {
    New,
    Znt,
    Sdl,
    And,
    Pay,
    Get,
    Where,
    Create,
    Update,
    Delete,
    Transact,
    Who,
    Amount,
    Encoding,
    In,
    Address
}

/// Definitions of heap keywords
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeapKeyword {
    If,
    Else,
    When,
    While,
    For,
    In,
    Stack,
    Set
}

/// Get the heap keyword enum for the provided keyword.
/// Rust does not support string-based enums like C, requiring this workaround.
///
/// ### Arguments
/// 
/// * `keyword` - The keyword to convert to enum format
pub fn get_heap_keyword(keyword: &str) -> Option<HeapKeyword> {
    match keyword {
        "if" =>     { Some(HeapKeyword::If) }
        "else" =>   { Some(HeapKeyword::Else) }
        "when" =>   { Some(HeapKeyword::When) }
        "while" =>  { Some(HeapKeyword::While) }
        "for" =>    { Some(HeapKeyword::For) }
        "in" =>     { Some(HeapKeyword::In) }
        "stack" =>  { Some(HeapKeyword::Stack) }
        "set" =>    { Some(HeapKeyword::Set) }
        _ =>        { None }
    }
}

/// Get the stack keyword enum for the provided keyword.
/// Rust does not support string-based enums like C, requiring this workaround.
/// 
/// ### Arguments
/// 
/// * `keyword` - The keyword to convert to enum format
pub fn get_stack_keyword(keyword: &str) -> Option<StackKeyword> {
    match keyword {
        "NEW" =>      { Some(StackKeyword::New) }
        "AND" =>      { Some(StackKeyword::And) }
        "ZNT" =>      { Some(StackKeyword::Znt) }
        "SDL" =>      { Some(StackKeyword::Sdl) }
        "PAY" =>      { Some(StackKeyword::Pay) }
        "GET" =>      { Some(StackKeyword::Get) }
        "WHERE" =>    { Some(StackKeyword::Where) }
        "CREATE" =>   { Some(StackKeyword::Create) }
        "UPDATE" =>   { Some(StackKeyword::Update) }
        "DELETE" =>   { Some(StackKeyword::Delete) }
        "TRANSACT" => { Some(StackKeyword::Transact) }
        "WHO" =>      { Some(StackKeyword::Who) }
        "AMOUNT" =>   { Some(StackKeyword::Amount) }
        "ENCODING" => { Some(StackKeyword::Encoding) }
        "IN" =>       { Some(StackKeyword::In) }
        _ =>          { None }
    }
}

/// Performs a match against an operator to find a 
/// grammar atom for it. This is a utility function for the parser.
/// 
/// ### Arguments
/// 
/// * `operator`    - Operator to find a grammar atom for
pub fn find_op_grammar_atom(operator: &String) -> ParseNode {
    let mut node = ParseNode::new();

    match operator.as_str() {
        "*" => {
            node.entry = GrammarAtom::Op(OpAtom::Multiply);
        }
        "+" => {
            node.entry = GrammarAtom::Op(OpAtom::Add);
        }
        "-" => {
            node.entry = GrammarAtom::Op(OpAtom::Subtract);
        }
        "/" => {
            node.entry = GrammarAtom::Op(OpAtom::Divide);
        }
        "<" => {
            node.entry = GrammarAtom::Op(OpAtom::LessThan);
        }
        ">" => {
            node.entry = GrammarAtom::Op(OpAtom::GreaterThan);
        }
        "=" => {
            node.entry = GrammarAtom::Op(OpAtom::To);
        }
        "==" => {
            node.entry = GrammarAtom::Op(OpAtom::EqualTo);
        }
        _ => {
            panic!("Unknown operator found: {}", operator);
        }
    }

    return node;
}
/// Definitions of grammar items
#[derive(Debug, Clone, PartialEq)]
pub enum GrammarAtom {
    Expression,
    Number(f64),
    Value(String),
    Punc(char),
    HeapExpression,
    StackExpression,
    StackKeyword(StackKeyword),
    HeapKeyword(HeapKeyword)
}

/// Definitions of operation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpAtom {
    Multiply,
    Add,
    Subtract,
    Divide,
    Power(i64),
    Root(i64),
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
    Pay,
    Get,
    Where,
    Create,
    Update,
    Delete,
    Transact,
    Who
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
/// Rust does not support string-based enums like C, requiring this workaround.Lexer
/// 
/// ### Arguments
/// 
/// * `keyword` - The keyword to convert to enum format
pub fn get_stack_keyword(keyword: &str) -> Option<StackKeyword> {
    match keyword {
        "NEW" =>      { Some(StackKeyword::New) }
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
        _ =>          { None }
    }
}

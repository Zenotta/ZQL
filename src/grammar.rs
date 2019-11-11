#[derive(Debug, Clone)]
enum GrammarAtom {
    Atom,
    Expression,
    Operator(OpAtom)
}

#[derive(Debug, Clone)]
pub enum KeywordAtom {
    Stack(StackKeyword),
    Heap(HeapKeyword)
}

#[derive(Debug, Clone)]
enum OpAtom {
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum HeapKeyword {
    If,
    Else,
    When,
    While,
    For,
    In
}

/// Get the heap keyword enum for the provided keyword.
/// Rust does not support string-based enums like C, requiring this workaround
pub fn get_heap_keyword(keyword: &str) -> Option<HeapKeyword> {
    match keyword {
        "if" =>     { Some(HeapKeyword::If) }
        "else" =>   { Some(HeapKeyword::Else) }
        "when" =>   { Some(HeapKeyword::When) }
        "while" =>  { Some(HeapKeyword::While) }
        "for" =>    { Some(HeapKeyword::For) }
        "in" =>     { Some(HeapKeyword::In) }
        _ =>        { None }
    }
}

/// Get the stack keyword enum for the provided keyword.
/// Rust does not support string-based enums like C, requiring this workaround
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
        "TRANSACT" => { Some(StackKeyword::Delete) }
        "WHO" =>      { Some(StackKeyword::Who) }
        _ =>          { None }
    }
}

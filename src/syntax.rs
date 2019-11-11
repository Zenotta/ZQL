use grammar::{ GrammarAtom };

#[derive(Debug, Clone)]
struct ParseNode {
    entry: GrammarAtom,
    children: Vec<ParseNode>
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            entry: GrammarAtom::Atom,
            children: Vec::new()
        }
    }
}
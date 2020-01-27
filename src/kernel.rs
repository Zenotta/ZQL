/// ZQL is required to simulate a kernel-like structure in order to execute its code

use crate::grammar::StackKeyword;
use crate::syntax::ParseNode;


/// Kernel instance
#[derive(Debug, Clone)]
pub struct Kernel;

impl Kernel {
    /// Parse through a stack expression
    pub fn parse_stack_expression(stack_expression: &ParseNode) {
        
    }
}

/// ZQL is required to simulate a kernel-like structure in order to execute its code

use std::collections::HashMap;
use crate::compiler::Compiler;


/// Kernel instance
#[derive(Debug, Clone)]
pub struct Kernel(pub Compiler);


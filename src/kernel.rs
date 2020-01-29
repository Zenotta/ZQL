/// ZQL is required to simulate a kernel-like structure in order to execute its code

use std::collections::HashMap;
use crate::grammar::{GrammarAtom, StackKeyword, HeapKeyword, OpAtom};
use crate::syntax::ParseNode;


/// Assignment value union workaround (unions are unsafe and full of crap)
#[derive(Debug, Clone)]
pub enum AssignmentValue {
    Text(String),
    Number(f64),
    Address(String)
}

/// A generic model for execution
#[derive(Debug, Clone)]
pub struct ExecutionModel {
    pub assignments: HashMap<String, AssignmentValue>
}


/// Kernel instance
#[derive(Debug, Clone)]
pub struct Kernel(pub ExecutionModel);



impl ExecutionModel {
    /// Instantiates a new execution model
    pub fn new() -> ExecutionModel {
        ExecutionModel {
            assignments: HashMap::new()
        }
    }
}

impl Kernel {

    /// Creates a new Kernel instance
    pub fn new() -> Kernel {
        Kernel(ExecutionModel::new())
    } 

    /// Parse through a heap expression
    /// 
    /// ### Arguments
    /// 
    /// * `heap_expression` - Heap expression to parse
    pub fn parse_heap_expression(&mut self, heap_expression: &ParseNode) {
        if heap_expression.entry == GrammarAtom::HeapExpression {
            for node in &heap_expression.children {

                match node.entry {
                    GrammarAtom::HeapKeyword(HeapKeyword::Stack) => {
                        self.parse_stack_expression(node);
                    }
                    GrammarAtom::HeapKeyword(HeapKeyword::Set) => {
                        self.perform_assignment(&node.children);
                    }
                    _ => {
                        panic!("Unknown or unsupported Heap expression");
                    }
                }
            }

        } else {
            panic!("Heap expression not declared with 'HeapExpression' grammar atom. Assuming this is an error and quitting...");
        }
    }

    /// Parse through a stack expression
    /// 
    /// ### Arguments
    /// 
    /// * `stack_expression`    - Stack expression to parse
    pub fn parse_stack_expression(&mut self, stack_expression: &ParseNode) {
        if stack_expression.entry == GrammarAtom::HeapKeyword(HeapKeyword::Stack) {
            // continue as normal
            

        } else {
            panic!("Stack expression provided doesn't begin with a STACK keyword. Assuming this is an error and quitting...");
        }
    }

    /// Performs an assignment operation for the execution model
    /// 
    /// ### Arguments
    /// 
    /// * `expression` - Assignment expression to parse and perform
    fn perform_assignment(&mut self, expression: &Vec<ParseNode>) {
        // Only 1 ParseNode should be present in an assignment expression
        if expression.len() == 1 {
            let full_expression = &expression[0].children;

            // An assignment expr should have left hand side, a To operator, and a right hand side
            if full_expression.len() > 3 {
                let mut right_hand_side = 0;

                for (index, atom) in full_expression.iter().enumerate() {
                    if atom.entry == GrammarAtom::Op(OpAtom::To) {
                        right_hand_side = index + 1;
                    }
                }

                let right_hand_side_value = self.calculate_expression(&full_expression[right_hand_side..]);
                
                match &full_expression[0].entry {
                    GrammarAtom::Value(key) => {
                        self.0.assignments.insert(key.to_string(), right_hand_side_value);
                    }
                    _ => {
                        panic!("Left hand side of assignment not valid");
                    }
                }

            } else {
                // It's a simple assignment process
                match &full_expression[0].entry {
                    GrammarAtom::Value(key) => {
                        let value = match &full_expression[2].entry {
                            GrammarAtom::Value(v) => AssignmentValue::Text(v.clone()),
                            GrammarAtom::Number(v) => AssignmentValue::Number(v.clone()),
                            _ => {
                                panic!("Unsupported or invalid right hand side of variable assignment: {:?}", full_expression[2].entry);
                            }
                        };
    
                        self.0.assignments.insert(key.to_string(), value);
                    }
                    _ => {
                        panic!("Left hand side of assignment not valid");
                    }
                }
            }

        } else {
            panic!("Assigning a variable failed. There should be exactly 1 expression in SET statement");
        }
    } 

    /// Calculates an arbitrary mathematical expression to return a single output value
    /// 
    /// ### Arguments
    /// 
    /// * `expression`  - Expression to calculate
    fn calculate_expression(&self, expression: &[ParseNode]) -> AssignmentValue {
        let mut value = 0.0;
        let mut previous_operator: Option<OpAtom> = None;

        for atom in expression {
            match &atom.entry {
                GrammarAtom::Number(a) => {
                    if previous_operator.is_none() {
                        value += a;
                    } else {
                        self.update_value_from_operator(&mut value, a, &previous_operator.unwrap());
                        previous_operator = None;
                    }
                }
                GrammarAtom::Op(a) => {
                    previous_operator = Some(a.clone());
                }
                GrammarAtom::Value(a) => {
                    // In the case of a generic value, we assume it is a previously assigned variable
                    let value_from_model = self.0.assignments.get(a);
                    
                    if value_from_model.is_some() {
                        let final_value_from_model = match value_from_model.unwrap() {
                            AssignmentValue::Number(f) => f,
                            _ => { panic!("Value from script model is invalid"); }
                        };

                        self.update_value_from_operator(&mut value, final_value_from_model, &previous_operator.unwrap());
                        previous_operator = None;

                    } else {
                        println!("");
                        println!("Variable '{}' is not assigned", a);
                        println!("");

                        panic!("Variable '{}' is not assigned", a);
                    }
                }
                _ => {
                    panic!("Grammar atom '{:?}' for expression calculation invalid or unsupported", atom.entry);
                }
            }
        }

        AssignmentValue::Number(value)
    }

    /// Updates the current value depending on the previous operator
    /// 
    /// ### Arguments
    /// 
    /// * `current_value`   - Current value to update
    /// * `new_value`       - New value to update the current value with
    /// * `prev_operator`   - The previous operator, to determine the nature of the update
    fn update_value_from_operator(&self, current_value: &mut f64, new_value: &f64, prev_operator: &OpAtom) {
        match prev_operator {
            OpAtom::Add => {
                *current_value += new_value;
            }
            OpAtom::Multiply => {
                *current_value *= new_value;
            }
            OpAtom::Subtract => {
                *current_value -= new_value;
            }
            OpAtom::Divide => {
                *current_value /= new_value;
            }
            _ => {
                println!("");
                println!("Cannot calculate an updated value due to invalid operator: {:?}", prev_operator);
                println!("");

                panic!("Cannot calculate an updated value due to invalid operator: {:?}", prev_operator);
            }
        }
    }
}

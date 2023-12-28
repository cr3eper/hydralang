pub mod expression;
pub mod expression_builder;
pub mod function;
pub mod symbol_table;
pub mod error;

pub use expression::Expression;




pub mod constraint {

    #[derive(Clone)]
    pub enum Constraint {
        Range(i64, i64),
        Type(String)
    }

}

pub use constraint::Constraint;
pub use script::Script;





pub mod script {
    use std::collections::HashMap;

    use crate::{parsing::parser::parse_script, visitor::{DefaultSimplifyVisitor, ExpressionModfierVisitor}};

    use super::{function::{FunctionCollection, FunctionDef}, Expression, error::DSLError};


    #[derive(Clone)]
    pub struct Script {
        function_defs: HashMap<String, FunctionCollection>,
        expressions: Vec<Expression>
    }


    impl Script {

        pub fn new(function_defs: Vec<FunctionDef>, expressions: Vec<Expression>) -> Self {
            let mut script = Script { function_defs: HashMap::new(), expressions };

            for function_def in function_defs {
                script.add_function_def(function_def);
            }

            script
        }

        pub fn add_function_def(&mut self, function_def: FunctionDef) {
            
            let name = function_def.get_name().clone();

            if let Some(function_collection) = self.function_defs.get_mut(&name) {
                function_collection.add_function_def(function_def);
            } else {
                let mut function_collection = FunctionCollection::new(name.clone());
                function_collection.add_function_def(function_def);
                self.function_defs.insert(name, function_collection);
            }
        }

        pub fn get_function_defs(&self) -> Vec<FunctionDef> {
            let mut result = Vec::new();
            for collection in self.function_defs.values() {
                for func in collection.get_function_defs() {
                    result.push(func);
                }
            }

            result
        }

        pub fn get_expression(&self, index: usize) -> Option<&Expression> {
            self.expressions.get(index)
        }

        pub fn add_expression_evaluation(&mut self, expression: Expression) {
            self.expressions.push(expression);
        }

        pub fn parse(input: &str) -> Result<Self, DSLError> {
            parse_script(input)
        }

        pub fn exec_function(&self, name: &str, args: Vec<Expression>) -> Option<Expression> {
            self.function_defs.get(&name.to_string())?.try_apply(&args)
        }

        pub fn merge(&mut self, other: &Self) {
            for f in other.get_function_defs() {
                self.add_function_def(f); // TODO: Figure out when functions should be overridden vs adjacent, for now newly added functions can never replace old ones
            }
            self.expressions.append(&mut other.expressions.clone())
        }

        pub fn run(&mut self) {
            for line in 0..self.expressions.len() {
                let mut simplify = DefaultSimplifyVisitor::new(self.clone()); //TODO: Fix dumb clone
                let new_expr = simplify.visit(self.expressions.get(line).unwrap().clone());
                self.expressions[line] = new_expr;
            }
        }

    }


    impl ToString for Script {
    
        fn to_string(&self) -> String {
            
            let mut result = String::new();
    
            for func_collection in self.function_defs.values() {
                result.push_str(func_collection.to_string().as_str());
                result.push_str("\n");
            }
    
            for expr in &self.expressions {
                result.push_str(expr.to_string().as_str());
                result.push_str("\n");
            }
    
            result
    
        }
    }


}



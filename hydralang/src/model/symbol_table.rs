use std::collections::HashMap;
use super::{Expression, expression::Node};


#[derive(Clone, Debug)]
pub struct SymbolTable {
    table: HashMap<String, Expression>
}

impl SymbolTable {
    
    pub fn get(&self, key: &String) -> Option<&Expression> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: String, value: Expression) {
        self.table.insert(key, value);
    }

    pub fn get_args(&self, args: &[String]) -> Option<Box<[Expression]>> {
        let mut result = Vec::new();
        for arg in args.iter() {
            result.push(self.table.get(arg)?.clone());
        }
        Some(result.into_boxed_slice())
    }

    pub fn get_args_nodes(&self, args: &[String]) -> Option<Box<[Node]>> {
        self.get_args(args).map(
            |arr| arr.into_iter().map(|e| e.get_root_node().clone()).collect() //TODO: Remove double clone
        )
    }

    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }

}





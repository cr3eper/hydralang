pub mod expression;
pub mod expression_builder;

pub use expression::Expression;




pub mod constraint {

    #[derive(Clone)]
    pub enum Constraint {
        Range(i64, i64),
        Type(String)
    }

}

pub use constraint::Constraint;


pub mod function {
    use super::{expression::Expression, Constraint};

    #[derive(Clone)]
    pub struct FunctionDef {
        name: String,
        args: Vec<String>,
        expr: Expression,
        constraints: Vec<Constraint>
    }


    /// A function collection is a group of function definitions that are of the same name, meaning they are overloaded.
    /// Which function we actually execute is determined by structural matching on the arguments and constraints
    #[derive(Clone)]
    pub struct FunctionCollection {
        name: String,
        function_defs: Vec<FunctionDef> // TODO: For now we linearly check each function def to keep things simple, This can and will be parallelized later
    }


    impl ToString for FunctionDef {
        fn to_string(&self) -> String {
            let mut result = String::new();

            result.push_str(format!("{}(", self.name).as_str());
            result.push_str(self.args.iter().map(|s| s.clone()).collect::<Vec<String>>().join(", ").as_str());
            result.push_str(") = ");
            result.push_str(self.expr.to_string().as_str());
            result.push_str(" where { #TODO: Use Constraints #");
            result.push_str("}");

            result
        }
    }

    impl FunctionDef {

        pub fn new(name: String, args: Vec<String>, expr: Expression, constraints: Vec<Constraint>) -> Self {
            FunctionDef { name, args, expr, constraints }
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }

    }


    impl FunctionCollection {

        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn new(name: String) -> Self {
            FunctionCollection { name, function_defs: Vec::new() }
        }

        pub fn add_function_def(&mut self, function_def: FunctionDef) {
            self.function_defs.push(function_def);
        }

        pub fn execute(&self, args: Vec<Expression>) -> Expression {
            // Here is where we apply pattern matching and constraint matching to determine which function to execute
            todo!()
        }   
    }

    impl ToString for FunctionCollection {
        fn to_string(&self) -> String {
            let mut result = String::new();

            for func in &self.function_defs {
                result.push_str(func.to_string().as_str());
                result.push_str("\n");
            }

            result
        }
    }

    
}

pub use function::FunctionDef;




pub mod script {
    use std::collections::HashMap;

    use super::{Expression, FunctionDef, function::FunctionCollection};


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

        pub fn add_expression_evaluation(&mut self, expression: Expression) {
            self.expressions.push(expression);
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

pub use script::Script;


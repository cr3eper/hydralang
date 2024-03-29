use std::rc::Rc;

use crate::{traits::{Callable, DeepEq}, visitor::{VariableReplacer, ExpressionModfierVisitor, DefaultSimplifyVisitor}};
use crate::model::expression_builder::num;
use super::{Expression, expression::Node, symbol_table::SymbolTable, Script};



pub struct ExpressionTemplate {
    expr: Expression
}


impl ExpressionTemplate {
    pub fn new(expr: Expression) -> Self { Self { expr } }
}

impl ToString for ExpressionTemplate {
    fn to_string(&self) -> String { self.expr.to_string() }
}

impl Callable for ExpressionTemplate {

    fn call(&self, symbol_table: SymbolTable) -> Expression {
        VariableReplacer::new(symbol_table).visit(self.expr.clone())
    }
}


pub struct RustInternalFunction {
    args: Box<[String]>,
    internal_function: fn(&[Node]) -> Expression
}

impl RustInternalFunction {
    pub fn new(args: Box<[String]>, f: fn(&[Node]) -> Expression) -> Self { Self { args, internal_function: f } }
}

impl Callable for RustInternalFunction {
    fn call(&self, symbol_table: SymbolTable) -> Expression { 
        (self.internal_function)(symbol_table.get_args_nodes(self.args.as_ref()).unwrap().as_ref()) 
    }
}

impl ToString for RustInternalFunction {
    fn to_string(&self) -> String { String::from("0 #(Rust Embedded Function)#") }
}


#[derive(Clone)]
pub struct FunctionDef {
    name: String,
    args: Vec<Expression>,
    expr: Rc<dyn Callable>,
    constraints: Vec<Expression>,
    annotations: Vec<String>,
    is_system_function: bool // Some functions require a system based implementation
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
        result.push_str(self.args.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ").as_str());
        result.push_str(") = ");
        result.push_str(self.expr.to_string().as_str());

        if self.constraints.len() != 0 {
            result.push_str(" where { ");
            result.push_str(self.constraints.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ").as_str());
            result.push_str(" }");
        }

        result
    }
}

impl FunctionDef {

    pub fn new(name: String, args: Vec<Expression>, expr: Expression, constraints: Vec<Expression>, annotations: Vec<String>) -> Self {
        FunctionDef { name, args, expr: Rc::new(ExpressionTemplate::new(expr)) as Rc<dyn Callable>, constraints, annotations, is_system_function: false }
    }

    pub fn new_system_function_def(name: String, args: Vec<Expression>, internal_function: RustInternalFunction, constraints: Vec<Expression>) -> Self {
        FunctionDef { name, args, expr: Rc::new(internal_function) as Rc<dyn Callable> , annotations: Vec::new(), constraints, is_system_function: true }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_args(&self) -> Vec<Expression> {
        self.args.clone()
    }

    pub fn hide(&mut self) {
        self.is_system_function = true;
    }

    pub fn get_annotations(&self) -> &Vec<String> {
        &self.annotations
    }

    pub fn try_apply<'a>(&self, input_args: &'a Vec<Expression>, script: &'a Script) -> Option<Expression> {

        if input_args.len() != self.args.len() { return None; }
        
        let mut symbol_table = SymbolTable::new();
        for (n, input_arg ) in input_args.iter().enumerate() {
            let is_match = self.args.get(n).unwrap().compare_to(input_arg, &mut symbol_table);
            if !is_match {
                return None;
            }
        }

        // Check if contraints are valid
        let mut constraint_eval = self.constraints.iter()
            .map(|e| ExpressionTemplate::new(e.clone()))
            .map(|t| t.call(symbol_table.clone()))
            .map(|e| DefaultSimplifyVisitor::new(script).visit(e));

        if !(constraint_eval.all(|e| e.deep_eq(&Expression::new(num(1))))) {
            return None;
        }

        Some(self.expr.call(symbol_table))

    }

}


impl FunctionCollection {

    pub fn get_name(&self) -> &String { &self.name }

    pub fn new(name: String) -> Self { FunctionCollection { name, function_defs: Vec::new() } }

    pub fn add_function_def(&mut self, function_def: FunctionDef) { self.function_defs.push(function_def); }

    pub fn get_function_defs(&self) -> Vec<FunctionDef> { self.function_defs.clone() }

    pub fn hide(&mut self) {
        for func in self.function_defs.iter_mut() {
            func.hide();
        }
    }

    fn contains_lazy(f: &FunctionDef) -> bool {
        f.get_annotations().contains(&"lazy".to_string())
    }

    pub fn try_apply<'a>(&self, args: &'a Vec<Expression>, script: &'a Script) -> Option<Expression> {

        for func in self.function_defs.iter().filter(|f| !Self::contains_lazy(f)) {
            if let Some(result) = func.try_apply(args, script) {
                return Some(result);
            }
        }

        // We currently iter through the whole function_defs list twice to account for lazy definitions, for now this is fine
        for func in self.function_defs.iter().filter(|f| Self::contains_lazy(f)) {
            if let Some(result) = func.try_apply(args, script) {
                return Some(result);
            }
        }

        None
    }   
}

impl ToString for FunctionCollection {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for func in &self.function_defs {
            if !func.is_system_function {
                result.push_str(func.to_string().as_str());
                result.push_str("\n");
            }
        }

        result
    }
}

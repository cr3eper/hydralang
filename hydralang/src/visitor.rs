use crate::{model::{Expression, expression::Node, Script, symbol_table::SymbolTable, number::Number}, traits::DeepEq};

// TODO: Currently expressions are immutable and need to be completely rebuilt to be modified. This makes sense for now and helps avoid many bugs, but optimisations are possible that have not been implemneted
// This is a basic left side, depth first traversal with no modifications made
pub trait ExpressionModfierVisitor {

    fn visit(&mut self, e: Expression) -> Expression { Expression::new(self.visit_node(e.get_root_node().clone() )) }

    fn visit_node(&mut self, n: Node) -> Node {
        match n {
            Node::Op(op_type, box l, box r) => self.visit_op(op_type, l, r),
            Node::LOp(op_type, box child) => self.visit_lop(op_type, child),
            Node::Num(n) => self.visit_num(n),
            Node::Float(n) => self.visit_float(n),
            Node::Var(name) => self.visit_var(name),
            Node::Vector(v) => self.visit_vec(v),
            Node::FunctionCall { name, args } => self.visit_function_call(name, args)
        }
    }

    fn visit_op(&mut self, op_type: String, l: Node, r: Node) -> Node { Node::Op(op_type, Box::new(self.visit_node(l)), Box::new(self.visit_node(r))) }

    fn visit_lop(&mut self, op_type: String, child: Node) -> Node { Node::LOp(op_type, Box::new(self.visit_node(child))) }

    fn visit_num(&mut self, n: Number) -> Node { Node::Num(n) }

    fn visit_float(&mut self, n: f64) -> Node { Node::Float(n) }

    fn visit_var(&mut self, name: String) -> Node { Node::Var(name) }

    fn visit_vec(&mut self, v: Vec<Node>) -> Node { Node::Vector(v.into_iter().map(|n| self.visit_node(n)).collect()) }

    fn visit_function_call(&mut self, name: String, args: Vec<Node>) -> Node { Node::FunctionCall { name, args: args.into_iter().map(|n| self.visit_node(n)).collect() } }


}

pub struct DefaultSimplifyVisitor<'a> {
    script: &'a Script
}

impl<'a> DefaultSimplifyVisitor<'a> {

    pub fn new(script: &'a Script) -> Self {
        DefaultSimplifyVisitor{ script }
    }

}

impl<'a> ExpressionModfierVisitor for DefaultSimplifyVisitor<'a> {

    fn visit_node(&mut self, n: Node) -> Node {
        let unsimplified_result = match n {
            Node::Op(op_type, box l, box r) => self.visit_op(op_type, l, r),
            Node::LOp(op_type, box child) => self.visit_lop(op_type, child),
            Node::Num(n) => self.visit_num(n),
            Node::Float(n) => self.visit_float(n),
            Node::Var(name) => self.visit_var(name),
            Node::Vector(v) => self.visit_vec(v),
            Node::FunctionCall { name, args } => self.visit_function_call(name, args)
        };

        let result = self.script.exec_function("eval", vec![Expression::new(unsimplified_result.clone())]);

        match result {
            Some(e) => if e.deep_eq(&Expression::new(unsimplified_result)) { e.get_root_node().clone() } else { self.visit_node(e.get_root_node().clone()) },
            None => unsimplified_result
        }
    }

    fn visit_vec(&mut self, v: Vec<Node>) -> Node {

        if v.len() == 1 {
            return self.visit_node(v[0].clone());
        } else {
            Node::Vector(v.into_iter().map(|n| self.visit_node(n)).collect())
        }
    }

    // We always want to "simplify" function calls by applying them, or else what's the point in having them
    fn visit_function_call(&mut self, name: String, args: Vec<Node>) -> Node {

        let args = args.into_iter().map(|n| self.visit_node(n)).collect::<Vec<Node>>();
        if let Some(result) = self.script.exec_function(name.as_str(), args.iter().map(|n| Expression::new(n.clone())).collect()) {
            self.visit_node(result.get_root_node().clone())
        }else {
            Node::FunctionCall { name, args }
        }
    }

}


pub trait ImmutableExpressionVisitor<T> {
    
        fn visit(&mut self, e: Expression) -> T {
            self.visit_node(&e.get_root_node())
        }
    
        fn visit_node(&self, n: &Node) -> T {
            match n {
                Node::Op(op_type, box l, box r) => self.visit_op(op_type, l, r),
                Node::LOp(op_type, box child) => self.visit_lop(op_type, child),
                Node::Num(n) => self.visit_num(n),
                Node::Float(n) => self.visit_float(n),
                Node::Var(name) => self.visit_var(name),
                Node::Vector(v) => self.visit_vec(v),
                Node::FunctionCall { name, args } => self.visit_function_call(name, args)
            }
        }
    
        fn visit_op(&self, op_type: &String, l: &Node, r: &Node) -> T;
        fn visit_lop(&self, op_type: &String, child: &Node) -> T;
        fn visit_num(&self, n: &Number) -> T;
        fn visit_float(&self, n: &f64) -> T;
        fn visit_var(&self, name: &String) -> T;
        fn visit_vec(&self, v: &Vec<Node>) -> T;
        fn visit_function_call(&self, name: &String, args: &Vec<Node>) -> T;
    
}


pub struct VariableReplacer {
    symbol_table: SymbolTable
}

impl VariableReplacer {

    pub fn new(symbol_table: SymbolTable) -> Self {
        Self { symbol_table }
    }
}

impl ExpressionModfierVisitor for VariableReplacer {

    fn visit_var(&mut self, name: String) -> Node {
        if let Some(replacement) = self.symbol_table.get(&name) {
            replacement.get_root_node().clone()
        }else{
            Node::Var(name)
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::expression_builder::*;
    use crate::parsing::parser::parse_statement;

    #[test]
    fn test_remove_unneeded_vec() {

        let test = "x^2 + (2 * x + ((10 - 4))) * 10";
        let script = Script::new(Vec::new(), Vec::new());
        let mut visitor = DefaultSimplifyVisitor::new(&script);

        let expect = Expression::new( 
         add(
            pow(var("x".to_string()), num(2)),
            mul(
                add(
                    mul(num(2), var("x".to_string())),
                    sub(num(10), num(4))
                ),
                num(10)
            )
        ));

        let parsed = parse_statement(test).unwrap();
        let result = visitor.visit(parsed.clone());

        assert_eq!(result, expect);



    }

    #[test]
    fn test_variable_substitution() {
        let test_script = "f(x) = x^2
        f(10 + 2)";
        let script = Script::parse(test_script).unwrap();
        println!("function args: {:?}", script.get_function_defs().get(0).unwrap().get_args());
        let expr = script.get_expression(0).unwrap().clone();
        println!("orginal expression {:?}", expr.clone());
        let mut visitor = DefaultSimplifyVisitor::new(&script);
        let result = visitor.visit(expr);
        println!("{:?}", result);
        //assert!(result.deep_eq(&Expression::parse("(10+2)^2".to_string()).unwrap()));
        
    }

}


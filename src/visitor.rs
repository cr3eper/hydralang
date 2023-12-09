use crate::{model::{Expression, expression::Node}, traits::DeepEq};

// TODO: Currently expressions are immutable and need to be completely rebuilt to be modified. This makes sense for now and helps avoid many bugs, but optimisations are possible that have not been implemneted
// This is a basic left side, depth first traversal with no modifications made
pub trait ExpressionModfierVisitor {

    fn visit(&mut self, e: Expression) -> Expression {
        Expression::new(self.visit_node(e.get_root_node().clone() ))
    }

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

    fn visit_op(&mut self, op_type: String, l: Node, r: Node) -> Node {
        Node::Op(op_type, Box::new(self.visit_node(l)), Box::new(self.visit_node(r)))
    }

    fn visit_lop(&mut self, op_type: String, child: Node) -> Node {
        Node::LOp(op_type, Box::new(self.visit_node(child)))
    }

    fn visit_num(&mut self, n: i64) -> Node {
        Node::Num(n)
    }

    fn visit_float(&mut self, n: f64) -> Node {
        Node::Float(n)
    }

    fn visit_var(&mut self, name: String) -> Node {
        Node::Var(name)
    }

    fn visit_vec(&mut self, v: Vec<Node>) -> Node {
        Node::Vector(v.into_iter().map(|n| self.visit_node(n)).collect())
    }

    fn visit_function_call(&mut self, name: String, args: Vec<Node>) -> Node {
        Node::FunctionCall { name, args: args.into_iter().map(|n| self.visit_node(n)).collect() }
    }


}

pub struct DefaultSimplifyVisitor {}

impl DefaultSimplifyVisitor {
    fn new() -> Self {
        DefaultSimplifyVisitor{}
    }
}

impl ExpressionModfierVisitor for DefaultSimplifyVisitor {

    fn visit_vec(&mut self, v: Vec<Node>) -> Node {

        if v.len() == 1 {
            return self.visit_node(v[0].clone());
        } else {
            Node::Vector(v.into_iter().map(|n| self.visit_node(n)).collect())
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
        fn visit_num(&self, n: &i64) -> T;
        fn visit_float(&self, n: &f64) -> T;
        fn visit_var(&self, name: &String) -> T;
        fn visit_vec(&self, v: &Vec<Node>) -> T;
        fn visit_function_call(&self, name: &String, args: &Vec<Node>) -> T;
    
}

pub struct CommutativeExpressionMatcher {
    target: Expression
}

impl CommutativeExpressionMatcher {

    pub fn new(target: Expression) -> Self {
        CommutativeExpressionMatcher { target }
    }

    pub fn matches(&self, e: &Expression) -> bool {
        self.target.get_root_node().deep_eq(e.get_root_node())
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
        let mut visitor = DefaultSimplifyVisitor::new();

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

}


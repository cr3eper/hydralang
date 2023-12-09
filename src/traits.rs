use std::collections::HashMap;

use crate::{model::expression::Node, visitor::ImmutableExpressionVisitor};


/// Compares expression equivalence not mathmatical equivalence ie: 4 / 2 = 2 would be false in this context
/// This function trait can be a more expensive comparison since it often involves traversing the entire tree to determine if two trees are logically equivalent
pub trait DeepEq {

    fn deep_eq(&self, other: &Self) -> bool;

}

pub trait ShallowEq {

    fn shallow_eq(&self, other: &Self) -> bool;
}

/// A complicated problem when you 
pub trait StructuralEq {

    fn structural_eq(&self, other: &Self) -> bool;

}

impl DeepEq for Node {

    fn deep_eq(&self, other: &Self) -> bool {
        use Node::*;
        match (self, other) {
            (Num(a), Num(b)) => a == b,
            (Var(a), Var(b)) => a == b,
            (Vector(v1), Vector(v2)) => v1.len() == v2.len() && v1.iter().zip(v2.iter()).all(|(a, b)| a.deep_eq(b)),
            (Op(s1, a1, a2), Op(s2, b1, b2)) => s1 == s2 && a1.deep_eq(b1) && a2.deep_eq(b2),
            (LOp(s1, a), LOp(s2, b)) => s1 == s2 && a.deep_eq(b),
            (Float(f1), Float(f2)) => f1 == f2,
            _ => false
        }
    }
}

impl ShallowEq for Node {

    fn shallow_eq(&self, other: &Self) -> bool {
        use Node::*;
        match (self, other) {
            (Num(_), Num(_)) => true,
            (Var(_), Var(_)) => true,
            (Vector(_), Vector(_)) => true,
            (Op(s1, _, _), Op(s2, _, _)) => s1 == s2,
            (LOp(s1, _), LOp(s2, _)) => s1 == s2,
            (Float(_), Float(_)) => true,
            _ => false
        }
    }
    
}

struct StructuralEqVisitor {
    main: Node,
    other: Node,
    variable_mapping: HashMap<String, Option<String>>
}

impl ImmutableExpressionVisitor<bool> for StructuralEqVisitor {
    
        fn visit_op(&self, op_type: &String, l: &Node, r: &Node) -> bool {
            todo!()
        }
    
        fn visit_lop(&self, op_type: &String, child: &Node) -> bool {
            todo!()
        }
    
        fn visit_num(&self, n: &i64) -> bool {
            todo!()
        }
    
        fn visit_float(&self, n: &f64) -> bool {
            todo!()
        }
    
        fn visit_var(&self, name: &String) -> bool {
            todo!()
        }
    
        fn visit_vec(&self, v: &Vec<Node>) -> bool {
            todo!()
        }
    
        fn visit_function_call(&self, name: &String, args: &Vec<Node>) -> bool {
            todo!()
        }
    
}

impl StructuralEq for Node {
    
        fn structural_eq(&self, other: &Self) -> bool {
            todo!()
        }
        
}



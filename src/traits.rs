use std::collections::HashMap;

use crate::{model::{expression::Node, Expression}, visitor::ImmutableExpressionVisitor};


pub trait Callable: ToString {

    fn call(&self, symbol_table: HashMap<String, Expression>) -> Expression;

}

/// Compares expression equivalence not mathmatical equivalence ie: 4 / 2 = 2 would be false in this context
/// This function trait can be a more expensive comparison since it often involves traversing the entire tree to determine if two trees are logically equivalent
pub trait DeepEq {

    fn deep_eq(&self, other: &Self) -> bool;

}

pub trait ShallowEq {

    fn shallow_eq(&self, other: &Self) -> bool;
}

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


impl StructuralEq for Node {
    
        fn structural_eq(&self, other: &Self) -> bool {
            todo!()
        }
        
}



use std::rc::Rc;

use crate::model::{expression::Node, Expression};



struct TraversalStructuralNode<'a> {
    children: Box<[TraversalStructuralNode<'a>]>,
    reference: &'a Node,
    state_ref: usize
}

// #[derive(Clone)]
// pub struct Traverser<'a, State: Default> {
//     root_traversal_node: Rc<TraversalStructuralNode<'a>>,
//     current_node: Rc<TraversalStructuralNode<'a>>,
//     root_reference_tree: &'a Node,
//     current_reference_node: &'a Node,
//     state_table: Vec<State>
// }


pub struct TreeStructure<'a, State: Default> {
    root_traversal_node: Box<TraversalStructuralNode<'a>>,
    root_reference_tree: &'a Node,
    state_table: Vec<State>
}

impl<'a, State: Default> TreeStructure<'a, State> {

    fn build_traversal_tree(node: &'a Node, state_table: &mut Vec<State>) -> TraversalStructuralNode<'a>{

        state_table.push(State::default());
        
        match node {
            Node::Op(_, l, r) => TraversalStructuralNode { 
                children: vec![Self::build_traversal_tree(l, state_table), Self::build_traversal_tree(r, state_table)].into_boxed_slice(),
                reference: &node,
                state_ref: state_table.len() - 1
            },
            Node::LOp(_, b) => TraversalStructuralNode { 
                children: vec![Self::build_traversal_tree(b, state_table)].into_boxed_slice(), 
                reference: &node, 
                state_ref: state_table.len() - 1 
            },
            Node::Num(_) =>   TraversalStructuralNode { children: Box::new([]), reference: &node, state_ref: state_table.len() - 1 },
            Node::Float(_) => TraversalStructuralNode { children: Box::new([]), reference: &node, state_ref: state_table.len() - 1 },
            Node::Var(_) => TraversalStructuralNode { children: Box::new([]), reference: &node, state_ref: state_table.len() - 1 } ,
            Node::Vector(args) => {
                let args_converted = args.iter().map(|node| Self::build_traversal_tree(node, state_table)).collect::<Vec<TraversalStructuralNode<'a>>>();
                let result = TraversalStructuralNode { children: args_converted.into_boxed_slice(), reference: &node, state_ref: state_table.len() - 1 };
                result
            },
            Node::FunctionCall { name:_, args } => {
                let args_converted = args.iter().map(|node| Self::build_traversal_tree(node, state_table)).collect::<Vec<TraversalStructuralNode<'a>>>();
                let result = TraversalStructuralNode { children: args_converted.into_boxed_slice(), reference: &node, state_ref: state_table.len() - 1 };
                result
            }
        }
    }

    pub fn new(expr: &'a Expression) -> Self {
        let root_reference_tree = expr.get_root_node();
        let mut state_table = Vec::new();
        let root_traversal_node = Box::new(Self::build_traversal_tree(root_reference_tree, &mut state_table));
        Self { root_traversal_node, root_reference_tree, state_table }
    }

    pub fn traverse(&'a self) -> TreeRunner<'a, State> {
        TreeRunner { current_node: &self.root_traversal_node, parent_structure: &self }
    }


}

pub struct TreeRunner<'a, State: Default> {
    current_node: &'a TraversalStructuralNode<'a>,
    parent_structure: &'a TreeStructure<'a, State>
}

impl<'a, State: Default> TreeRunner<'a, State> {
    
}

// impl<'a, State: Default> Traverser<'a, State> {

//     fn static_array<T>(data: Vec<T>) -> Rc<[T]> {
//         Rc::from(data.into_boxed_slice())
//     }

//     fn build_traversal_tree(node: &'a Node, state_table: &mut Vec<State>) -> TraversalStructuralNode<'a>{

//         state_table.push(State::default());
        
//         match node {
//             Node::Op(_, l, r) => TraversalStructuralNode { children: Rc::new([Rc::new(Self::build_traversal_tree(l, state_table)), Rc::new(Self::build_traversal_tree(r, state_table))]), reference: &node, state_ref: state_table.len() - 1},
//             Node::LOp(_, b) => TraversalStructuralNode { children: Rc::new([Rc::new(Self::build_traversal_tree(&b, state_table))]), reference: &node, state_ref: state_table.len() - 1 },
//             Node::Num(_) =>   TraversalStructuralNode { children: Rc::new([]), reference: &node, state_ref: state_table.len() - 1 },
//             Node::Float(_) => TraversalStructuralNode { children: Rc::new([]), reference: &node, state_ref: state_table.len() - 1 },
//             Node::Var(_) => TraversalStructuralNode { children: Rc::new([]), reference: &node, state_ref: state_table.len() - 1 } ,
//             Node::Vector(args) => {
//                 let args_converted = args.iter().map(|node| Rc::new(Self::build_traversal_tree(node, state_table))).collect::<Vec<Rc<TraversalStructuralNode<'a>>>>();
//                 let result = TraversalStructuralNode { children: Self::static_array(args_converted), reference: &node, state_ref: state_table.len() - 1 };
//                 result
//             },
//             Node::FunctionCall { name:_, args } => {
//                 let args_converted = args.iter().map(|node| Rc::new(Self::build_traversal_tree(node, state_table))).collect::<Vec<Rc<TraversalStructuralNode<'a>>>>();
//                 let result = TraversalStructuralNode { children: Self::static_array(args_converted), reference: &node, state_ref: state_table.len() - 1 };
//                 result
//             }
//         }
//     }

//     pub fn new(expr: &'a Expression) -> Self {
//         let root_reference_tree = expr.get_root_node();
//         let current_reference_node = root_reference_tree;
//         let mut state_table = Vec::new();
//         let node = Self::build_traversal_tree(root_reference_tree, &mut state_table);
//         let current_node = Rc::new(node);
//         let root_traversal_node = current_node.clone();
//         Traverser { root_traversal_node, current_node, root_reference_tree, current_reference_node, state_table }
//     }

//     pub fn root(&'a mut self) -> &mut Self {
//         self.current_node = self.root_traversal_node.clone();
//         self.current_reference_node = self.root_reference_tree;
//         self
//     }

//     pub fn get_state_mut(&mut self) -> &mut State {
//         &mut self.state_table[self.current_node.state_ref]
//     }
 

//     pub fn compare_to(&self, other: &Node) {

//     }


//     fn traverse<T>(&mut self, state: T , f: fn(&Self, s: T) -> Option<&Rc<TraversalStructuralNode<'a>>> ) -> Option<&mut Self> {
//         if let Some(node) = f(self, state) {
//             self.current_node = node.clone();
//             Some(self)
//         } else {
//             None
//         }
//     }

//     pub fn left(&mut self) -> Option<&mut Self> {
//         self.traverse((), |s, ()| s.current_node.children.first())
//     }

//     pub fn right(&mut self) -> Option<&mut Self> {
//         self.traverse((), |s, ()| s.current_node.children.last())
//     }

//     pub fn get(&mut self, index: usize) -> Option<&mut Self> {
//         self.traverse(index,|s, i| s.current_node.children.get(i))
//     }


// }



#[cfg(test)]
mod tests{
    use crate::parsing::parser::parse_statement;

    use super::Traverser;


    #[test]
    fn build_tree_sitter() {
        let expression = parse_statement("x^2 + 2*x - 10 * (10, 20 - 10, 30)").unwrap();
        let component = parse_statement("x^2").unwrap();
        let tree: Traverser<'_, ()> = Traverser::new(&expression);
        
    }

}





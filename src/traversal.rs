use crate::{model::{expression::Node, Expression}, stack::Stack};



struct TraversalStructuralNode<'a> {
    children: Box<[TraversalStructuralNode<'a>]>,
    reference: &'a Node,
    state_ref: usize
}


pub struct TreeStructure<'a, State: Default + Clone> {
    root_traversal_node: Box<TraversalStructuralNode<'a>>,
    root_reference_tree: &'a Node,
    state_table: Vec<State>
}


impl<'a, State: Default + Clone> TreeStructure<'a, State> {

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
        TreeRunner { history: Stack::new(), current_node: &self.root_traversal_node, parent_structure: &self }
    }


}

#[derive(Clone)]
pub struct TreeRunner<'a, State: Default + Clone> {
    history: Stack<&'a TraversalStructuralNode<'a>>,
    current_node: &'a TraversalStructuralNode<'a>,
    parent_structure: &'a TreeStructure<'a, State>
}

impl<'a, State: Default + Clone> TreeRunner<'a, State> {

    fn traverse<T>(&'a mut self, state: T , f: fn(&Self, s: T) -> Option<&'a TraversalStructuralNode<'a>> ) -> Option<&'a mut Self> {
        let node_ref = f(&self, state)?;
        self.history.push(self.current_node);
        self.current_node = node_ref;
        Some(self)
    }

    pub fn back(&'a mut self) -> Option<&'a mut Self> {
        let node_ref = self.history.pop()?;
        self.current_node = node_ref;
        Some(self)
    }

    pub fn left(&'a mut self) -> Option<&'a mut Self> {
        self.traverse((), |s, ()| s.current_node.children.first())
    }

    pub fn right(&'a mut self) -> Option<&'a mut Self> {
        self.traverse((), |s, ()| s.current_node.children.last())
    }

    pub fn get(&'a mut self, index: usize) -> Option<&'a mut Self> {
        self.traverse(index,|s, i| s.current_node.children.get(i))
    }

    pub fn count(&'a self) -> usize {
        self.current_node.children.len()
    }

    pub fn get_node(&'a self) -> &'a Node {
        self.current_node.reference
    }



}




#[cfg(test)]
mod tests{
    use crate::{parsing::parser::parse_statement, traits::DeepEq};
    use super::TreeStructure;


    #[test]
    fn basic_tree_sitter() {
        let expression = parse_statement("x^2 + 2*x - 10 * (10, 20 - 10, 30)").unwrap();
        let component = parse_statement("x^2").unwrap();
        let tree: TreeStructure<'_, ()> = TreeStructure::new(&expression);
        let mut traversal = tree.traverse();

        let left_left = traversal.left().unwrap().left().unwrap();

        assert!(left_left.get_node().deep_eq(&component.get_root_node()));

        let node3 = left_left.get_node();
        println!("left left {}", node3.to_string());
        

    }

}





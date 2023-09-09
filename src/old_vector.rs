
type NodePtr = Box<Node>;

#[derive(Debug, Clone)]
pub enum Node {
    VectorNode(Vector),
    LROperationNode(NodePtr, Box<Operation>, NodePtr)

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {

    Add,
    Subtract,
    Multiply,
    Divide,
    Dot,
    Cross

}


impl Node {

    fn unwrap_vector(&self) -> Vector {
        match self.clone() {

            Node::VectorNode(v) => v,
            _ => panic!("Not a Vector")
        }

    }
    
    fn run(&self) -> Self {

        match self.clone() {
                Node::VectorNode(v) => Node::VectorNode(v.clone()),
                Node::LROperationNode(left, op, right) => {
    
                    let left = left.run().unwrap_vector();
                    let right = right.run().unwrap_vector();
    
                    Node::VectorNode(match *op {
    
                        Operation::Add => left.add(&right),
                        Operation::Subtract => left.subtract(&right),
                        Operation::Cross => left.cross(&right),
                        _ => panic!("Invalid Operation")
    
                    })
    
                }
        }

    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {

        match (self, other) {
            (Self::VectorNode(u), Self::VectorNode(v)) => u == v,
            (Self::LROperationNode(l1, op1, r1), Self::LROperationNode(l2, op2, r2)) => l1 == l2 && op1 == op2 && r1 == r2,
            _ => false,
        }

    }
}

impl From<&str> for Operation {
    
        fn from(s: &str) -> Self {
    
            match s {
    
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                "dot" | "." => Operation::Dot,
                "cross" | "x" => Operation::Cross,
                _ => panic!("Invalid Operation")
    
            }
    
        }
    
}

impl Operation {

    fn parse_operation(input: &str) -> IResult<&str, Operation> {
        let (input, ch) = alt((char('+'), char('-')))(input)?;
        Ok((input, Operation::from(ch.to_string().as_str())))
    }
}




fn vector_parser(input: &str) -> IResult<&str, Node> {
    
        
    let (input, _) = space0(input)?;
    let (input, _) = char('{')(input)?;
    let (input, first) = double(input)?;
    let (input, rest) = many0(preceded(space1, double))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('}')(input)?;

    let mut data = Vec::with_capacity(rest.len() + 1);
    data.push(first);
    data.append(&mut rest.clone());

    let result = Vector::from(data);
    
    Ok((input, Node::VectorNode(result)))
}

// Top level parser
fn ast_parser(input: &str) -> IResult<&str, Node> {
    let (input, node) = alt((operation_parser, vector_parser))(input)?;
    Ok((input, node))
}



fn operation_parser(input: &str) -> IResult<&str, Node> {

    let (input, left) = alt((vector_parser, ast_parser))(input)?;
    let (input, _) = space0(input)?;
    let (input, op) = Operation::parse_operation(input)?;
    let (input, _) = space0(input)?;
    let (input, right) = ast_parser(input)?;

    Ok((input, Node::LROperationNode(Box::new(left), Box::new(Operation::from(op)), Box::new(right))))

}


mod ast_builder {
    use super::*;
    use Operation::*;

    pub fn lrop(left: Node, op: Operation, right: Node) -> Node {
        Node::LROperationNode(Box::new(left), Box::new(op), Box::new(right))
    }

    pub fn vec(v: Vector) -> Node {
        Node::VectorNode(v)
    }

    pub fn vec3(x: f64, y: f64, z: f64) -> Node {
        vec(Vector::new(3).x(x).x(y).x(z))
    }

    pub fn add(left: Node, right: Node) -> Node {
        lrop(left, Add, right)
    }

    pub fn subtract(left: Node, right: Node) -> Node {
        lrop(left, Subtract, right)
    }
}



#[cfg(test)]
mod tests{
    use super::*;
    use ast_builder::*;
    use Operation::*;


    #[test]
    fn test_vector_parser() {

        let u = vector_parser("{2.0 -2.0 3.0}").unwrap().1.unwrap_vector();
        let v = vector_parser("{3.0 2.0 -2.0}").unwrap().1.unwrap_vector();

        assert_eq!(u, Vector::new(3).x(2.0).y(-2.0).z(3.0));
        assert_eq!(v, Vector::new(3).x(3.0).y(2.0).z(-2.0));

    }

    #[test]
    fn parse_3_vec_operation() {

        let txt = "{1.0 2.0 3.0} + {1.0 1.0 1.0} - {1.0, 1.0, 1.0}";
        let result = operation_parser(txt).unwrap().1;

        // let expected = subtract(
        //     add(vec3(1.0, 2.0, 3.0), vec3(1.0, 1.0, 1.0)), 
        //     vec3(1.0, 1.0, 1.0)
        // );

        println!("result = {:?}", result);
        println!("--------------------------------");
        // println!("expected = {:?}", expected);

        // assert_eq!(result, expected);
    }


    #[test]
    fn parse_derivation() {
        let test = "f(x) = x^2 + 2x + 5";


    }

}


fn main() {

    let u = vector_parser("{2.0 -2.0 3.0}").unwrap().1.unwrap_vector();
    let v = vector_parser("{3.0 2.0 -2.0}").unwrap().1.unwrap_vector();

    println!("u + v = {:?}",  u.add(&v));
    println!("u + 2v = {:?}",  u.add(&v.multiply_scalar(2.0)));
    println!("u - v = {:?}",  u.subtract(&v));
    println!("v - u = {:?}",  v.subtract(&u));

    let v = vector_parser("{1.0 2.0 3.0}").unwrap().1;

    println!("v = {:?}", v);

    let op = operation_parser("{2.0 -2.0 3.0} + {3.0 2.0 -2.0}").unwrap().1;

    println!("op = {:?}", op);
    println!("result = {:?}", op.run());





}

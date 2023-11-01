

#[derive(Clone)]
pub struct Stack<T>{
    data: Vec<T>
}

impl<T> Stack<T> {

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        self.data.push(value);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T>{
        if self.len() == 0 {
            None
        }else{
            Some(&self.data[self.len() - 1])
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

}



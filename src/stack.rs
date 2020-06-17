#[derive(Debug)]
pub struct Stack {
    data_stack: Vec<i32>,
}

impl Stack {
    pub(crate) fn new() -> Stack {
        Stack {
            data_stack: Vec::<i32>::new(),
        }
    }
    
    pub fn push(&mut self, d: i32) {
        self.data_stack.push(d);
    }

    pub fn pop(&mut self) -> i32 {
        match self.data_stack.pop() {
            Some(x) => x,
            None => panic!("stack is empty"),
        }
    }
}
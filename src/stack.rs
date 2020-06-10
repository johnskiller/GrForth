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

    fn pop(&mut self) -> i32 {
        match self.data_stack.pop() {
            Some(x) => x,
            None => panic!("stack is empty"),
        }
    }

    pub fn dup(&mut self) {
        let x = self.pop();
        self.data_stack.push(x);
        self.data_stack.push(x);
    }
    pub fn swap(&mut self) {
        let x = self.pop();
        let y = self.pop();
        self.push(y);
        self.push(x);
    }

    pub fn mul(&mut self) {
        let x = self.pop();
        let y = self.pop();

        self.push(x * y);
    }
    pub fn disp(&mut self) {
        let x = self.pop();
        print!("{:?} ", x);
    }
}
use crate::core::ForthCore;

pub trait Primv {
    fn dup(&mut self);
    fn swap(&mut self);
    fn mul(&mut self);
    fn div(&mut self);
    fn muldiv(&mut self);
    fn eq(&mut self);
    fn disp(&mut self);
    fn emit(&mut self);
    fn cr(&mut self);
    fn eval(&mut self);
}

impl Primv for ForthCore<'_> {
    fn cr(&mut self) {
        self.push('\n' as i32);
        self.emit();
    }

    fn dup(&mut self) {
        let x = self.pop();
        self.push(x);
        self.push(x);
    }
    fn swap(&mut self) {
        let x = self.pop();
        let y = self.pop();
        self.push(y);
        self.push(x);
    }

    fn mul(&mut self) {
        let x = self.pop();
        let y = self.pop();
        self.push(x * y);
    }
    fn div(&mut self) {
        let x = self.pop();
        let y = self.pop();
        self.push(y / x);
    }
    fn muldiv(&mut self) {
        let x = self.pop();
        let y = self.pop();
        let z = self.pop();
        self.push(z * y / x  );
    }
    fn eq(&mut self) {
        let x = self.pop();
        let y = self.pop();
        if x == y {
            self.push(1);
        } else {
            self.push(0);
        }
    }
    fn disp(&mut self) {
        let x = self.pop();
        print!("{} ", x);
    }

    fn emit(&mut self) {
        let x = self.pop() as u8 as char;
        print!("{}", x);
    }

    fn eval(&mut self) {

    }
}

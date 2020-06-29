use crate::word::ForthWord;
use crate::core::ForthCore;

pub trait Primv {
    fn dup(&mut self, _: &ForthWord);
    fn swap(&mut self, _: &ForthWord);
    fn mul(&mut self, _: &ForthWord);
    fn eq(&mut self, _: &ForthWord);
    fn disp(&mut self, _: &ForthWord);
    fn emit(&mut self, _: &ForthWord);
    fn cr(&mut self, defines: &ForthWord);
    fn eval(&mut self, _: &ForthWord);
}

impl Primv for ForthCore<'_> {
    fn cr(&mut self, defines: &ForthWord) {
        self.push('\r' as i32);
        self.emit(defines);
    }

    fn dup(&mut self, _: &ForthWord) {
        let x = self.pop();
        self.push(x);
        self.push(x);
    }
    fn swap(&mut self, _: &ForthWord) {
        let x = self.pop();
        let y = self.pop();
        self.push(y);
        self.push(x);
    }

    fn mul(&mut self, _: &ForthWord) {
        let x = self.pop();
        let y = self.pop();
        self.push(x * y);
    }
    fn eq(&mut self, _: &ForthWord) {
        let x = self.pop();
        let y = self.pop();
        if x == y {
            self.push(1);
        } else {
            self.push(0);
        }
    }
    fn disp(&mut self, _: &ForthWord) {
        let x = self.pop();
        print!("{} ", x);
    }

    fn emit(&mut self, _: &ForthWord) {
        let x = self.pop() as u8 as char;
        print!("{}", x);
    }

    fn eval(&mut self, _: &ForthWord) {

    }
}

use crate::core::Defines;
use crate::core::ForthCore;

pub trait Primv {
    fn dup(&mut self, _: &Defines);
    fn swap(&mut self, _: &Defines);
    fn mul(&mut self, _: &Defines);
    fn disp(&mut self, _: &Defines);
    fn emit(&mut self, _: &Defines);
    fn cr(&mut self, defines: &Defines);
}

impl Primv for ForthCore<'_> {
    fn cr(&mut self, defines: &Defines) {
        self.push('\r' as i32);
        self.emit(defines);
    }

    fn dup(&mut self, _: &Defines) {
        let x = self.pop();
        self.push(x);
        self.push(x);
    }
    fn swap(&mut self, _: &Defines) {
        let x = self.pop();
        let y = self.pop();
        self.push(y);
        self.push(x);
    }

    fn mul(&mut self, _: &Defines) {
        let x = self.pop();
        let y = self.pop();
        self.push(x * y);
    }
    fn disp(&mut self, _: &Defines) {
        let x = self.pop();
        print!("{:?} ", x);
    }

    fn emit(&mut self, _: &Defines) {
        let x = self.pop() as u8 as char;
        print!("emit:{}", x);
    }
}

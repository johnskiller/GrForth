use std::rc::Rc;

#[derive(Clone)]
struct Word {
    name: &'static str,
    func: fn(&mut ForthCore),
    //func: for<'r> fn(&'r ForthCore<i32>),
    wtype: WordType,
}

struct ForthCore {
    stack: Vec<i32>,
    //v: Vec<fn()>,
    words: Vec<Word>,
}
#[derive(Clone,Copy)]
enum WordType {
    Internal,
    Dict,
    Lit,
    Imed, //immediate
}

impl ForthCore {
    fn new() -> ForthCore {
        let words = vec![
            Word {
                name: "dup",
                func: Self::dup,
                wtype: WordType::Internal,
            },
            Word {
                name: "swap",
                func: Self::swap,
                wtype: WordType::Internal,
            },
            Word {
                name: ".",
                func: Self::disp,
                wtype: WordType::Internal,
            },
            Word {
                name: "*",
                func: Self::mul,
                wtype: WordType::Internal,
            },
        ];

        ForthCore {
            
            stack: Vec::<i32>::new(),
            //v: Vec::new(),
            words: words,
        }
    }
    fn dup(&mut self) {
        let x = self.stack.pop().unwrap();
        self.stack.push(x);
        self.stack.push(x);
    }
    fn swap(&mut self) {
        println!("SWAP");
    }

    fn mul(&mut self) {
        let x = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();

        self.stack.push(x * y);
    }
    fn disp(&mut self) {
        
            let x = self.stack.pop().unwrap();
            print!("{:?} ", x);
        
    }

    fn call(&mut self, name: &str) {
        let mut word = self.words.iter().rev().filter( |w|  w.name.eq_ignore_ascii_case(name));
        match word.next() {
            Some(a) => {(a.func)(self)},
            None => {println!("[{}] word not found",name)},
        }

    }

    fn run(&mut self, s: &str) {
        for token in s.split(" ").filter(|x| !x.eq_ignore_ascii_case("")) {
            match token.parse::<i32>() {
                Ok(x) => self.stack.push(x),
                Err(_) => {
                    self.call(token);
                }
            }
            //println!("[{}]",token);
        }
    }
}

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new();

    let s = ": 52    dup *  . ; ";
    core.run(s);
}
fn main() {
    test()
}

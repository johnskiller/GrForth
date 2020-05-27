#[derive(Clone,Copy)]
struct Word {
    name: &'static str,
    func: fn(&mut ForthCore),
    //func: for<'r> fn(&'r ForthCore<i32>),
    //type: WORD_TYPE = Internal,
}

struct ForthCore {
    stack: Vec<i32>,
    v: Vec<fn()>,
    words: Vec<Word>,
}

enum WORD_TYPE {
    Internal,
    Dict,
    Lit,
}

impl ForthCore {
    fn new() -> ForthCore {
        ForthCore {
            stack: Vec::<i32>::new(),
            v: Vec::new(),
            words: Vec::<Word>::new(),
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
        for w in &self.words.clone() {
            if name.eq_ignore_ascii_case(w.name) {
                (w.func)(self)
            }
        }
    }
    fn init(&mut self) {
        let words = vec![
            Word {
                name: "dup",
                func: Self::dup,
            },
            Word {
                name: "swap",
                func: Self::swap,
            },
            Word {
                name: ".",
                func: Self::disp,
            },
            Word {
                name: "*",
                func: Self::mul,
            },
        ];
        self.words = words;
    }

    fn run(&mut self, s: &str) {
        for token in s.split(" ") {
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
    core.init();
    // v.push(dup);
    // v.push(swap);
    //println!("{:x}",v);
    /*
    for p in v.iter(){
        p();
    }
    */

    let s = ": 52    dup *  . ; ";
    core.run(s);
}
fn main() {
    test()
}

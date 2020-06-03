
#[derive(Clone)]
struct InnerWord {
    name: &'static str,
    func: fn(&mut ForthCore),
    //func: for<'r> fn(&'r ForthCore<i32>),
    wtype: WordType,
}


struct UserDefinedWord {
    name: &'static str,
    define: Vec<ForthWord>,
}

enum ForthWord {
    Inner(InnerWord),
    Udw(UserDefinedWord),
}
struct ForthCore {
    stack: Vec<i32>,
    //v: Vec<fn()>,
    words: Vec<ForthWord>,
}
#[derive(Clone, Copy)]
enum WordType {
    Internal,
    Dict,
    Lit,
    Imed, //immediate
}

impl ForthCore {

    fn init(&mut self) {
        let mut add_inner_word = |word|  {
            //let w = Box::new(ForthWord::Inner(word));
            self.words.push(ForthWord::Inner(word));
        };

        let mut words = Vec::<ForthWord>::new();
        add_inner_word(InnerWord {
            name: "swap",
            func: Self::swap,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: ".",
            func: Self::disp,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: "*",
            func: Self::mul,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: "dup",
            func: Self::dup,
            wtype: WordType::Internal,
        });
        let define = Vec::<ForthWord>::new();
        //define.push(self.find("dup").unwrap());

        let udw = UserDefinedWord{
            name:"**",
            define,
        };
        words.push(ForthWord::Udw(udw));
    }
    fn new() -> ForthCore {
        ForthCore {
            stack: Vec::<i32>::new(),
            //v: Vec::new(),
            words: Vec::<ForthWord>::new(),
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

    fn find(&self, name: &str) -> Option<&ForthWord> {
        let  words = self
        .words
        .iter()
        .rev();
        
        for item in words {
            match item {
                ForthWord::Inner(w) => {if w.name.eq_ignore_ascii_case(name) {return Some(item);}}
                ForthWord::Udw(w) => {if w.name.eq_ignore_ascii_case(name) {return Some(item);}}
            }
        }
        return None;
    }
    fn call(&mut self, name: &str) {
        let word = self.find(name);

        match word {
            Some(w) => {
                match w {
                    ForthWord::Inner(x) => (x.func)(self),
                    ForthWord::Udw(_) => (),
                }
                
            }
            None => println!("[{}] word not found", name),
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
    core.init();
    let s = ": 52    dup *  . ; ";
    core.run(s);
}
fn main() {
    test()
}

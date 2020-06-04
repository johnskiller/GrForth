use std::ops::Deref;

#[derive(Clone)]
struct InnerWord {
    name: &'static str,
    func: fn(&mut Stack),
    wtype: WordType,
}

struct Stack {
    data_stack: Vec<i32>,
}

impl Stack {
    fn new() -> Stack {
        Stack{data_stack : Vec::<i32>::new()}
    }
    fn push(&mut self, d: i32) {
        self.data_stack.push(d);
    }

    fn pop(&mut self) -> i32 {
        match self.data_stack.pop() {
            Some(x) => x,
            None => panic!("stack is empty"),
        }
    }

    fn dup(&mut self) {
        let x = self.pop();
        self.data_stack.push(x);
        self.data_stack.push(x);
    }
    fn swap(&mut self) {
        println!("SWAP");
    }

    fn mul(&mut self) {
        let x = self.pop();
        let y = self.pop();

        self.push(x * y);
    }
    fn disp(&mut self) {
        let x = self.pop();
        print!("{:?} ", x);
    }
}
struct UserDefinedWord {
    name: &'static str,
    //func: fn(&mut ForthCore, defines: &Vec<usize>),
    defines: Vec<usize>,
}



enum ForthWord {
    Inner(InnerWord),
    Udw(UserDefinedWord),
}
struct ForthCore {
    stack: Stack,
    //v: Vec<fn()>,
    words: Vec<Box<ForthWord>>,
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
            self.words.push(Box::new(ForthWord::Inner(word)));
        };

       
        add_inner_word(InnerWord {
            name: "swap",
            func: Stack::swap,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: ".",
            func: Stack::disp,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: "*",
            func: Stack::mul,
            wtype: WordType::Internal,
        });
        add_inner_word(InnerWord {
            name: "dup",
            func: Stack::dup,
            wtype: WordType::Internal,
        });
        let mut defines = Vec::<usize>::new();
        let w = self.find("dup").unwrap();
        defines.push(w);
        let w = self.find("*").unwrap();
        defines.push(w);

        let udw = UserDefinedWord{
            name:"**",
            //func: Self::exec_udw,
            defines,
        };
        self.words.push(Box::new(ForthWord::Udw(udw)));
    }
    fn new() -> ForthCore {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: Vec::<Box<ForthWord>>::new(),
        }
    }



    fn find(&self, name: &str) -> Option<usize> {
        let  word = self
        .words
        .iter()
        .rev().position(|x| {
            match x.deref() {
                ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
                ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
            }
        } );
        
        match word {
            Some(w) => Some(self.words.len() - w -1),
            None => None,
        }
    }

    fn call_by_pos(&mut self, pos: usize) {
        let len = self.words.len();
        println!("len: {}, pos: {}",len,pos);
        let word = (&self.words[pos]).deref();
        match word {
            ForthWord::Inner(w) => {
                println!("call word: {}",w.name);
                (w.func)(&mut self.stack)
            },
            ForthWord::Udw(w) => {
                let defs =  w.defines.clone();
                for d in defs {
                    println!("find at pos:{}",d);
                    self.call_by_pos(d);
                }
            }   
        }       
    }
    fn call_by_name(&mut self, name: &str) {
        let pos = self.find(name);

        match pos {
            Some(w) => {
                
                self.call_by_pos(w);
                }
            
            None => println!("[{}] word not found", name),
        }
    }


    fn run(&mut self, s: &str) {
        for token in s.split(" ").filter(|x| !x.eq_ignore_ascii_case("")) {
            match token.parse::<i32>() {
                Ok(x) => self.stack.push(x),
                Err(_) => {
                    println!("will run:[{}]",token);
                    self.call_by_name(token);
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
    let s = ": 52    dup *  . 52 ** . ; ";
    core.run(s);
}
fn main() {
    test()
}

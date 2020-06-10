use crate::stack::Stack;
use std::io::stdin;
use std::io::Read;
use std::fmt;
use std::ops::Deref;

mod stack;

#[derive(Clone)]
struct InnerWord {
    name: &'static str,
    func: fn(&mut Stack),
    wtype: WordType,
}

impl fmt::Display for InnerWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for InnerWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
struct UserDefinedWord<'a> {
    name: &'a  str,
    //func: fn(&mut ForthCore, defines: &Vec<usize>),
    defines: Vec<usize>,
}

impl<'a> UserDefinedWord<'a> {
    fn exec(&self, core:&mut ForthCore) {
        for d in &self.defines {
            //println!("find at pos:{}", d);
            core.call_by_pos(*d);
        } 
    }
}
struct CompileWord<'a> {
    name: &'static str,
    func: fn(&mut ForthCore<'a>),
}

impl<'a> CompileWord<'a> {
    fn new(name: &'static str, func: fn(& mut ForthCore<'a>)) -> Self { 
        Self { name, func } 
    }
}
impl fmt::Debug for CompileWord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
#[derive(Debug)]
enum ForthWord<'a> {
    Inner(InnerWord),
    Udw(UserDefinedWord<'a>),
    Compiler(CompileWord<'a>),
}

#[derive(Debug)]
enum CoreState {
    Normal,
    CustomInit,
    Custom,
}
#[derive(Debug)]
struct ForthCore<'a> {
    stack: Stack,
    //v: Vec<fn()>,
    words: Vec<Box<ForthWord<'a>>>,
    state: CoreState,
}
#[derive(Clone, Copy, Debug)]
enum WordType {
    Internal,
    Dict,
    Lit,
    Imed, //immediate
}

impl<'a> ForthCore<'a> {
    pub fn add_udw(&mut self, name: &'a str, def: Vec<&str>) {
        let mut defines = Vec::<usize>::new();
        for n in def {
            let w = self.find(n).unwrap();
            defines.push(w);
        }
        let udw = UserDefinedWord {
            name,
            //func: Self::exec_udw,
            defines,
        };
        self.words.push(Box::new(ForthWord::<'a>::Udw(udw)));
    }
    fn init_dict() -> Vec<Box<ForthWord<'a>>> {
        let mut dict = Vec::<Box<ForthWord>>::new();
        let mut add_inner_word = |word| {
            //let w = Box::new(ForthWord::Inner(word));
            dict.push(Box::new(ForthWord::Inner(word)));
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
        //self.add_udw("**", vec!["dup", "*"]);

        let d_word = CompileWord::new(":",Self::define_word);
        dict.push(Box::new(ForthWord::Compiler(d_word)));
        let end_def = CompileWord::new(";",Self::end_of_define);
        dict.push(Box::new(ForthWord::Compiler(end_def)));

        dict
    }
    fn new(dict:Vec<Box<ForthWord<'a>>>) -> ForthCore<'a> {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: dict,
            state: CoreState::Normal,
        }
    }

    fn find(&self, name: &str) -> Option<usize> {
        let word = self.words.iter().rev().position(|x| match x.deref() {
            ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Compiler(w) =>w.name.eq_ignore_ascii_case(name), 
        });

        match word {
            Some(w) => Some(self.words.len() - w - 1),
            None => None,
        }
    }


    fn define_word(&mut self) {
        self.state = CoreState::CustomInit;
        print!("define a new word ");
    }

    fn end_of_define(&mut self) {
        self.state = CoreState::Normal;
    }
    fn exec(core: &mut ForthCore, w:&UserDefinedWord) {
        let defs = w.defines.clone();
                for d in defs {
                    //println!("find at pos:{}", d);
                    core.call_by_pos(d);
                }
    }
    fn call_by_pos(&mut self, pos: usize) {
        let _len = self.words.len();
        //println!("len: {}, pos: {}", _len, pos);
        let word = (&self.words[pos]).deref();
        println!("ForthWord: {:?}", word);
        match word {
            ForthWord::Inner(w) => {
                //println!("call word: {}", w.name);
                (w.func)(&mut self.stack)
            }
            ForthWord::Udw(w) => {
                let defs = w.defines.clone();
                for d in defs {
                    //println!("find at pos:{}", d);
                    self.call_by_pos(d);
                }
                //w.exec(self);
                //ForthCore::exec(self,w);
            }
            ForthWord::Compiler(w) => {
                //println!("call word: {}", w.name);
                (w.func)(self)
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

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

}


fn interpret<'a>(core: &mut ForthCore<'a>, s: &'a String) {
    //let tokenizer = Tokenizer::new(s);
    let tokenizer = s.split_whitespace();
    let mut new_word: & str = "";
    let mut w_list = Vec::<&str>::new();
    for token in tokenizer {
        match token.parse::<i32>() {
            Ok(x) => core.push(x),
            Err(_) => {
                //println!("will run:[{}]", token);
                match core.state {
                    CoreState::CustomInit => {
                        println!("{}",token);
                        core.state = CoreState::Custom;
                        new_word = token;
                    },
                    CoreState::Normal => core.call_by_name(token),
                    CoreState::Custom => {
                        println!("body: {}",token);
                        if token.eq(";") {
                            core.state = CoreState::Normal;

                            core.add_udw(new_word.clone(),w_list.clone());
                            //self.add_udw("2dup", w_list.clone());
                            println!("{} define complete",new_word);
                        } else {
                            w_list.push(token);
                        }
                    }

                }
                
            }
        }
        //println!("[{}]",token);
    }
}

struct Interpretor {
    name: &'static str,
}

impl Interpretor {
    fn exec(&self, core: &mut ForthCore) {
        core.call_by_pos(1);
    }
}

fn call(core: &mut ForthCore) {
    core.call_by_pos(1);
}

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new(ForthCore::init_dict());
    //core.init();
    core.add_udw("**", vec!["dup", "*"]);
    println!("{:?}", core);
    let s = "3 2 * . : 2dup dup dup ; 3 2dup * * .";
    let input = &s.to_string();
    interpret(&mut core,input);

    let mut buffer = String::new();

    print!("OK.");
    stdin().read_line(&mut buffer);

    interpret(&mut core,&buffer);
    println!("we got {}",buffer);
    println!("{:?}", core);
    
}
fn main() {
    test()
}

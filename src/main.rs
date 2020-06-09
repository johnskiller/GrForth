use std::fmt;
use std::ops::Deref;

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
struct Stack {
    data_stack: Vec<i32>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            data_stack: Vec::<i32>::new(),
        }
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
    fn disp(&mut self) {
        let x = self.pop();
        print!("{:?} ", x);
    }
}
#[derive(Debug)]
struct UserDefinedWord<'a> {
    name: &'a  str,
    //func: fn(&mut ForthCore, defines: &Vec<usize>),
    defines: Vec<usize>,
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
    CompileName,
    CompileBody,
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
    fn init(&mut self) {
        let mut add_inner_word = |word| {
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
        self.add_udw("**", vec!["dup", "*"]);

        let d_word = CompileWord::new(":",Self::define_word);
        self.words.push(Box::new(ForthWord::Compiler(d_word)));
        let end_def = CompileWord::new(";",Self::end_of_define);
        self.words.push(Box::new(ForthWord::Compiler(end_def)));
    }
    fn new() -> ForthCore<'a> {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: Vec::<Box<ForthWord>>::new(),
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
        self.state = CoreState::CompileName;
        print!("define a new word ");
    }

    fn end_of_define(&mut self) {
        self.state = CoreState::Normal;
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
struct Tokenizer<'a> {
    tokens: std::str::SplitWhitespace<'a>,
}

impl<'a> Tokenizer<'a> {
    //fn new(tokens: Vec<String>) -> Self { Self { tokens } }

    fn new(input: &'a str) -> Self {
        let tokens = input.split_whitespace();
        Tokenizer { tokens }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next()
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
                    CoreState::CompileName => {
                        println!("{}",token);
                        core.state = CoreState::CompileBody;
                        new_word = token;
                    },
                    CoreState::Normal => core.call_by_name(token),
                    CoreState::CompileBody => {
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
fn test2() {
    let mut tokenizer = Tokenizer::new(": abc 52 dup * . ;");
    assert_eq!(Some(":"),tokenizer.next());
    
    for x in tokenizer.into_iter() {
        println!("{}", x);
    }
}

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new();
    core.init();
    println!("{:?}", core);
    let s = "3 2 * . : 2dup dup dup ; 3 2dup * * .";
    interpret(&mut core,&s.to_string());
}
fn main() {
    test()
}

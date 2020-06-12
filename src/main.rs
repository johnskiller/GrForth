use std::fmt::Debug;
use std::io::stdin;
use std::fmt;

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
struct UserDefinedWord<'a> {
    name: &'a  str,
    //func: fn(&mut ForthCore, defines: &Vec<usize>),
    defines: Vec<&'a ForthWord<'a>>,//Vec<usize>,
    words:  &'a[fn(&mut ForthCore<'a>)],
}

impl<'a> Debug for UserDefinedWord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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
    words: Vec<ForthWord<'a>>,
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
    fn find_ref(& self, name: &str) -> Option<&ForthWord<'a>> {
        let word = self.words.iter().rev().find(|x| match x {
            ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Compiler(w) =>w.name.eq_ignore_ascii_case(name), 
        });
        word
    } 
    /*
    pub fn add_udw<'b: 'a > (&'b mut self, name: &'a str, def: Vec<&str>) {
        // let mut defines = Vec::<usize>::new();
        // for n in def {
        //     let w = self.find(n).unwrap();
        //     defines.push(w);
        // }
        /*
        let search = |name| {
            let word = self.words.iter().rev().find(|x| match x {
                ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
                ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
                ForthWord::Compiler(w) =>w.name.eq_ignore_ascii_case(name), 
            });
            word.unwrap()
            
        };*/
        let fdefs = def.iter().map(|w| self.find_ref(w).unwrap()).collect();
        let udw = UserDefinedWord {
            name,
            //func: Self::exec_udw,
            defines: fdefs,
            words:&[],
        };
        self.words.push(ForthWord::<'a>::Udw(udw));
    }*/
    fn init(&mut self) {
        let mut add_inner_word = |word| {
            //let w = Box::new(ForthWord::Inner(word));
            self.words.push(ForthWord::Inner(word));
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
        self.words.push(ForthWord::Compiler(d_word));
        let end_def = CompileWord::new(";",Self::end_of_define);
        self.words.push(ForthWord::Compiler(end_def));
    }
    fn new() -> ForthCore<'a> {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: Vec::<ForthWord>::new(),
            state: CoreState::Normal,
        }
    }
    /*
    fn find(&self, name: &str) -> Option<usize> {
        let word = self.words.iter().rev().position(|x| match x {
            ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Compiler(w) =>w.name.eq_ignore_ascii_case(name), 
        });

        match word {
            Some(w) => Some(self.words.len() - w - 1),
            None => None,
        }
    }*/

    fn define_word(&mut self) {
        self.state = CoreState::CustomInit;
        print!("define a new word ");
    }

    fn end_of_define(&mut self) {
        self.state = CoreState::Normal;
    }
    /*
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
    */

    fn get_name(word: &ForthWord<'a>) -> &'a str {
        match word {
            ForthWord::Inner(w) => {w.name}
            ForthWord::Udw(w) => {w.name}
            ForthWord::Compiler(w) => {w.name}
        }
    } 
    pub fn call_by_name2(&mut self, name: &str) {
        let word = self.find_ref(name);
        match word {
            Some(x) => {
                match x {
                    ForthWord::Inner(w) => {(w.func)(&mut self.stack)}
                    ForthWord::Udw(w) => {
                        w.defines.iter().map(|d|{
                            let name = ForthCore::get_name(*d);
                            self.call_by_name2(name);
                        });
                       

                    }
                    ForthWord::Compiler(_) => {}
                }
            }
            None => println!("{} not found",name),
        }
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

fn interpret<'a>(core: &'a mut ForthCore<'a>, s: &'a String) {
    //let tokenizer = Tokenizer::new(s);
    let tokenizer = s.split_whitespace();
    let mut new_word: & str = "";
    let mut w_list = Vec::<&str>::new();
    for token in tokenizer {
        match token.parse::<i32>() {
            Ok(x) => core.stack.push(x),
            Err(_) => {
                //println!("will run:[{}]", token);
                match core.state {
                    CoreState::CustomInit => {
                        println!("{}",token);
                        core.state = CoreState::Custom;
                        new_word = token;
                    },
                    CoreState::Normal => core.call_by_name2(token),
                    CoreState::Custom => {
                        println!("body: {}",token);
                        if token.eq(";") {
                            core.state = CoreState::Normal;
                            let mut words = core.words;
                            add_udw(words, new_word.clone(),w_list.clone());
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
    //core.add_udw("**", vec!["dup", "*"]);
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
pub fn add_udw<'a> (words: &'a mut Vec<ForthWord<'a>>, name: &'a str, def: Vec<&str>) {
    // let mut defines = Vec::<usize>::new();
    // for n in def {
    //     let w = self.find(n).unwrap();
    //     defines.push(w);
    // }
    
    let search = |name| {
        let word = words.iter().rev().find(|x| match x {
            ForthWord::Inner(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Udw(w) => w.name.eq_ignore_ascii_case(name),
            ForthWord::Compiler(w) =>w.name.eq_ignore_ascii_case(name), 
        });
        word.unwrap()
        
    };
    let fdefs = def.iter().map(|w| search(w)).collect();
    let udw = UserDefinedWord {
        name,
        //func: Self::exec_udw,
        defines: fdefs,
        words:&[],
    };
    words.push(ForthWord::<'a>::Udw(udw));
}
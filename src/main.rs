use crate::stack::Stack;
use std::fmt;
use std::io::prelude::*;

mod stack;

#[derive(Clone)]
struct ForthWord<'a> {
    name: String,
    func: fn(&mut ForthCore<'a>, pos: usize),
    defines: Vec<usize>,
    wtype: WordType,
    immediate: bool,
}

impl<'a> fmt::Display for ForthWord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "name:{} defines:{:?} immediate?{}",
            self.name, self.defines, self.immediate
        )
    }
}

impl<'a> fmt::Debug for ForthWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name:{} defines:{:?} immediate?{}",
            self.name, self.defines, self.immediate
        )
    }
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
    words: Vec<ForthWord<'a>>,
    state: CoreState,
    param: Vec<usize>,
}
#[derive(Clone, Copy, Debug)]
enum WordType {
    Internal,
    Dict,
    Lit,
    Imed, //immediate
    Const,
    Var,
}

impl<'a> ForthCore<'a> {
    pub fn add_udw(&mut self, name: String, def: Vec<&str>) {
        let mut defines = Vec::<usize>::new();
        for n in def {
            let w = self.find(n).unwrap();
            defines.push(w);
        }
        //let name = String::from(name);
        let udw = ForthWord {
            name,
            func: Self::exec_udw,
            defines,
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.push(udw);
    }
    fn do_lit(&mut self, _: usize) {
        println!("DOLIT {:?}", self);
        // how to get next word from defines that use lit?
        let caller_pos = self.param.last().unwrap();
        let caller = &self.words[*caller_pos];
        println!("Caller: {:?}", caller);
        let caller_defs = &caller.defines;
        let n = caller_defs[0];
        self.push(n as i32);
        println!("push {} to data stack", n);
    }

    fn do_exit(&mut self, _: usize) {
        // pop from param stack
        self.param.pop().unwrap();
    }
    fn do_colon(&mut self, defines: Vec<usize>) {
        for d in defines {
            self.call_by_pos(d);
        }
    }
    fn exec_udw(&mut self, pos: usize) {
        println!("UDW");
        // push current pos to param stack
        self.param.push(pos);
        let word = &self.words[pos];
        let defines = word.defines.clone();
        self.do_colon(defines);
    }
    fn init_dict() -> Vec<ForthWord<'a>> {
        let mut dict = Vec::<ForthWord>::new();
        let mut add_inner_word = |name:&str, func| {
            let word = ForthWord {
                name: name.to_string(),
                func,
                defines: vec![],
                wtype: WordType::Internal,
                immediate: false,
            };
            //let w = Box::new(ForthWord::Inner(word));
            dict.push(word);
        };
        add_inner_word("swap",Self::swap);
        add_inner_word(".", Self::disp);
        add_inner_word("*", Self::mul);
        add_inner_word("dup",Self::dup);
        add_inner_word("exit", Self::do_exit);
        //self.add_udw("**", vec!["dup", "*"]);

        add_inner_word(":", Self::define_word);
        let word = ForthWord {
            name: ";".to_string(),
            defines: vec![],
            func: Self::end_of_define,
            wtype: WordType::Imed,
            immediate: true,
        };
        dict.push(word);
        let word = ForthWord {
            name: "lit".to_string(),
            defines: vec![],
            func: Self::do_lit,
            wtype: WordType::Lit,
            immediate: false,
        };
        dict.push(word);
        dict
    }
    fn new(dict: Vec<ForthWord<'a>>) -> ForthCore<'a> {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: dict,
            state: CoreState::Normal,
            param: Vec::<usize>::new(),
        }
    }

    fn find(&self, name: &str) -> Option<usize> {
        let word = self
            .words
            .iter()
            .rev()
            .position(|x| x.name.eq_ignore_ascii_case(name));

        match word {
            Some(w) => Some(self.words.len() - w - 1),
            None => None,
        }
    }

    fn define_word(&mut self, _: usize) {
        self.state = CoreState::CustomInit;
        print!("define a new word ");
    }

    fn end_of_define(&mut self, _: usize) {
        self.state = CoreState::Normal;
        self.compile("exit");
    }
    /*
    fn exec(core: &mut ForthCore, w:&UserDefinedWord) {
        let defs = w.defines.clone();
                for d in defs {
                    //println!("find at pos:{}", d);
                    core.call_by_pos(d);
                }
    }*/
    fn call_by_pos(&mut self, pos: usize) {
        let _len = self.words.len();
        //println!("len: {}, pos: {}", _len, pos);
        let func = self.words[pos].func;
        println!("ForthWord: {:?}", self.words[pos]);

        func(self, pos);
    }
    fn call_by_name(&'a mut self, name: &str) {
        let pos = self.find(name);

        match pos {
            Some(w) => {
                self.call_by_pos(w);
            }

            None => println!("[{}] word not found", name),
        }
    }
}

impl<'a> ForthCore<'a> {
    fn get_state(&self) -> &CoreState {
        &self.state
    }

    fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }
    fn parse_word(&mut self, token: &str) {
        match self.find(token) {
            Some(pos) => {
                match self.get_state() {
                    CoreState::Normal => self.call_by_pos(pos),
                    CoreState::CustomInit => {
                        println!("{} redefined", token);
                        let new_word = String::from(token); // duplicate word, redefine
                        self.add_udw(new_word, Vec::<&str>::new());
                        self.state = CoreState::Custom;
                    }
                    CoreState::Custom => {
                        let immediate = self.words[pos].immediate;
                        if immediate {
                            println!("exec immediate word {}", token);
                            self.call_by_pos(pos);
                        } else {
                            self.compile(token);
                        }
                    }
                }
            }
            None => {
                // not found
                match self.state {
                    CoreState::CustomInit => {
                        println!("{} define ", token);
                        let new_word = String::from(token);
                        self.add_udw(new_word.clone(), Vec::<&str>::new());
                        self.state = CoreState::Custom;
                    }

                    CoreState::Normal => match token.parse::<i32>() {
                        Ok(n) => {
                            println!("{} pushed", n);
                            self.stack.push(n)
                        }
                        Err(_) => println!("Unknown word: {}", token),
                    },
                    CoreState::Custom => {
                        match token.parse::<i32>() {
                            Ok(n) => {
                                //println!("push Lit {} to param stack",n);
                                //self.param.push(n);

                                self.compile("lit");
                                self.compile_lit(n as usize);
                            }
                            Err(_) => panic!("Unknown word when define:{}", token),
                        }
                    }
                }
            }
        }
    }

    fn compile(&mut self, token: &str) {
        println!("compile {} into last word", token);
        let pos = self.find(token).unwrap();
        self.compile_lit(pos);
    }
    fn compile_lit(&mut self, val: usize) {
        let len = self.words.len();
        let defines = &mut self.words[len - 1].defines;
        defines.push(val);
    }

    fn interpret(&mut self, s: String) {
        //let tokenizer = Tokenizer::new(s);

        let tokenizer = s.split_whitespace();
        //let mut new_word = String::new();
        //let mut w_list = Vec::<&str>::new();

        for token in tokenizer {
            self.parse_word(token);
        }
    }

    pub fn push(&mut self, d: i32) {
        self.stack.push(d);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop()
    }

    pub fn dup(&mut self, _: usize) {
        let x = self.pop();
        self.stack.push(x);
        self.stack.push(x);
    }
    pub fn swap(&mut self, _: usize) {
        let x = self.pop();
        let y = self.pop();
        self.push(y);
        self.push(x);
    }

    pub fn mul(&mut self, _: usize) {
        let x = self.pop();
        let y = self.pop();

        self.push(x * y);
    }
    pub fn disp(&mut self, _: usize) {
        let x = self.pop();
        print!("{:?} ", x);
    }
}

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new(ForthCore::init_dict());
    //core.init();
    core.add_udw("**".to_string(), vec!["dup", "*"]);
    println!("{:?}", core);
    let s = "3 2 * . : 3x 3 * ; 4 3x .";
    let input = s.to_string();
    core.interpret(input);
    //println!("{:?}", core);

    loop {
        let line = readline();
        core.interpret(line);
    }
    println!("{:?}", core);
}

fn readline() -> String {
    let stdin = std::io::stdin();

    let input = stdin.lock().lines().next();

    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()
}

fn main() {
    test()
}

use crate::dictionary::WFunc;
use crate::dictionary::Dictionary;
use crate::dictionary::DefineItem;
use crate::primv::Primv;
use crate::stack::Stack;
use crate::word::{ForthWord, WordType};
use log::{info, trace, warn};
use std::io::{stdout, Write};

// pub type Defines = Vec<usize>;
// const LITERAL: usize = 9999;
// const IF: usize = 9998;
// const JMP: usize = 9997;

// macro_rules! print_flush {
//     ( $($t:tt)* ) => {
//         {
//             let mut h = stdout();
//             write!(h, $($t)* ).unwrap();
//             h.flush().unwrap();
//         }
//     }
// }

#[derive(Debug)]
pub enum CoreState {
    Normal,
    Custom,
}

#[derive(Debug)]
pub struct ForthCore<'a> {
    stack: Stack,
    words: Dictionary<'a>,
    state: CoreState,
    return_stack: Vec<usize>,
    //input: Option<SplitWhitespace<'a>>,
    text: Vec<String>,
    IP: usize, // DEFINES POINTER
    //WP: usize, // WORD POINTER
}

impl<'a> ForthCore<'a> {

    /*
    fn do_lit(&mut self, _: usize) {
        trace!("DOLIT {:?}", self);
        // how to get next word from defines that use lit?
        let caller_pos = self.param.last().unwrap();
        let caller = &self.words[*caller_pos];
        trace!("Caller: {:?}", caller);
        let caller_defs = &caller.defines;
        let n = caller_defs[0];
        self.push(n as i32);
        trace!("push {} to data stack", n);
    } */

    fn init_dict(&mut self) {
        //let mut dict = Vec::<ForthWord>::new();
        let mut add_inner_word = |name: &'a str, func, immediate| {
            let word = ForthWord::<'a> {
                name: String::from(name),
                func,
                //defines: vec![],
                wtype: WordType::Primv,
                immediate,
                define_ptr: 999,
            };
            //let w = Box::new(ForthWord::Inner(word));
            self.words.create_primv_word(word);
        };
        add_inner_word("swap", Self::swap, false);
        add_inner_word(".", Self::disp, false);
        add_inner_word("*", Self::mul, false);
        add_inner_word("/", Self::div, false);
        add_inner_word("*/", Self::muldiv, false);
        add_inner_word("dup", Self::dup, false);
        add_inner_word("exit", Self::do_exit, false);
        add_inner_word("=", Self::eq, false);
        //self.add_udw("**", vec!["dup", "*"]);

        add_inner_word(":", Self::define_word, false);
        add_inner_word("create", Self::do_create, true);
        add_inner_word(";", Self::end_of_define, true);
        add_inner_word("emit", Self::emit, false);
        add_inner_word("cr", Primv::cr, false);
        add_inner_word("const", Self::define_const, false);
        add_inner_word("var", Self::define_var, false);
        add_inner_word("_lit", Self::do_literal, false); // runtime of const
        add_inner_word("words", Self::do_words, false);
        add_inner_word("core", Self::do_core, false);
        add_inner_word(">R", Self::do_toR, false);
        add_inner_word("R@", Self::do_fromR, false);

        add_inner_word("@", Self::do_retrive, false);
        add_inner_word("!", Self::do_store, false);

        add_inner_word("if", Self::do_if, true);
        add_inner_word("then", Self::do_then, true);
        add_inner_word("endif", Self::do_then, true);
        add_inner_word("else", Self::do_else, true);
        add_inner_word("_if", Self::_if, false);
        add_inner_word("_jmp", Self::_jmp, false);

        /*
        let word = ForthWord {
            name: "lit".to_string(),
            defines: vec![],
            func: Self::do_lit,
            wtype: WordType::Lit,
            immediate: false,
        };
        dict.push(word);
        */
    }
    pub fn new() -> Self {
        //let dict = Self::init_dict();
        let mut core = Self {
            stack: Stack::new(),
            //v: Vec::new(),
            words: Dictionary::new(),
            state: CoreState::Normal,
            return_stack: Vec::<usize>::new(),
            //       input: None,
            text: vec![],
            IP: 99999,
            //WP: 99999,
        };
        core.init_dict();
        core
    }


    /*
    fn exec(core: &mut ForthCore, w:&UserDefinedWord) {
        let defs = w.defines.clone();
                for d in defs {
                    //trace!("find at pos:{}", d);
                    core.call_by_pos(d);
                }
    }
    fn call_by_pos(&mut self, pos: usize) {
        //let _len = self.words.len();
        //trace!("len: {}, pos: {}", _len, pos);
        //let func = self.words[pos].func;
        let word = self.words.get_by_pos(pos).clone();
        let func = word.func;
        //let defines = &self.words[pos].defines.clone();
        trace!("ForthWord: {:?}", word);
        self.WP = pos;
        func(self);
    }
    */
    /*
    fn call_by_name(&'a mut self, name: &str) {
        let pos = self.words.find(name);

        match pos {
            Some(w) => {
                self.call_by_pos(w);
            }

            None => trace!("[{}] word not found", name),
        }
    }
    */

    fn get_state(&self) -> &CoreState {
        &self.state
    }

    fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }
    fn readline(&mut self) -> usize {
        //type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

        //let stdin = std::io::stdin();
        //let input = stdin.lock().lines().next();
        /*let text = input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string();*/

        //self.text = text.clone();
        print!("Ok. ");
        stdout().flush();
        let mut input = String::new();
        let _len = std::io::stdin().read_line(&mut input);

        let text = input
            .split_whitespace()
            .map(|s| s.to_string())
            .rev()
            .collect();
        //trace!("got input {:?}", text);
        self.text = text;

        return self.text.len();
        //self.input = Some(text.split_whitespace());
        //.to_string()
    }

    fn get_next_token(&mut self) -> String {
        loop {
            match self.text.pop() {
                Some(n) => return n,
                None => {
                    self.readline();
                    // trace!("in loop");
                }
            }
        }
    }

    pub fn interpret2(&mut self, input: &str) {
        let text = input
            .split_whitespace()
            .map(|s| s.to_string())
            .rev()
            .collect();
        //trace!("got input {:?}", text);
        self.text = text;
        loop {
            match self.text.pop() {
                Some(n) => self.parse_word(n.as_ref()),
                None => {
                    return;
                    // trace!("in loop");
                }
            }
        }
    }
    pub fn interpret(&mut self) {
        loop {
            let tz = self.get_next_token();

            //    trace!("got token {}", tz);
            self.parse_word(&tz);
        }
    }
    
    /*
    fn execute(&mut self, word: &ForthWord<'a>) {
        let func = word.func;
        func(self); //execute word
    }
    */ 

    fn parse_word(&mut self, token: &str) {
        match self.words.find(token) {
            Some(word) => match self.get_state() {
                CoreState::Normal => {
                    self.IP = word.define_ptr + 1;
                    let func = word.func;
                    func(self);
                },
                CoreState::Custom => {
                    let immediate = word.immediate;
                    if immediate {
                        trace!("exec immediate word {}", token);
                        self.IP = word.define_ptr;
                        let func = word.func;
                        func(self);
                    } else {
                        self.compile(token);
                    }
                }
            },
            None => {
                // not found
                match self.state {
                    CoreState::Normal => match token.parse::<i32>() {
                        Ok(n) => {
                            trace!("{} pushed", n);
                            self.stack.push(n)
                        }
                        Err(_) => println!("Unknown word: {}", token),
                    },
                    CoreState::Custom => {
                        match token.parse::<i32>() {
                            Ok(n) => {
                                //trace!("push Lit {} to param stack",n);
                                //self.param.push(n);

                                self.compile("_lit");
                                //self.compile_lit(LITERAL);
                                self.compile_lit(n);
                                trace!("compile {} literal to word", n);
                            }
                            Err(_) => panic!("Unknown word when define:{}", token),
                        }
                    }
                }
            }
        }
    }
    fn compile(&mut self, token: &str) {
        trace!("compile {} into last word", token);
        self.words.compile(token);
        // match self.words.find(token) {
        //     Some(w) => {},
        //     None => println!("{} not found",token), 
        // }
        //self.compile_lit(pos);
    }
    fn compile_lit(&mut self, val: i32) {
        self.words.compile_lit(val);
    }

    pub fn push(&mut self, d: i32) {
        self.stack.push(d);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop()
    }

    fn do_literal(&mut self) {
        //let word = self.words.get_by_pos(self.WP);
        self.IP = self.IP + 1;       
        let lit = self.words.get_lit(self.IP);
        self.push(lit as i32);
        //self.IP = self.IP + 1;       
    }

    fn do_if(&mut self) {
        self.compile("_if");

        let pholder = self.words.tod();
        self.compile_lit(0); // place holder
        trace!("place holder of if {}", pholder);
        self.push(pholder as i32); // push place holder addr
    }
    fn do_else(&mut self) {
        //  at and of IF, jump to THEN
        self.compile("_jmp");
        let pholder_else = self.words.tod();
        self.compile_lit(0); // place holder, will be filled by THEN
        trace!("place holder of else {}", pholder_else);
        
        // fill place holder of if
        let pholder = self.pop() as usize;
        //let word = self.words.last_word();
        let else_pos = self.words.tod();
        trace!("then pos {}", else_pos); // put then pos/ or else pos
        self.words.put_lit(pholder , else_pos);

        self.push(pholder_else as i32); // push place holder addr
    }
    fn do_then(&mut self) {
        let pholder = self.pop() as usize;
        // no else
        //let word = self.words.last_word();
        let then_pos = self.words.tod();
        trace!("then pos {}", then_pos); // put then pos/ or else pos
        self.words.put_lit(pholder, then_pos);
    }
}
pub trait Vocabulary {
    fn do_const(&mut self);
    fn do_var(&mut self);
    fn do_retrive(&mut self);
    fn do_store(&mut self);
    fn define_const(&mut self);
    fn define_var(&mut self);
    fn define_word(&mut self);
    fn do_words(&mut self);
    fn do_core(&mut self);
    fn end_of_define(&mut self);
    fn do_create(&mut self);
    fn do_immediate(&mut self);
    fn do_colon(&mut self);
    fn do_exit(&mut self);
    fn do_toR(&mut self);
    fn do_fromR(&mut self);
    fn _if(&mut self);
    fn _jmp(&mut self);
}

impl Vocabulary for ForthCore<'_> {
    fn define_const(&mut self) {
        //self.state = CoreState::CustomInit;
        print!("define a const ");
        self.do_create();
        self.words.set_last_func(Self::do_const);
        self.words.set_last_type(WordType::Const);
        let const_value = self.pop();
        self.compile_lit(const_value);
    }

    fn define_var(&mut self) {
        trace!("define var");
        self.push(0);
        self.define_const();
        self.words.set_last_func(Self::do_var); // which push var addr to stack
        self.words.set_last_type(WordType::Var);
    }
    fn define_word(&mut self) {
        self.state = CoreState::Custom;
        print!("define a new word ");
        self.do_create();
        //self.words.last_word().func = Self::do_colon;
    }

    fn do_words(&mut self) {
        self.words.list_words();
    }
    fn do_core(&mut self) {
        println!("{:?}", self);
    }
    fn do_toR(&mut self) {
        let v = self.pop() as usize;
        self.return_stack.push(v);
    }

    fn do_fromR(&mut self) {
        let v = self.return_stack.pop().unwrap() as i32;
        self.push(v);
    }
    fn end_of_define(&mut self) {
        self.state = CoreState::Normal;
        self.compile("exit");
    }

    fn do_immediate(&mut self) {
        self.words.set_last_immediate();
    }

    fn do_create(&mut self) {
        let token = self.get_next_token();
        self.words.create_word(token);
    }
    fn do_exit(&mut self) {
        // pop from param stack
        //self.param.pop().unwrap();
        match self.return_stack.pop() {
            Some(n) => self.IP = n,
            None => {},
        }
    }

    // runtime of const
    fn do_const(&mut self) {
        //let word = self.words.get_by_pos(self.WP).clone();
        //self.IP = self.IP + 1;
        let value = self.words.get_lit(self.IP) as i32;
        self.push(value);
        //self.IP = self.IP + 1;
    }

    // runtime of variable 
    fn do_var(&mut self) {
        self.push(self.IP as i32);
    }
    fn do_store(&mut self) {
        let v_pos = self.pop() as usize;
        let val = self.pop();
        self.words.put_lit(v_pos, val as usize);
    }

    fn do_retrive(&mut self) {
        let v_pos = self.pop() as usize;
        let val = self.words.get_lit(v_pos);
        self.push(val);
    }
    fn _if(&mut self) {
        if self.pop() == 0 {
            // jump to ELSE/THEN
            self.IP = self.IP + 1;
            let then_pos = self.words.get_lit(self.IP);
            trace!("will jump to {}", then_pos);
            self.IP = (then_pos - 1 ) as usize;
        } else {
            self.IP += 1; 
        }
    }

    fn _jmp(&mut self) {
        self.IP = self.IP + 1;
        let then_pos = self.words.get_lit(self.IP);
        trace!("will jump to {}", then_pos);
        self.IP = (then_pos - 1) as usize;
    }

    // runtime of colon defination
    fn do_colon(&mut self) {
        trace!("do_colon");

        loop {
            match self.words.get_define(self.IP) {
                DefineItem::Lit(_) => {
                    warn!("wrong type");
                    self.IP += 1;
                },
                DefineItem::Func(f) =>  {
                    //f(self);
                    let exit : WFunc = Self::do_exit;
                    //let fp =  f as * const fn(WFunc) -> ();
                    if *f as usize  == exit as usize{
                        f(self);
                        break;
                    }
                    f(self);
                    self.IP += 1;
                },
                DefineItem::Addr(a) => {
                    trace!("Defined Word at {}",a);
                    self.return_stack.push(self.IP+1);
                    self.IP = *a;
                    if let DefineItem::Func(func) = self.words.get_define(self.IP) { 
                        self.IP +=1;
                        func(self); 
                        }
                }, 
            }
        } 
    }
}



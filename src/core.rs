use crate::dictionary::Dictionary;
use crate::primv::Primv;
use crate::stack::Stack;
use crate::word::{ForthWord, WordType};
use log::{info, trace, warn};
use std::io::{stdout, Write};

pub type Defines = Vec<usize>;
const LITERAL: usize = 9999;
const IF: usize = 9998;
const JMP: usize = 9997;

macro_rules! print_flush {
    ( $($t:tt)* ) => {
        {
            let mut h = stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

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
    IP: usize,
}

impl<'a> ForthCore<'a> {
    pub fn add_udw(&mut self, name: String, def: Vec<&str>) {
        let mut defines = Vec::<usize>::new();
        for n in def {
            let w = self.words.find(n).unwrap();
            defines.push(w);
        }
        //let name = String::from(name);
        let udw = ForthWord {
            name,
            func: Self::do_colon,
            defines,
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.add(udw);
    }
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

    fn init_dict() -> Vec<ForthWord<'a>> {
        let mut dict = Vec::<ForthWord>::new();
        let mut add_inner_word = |name: &str, func, immediate| {
            let word = ForthWord {
                name: name.to_string(),
                func,
                defines: vec![],
                wtype: WordType::Internal,
                immediate,
            };
            //let w = Box::new(ForthWord::Inner(word));
            dict.push(word);
        };
        add_inner_word("swap", Self::swap, false);
        add_inner_word(".", Self::disp, false);
        add_inner_word("*", Self::mul, false);
        add_inner_word("dup", Self::dup, false);
        add_inner_word("exit", Self::do_exit, false);
        add_inner_word("=", Self::eq, false);
        //self.add_udw("**", vec!["dup", "*"]);

        add_inner_word(":", Self::define_word, false);
        add_inner_word(";", Self::end_of_define, true);
        add_inner_word("emit", Self::emit, false);
        add_inner_word("cr", Primv::cr, false);
        add_inner_word("const", Self::define_const, false);
        add_inner_word("words", Self::do_words, false);
        add_inner_word("core", Self::do_core, false);
        add_inner_word(">R", Self::do_toR, false);
        add_inner_word("R@", Self::do_fromR, false);

        add_inner_word("if", Self::do_if, true);
        add_inner_word("then", Self::do_then, true);
        add_inner_word("endif", Self::do_then, true);
        add_inner_word("else", Self::do_else, true);

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
        dict
    }
    pub fn new() -> ForthCore<'a> {
        let dict = Self::init_dict();
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: Dictionary::new(dict),
            state: CoreState::Normal,
            return_stack: Vec::<usize>::new(),
            //       input: None,
            text: vec![],
            IP: 99999,
        }
    }

    fn create(&mut self) {
        // fetch name from input and create a dict entry
        let name = self.get_next_token();

        let word = ForthWord {
            name,
            func: Self::do_colon,
            defines: vec![],
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.add(word);
    }

    /*
    fn exec(core: &mut ForthCore, w:&UserDefinedWord) {
        let defs = w.defines.clone();
                for d in defs {
                    //trace!("find at pos:{}", d);
                    core.call_by_pos(d);
                }
    }*/
    fn call_by_pos(&mut self, pos: usize) {
        //let _len = self.words.len();
        //trace!("len: {}, pos: {}", _len, pos);
        //let func = self.words[pos].func;
        let word = self.words.get_by_pos(pos).clone();
        let func = word.func;
        //let defines = &self.words[pos].defines.clone();
        trace!("ForthWord: {:?}", word);

        func(self, &word);
    }
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
        let len = std::io::stdin().read_line(&mut input);

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
    pub fn interpret(&mut self) {
        loop {
            let tz = self.get_next_token();

            //    trace!("got token {}", tz);
            self.parse_word(&tz);
        }
    }

    fn parse_word(&mut self, token: &str) {
        match self.words.find(token) {
            Some(pos) => match self.get_state() {
                CoreState::Normal => self.call_by_pos(pos),

                CoreState::Custom => {
                    let immediate = self.words.get_by_pos(pos).immediate;
                    if immediate {
                        trace!("exec immediate word {}", token);
                        self.call_by_pos(pos);
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

                                //self.compile("lit");
                                self.compile_lit(LITERAL);
                                self.compile_lit(n as usize);
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
        let pos = self.words.find(token).unwrap();
        self.compile_lit(pos);
    }
    fn compile_lit(&mut self, val: usize) {
        self.words.last_word().defines.push(val);
    }

    pub fn push(&mut self, d: i32) {
        self.stack.push(d);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop()
    }

    fn do_if(&mut self, _: &ForthWord) {
        self.compile_lit(IF);
        let pholder = self.words.last_word().defines.len();
        self.compile_lit(0); // place holder
        trace!("place holder of if {}", pholder);
        self.push(pholder as i32); // push place holder addr
    }
    fn do_else(&mut self, _: &ForthWord) {
        //  at and of IF, jump to THEN
        self.compile_lit(JMP);
        let pholder_else = self.words.last_word().defines.len();
        self.compile_lit(0); // place holder, will be filled by THEN
        trace!("place holder of else {}", pholder_else);
        
        // fill place holder of if
        let pholder = self.pop() as usize;
        let word = self.words.last_word();
        let else_pos = word.defines.len();
        trace!("then pos {}", else_pos); // put then pos/ or else pos
        word.defines[pholder] = else_pos;

        self.push(pholder_else as i32); // push place holder addr
    }
    fn do_then(&mut self, _: &ForthWord) {
        let pholder = self.pop() as usize;
        // no else
        let word = self.words.last_word();
        let then_pos = word.defines.len();
        trace!("then pos {}", then_pos); // put then pos/ or else pos
        word.defines[pholder] = then_pos;
    }
}

trait Vocabulary {
    fn do_const(&mut self, word: &ForthWord);
    fn define_const(&mut self, _: &ForthWord);
    fn define_word(&mut self, _: &ForthWord);
    fn do_words(&mut self, _: &ForthWord);
    fn do_core(&mut self, _: &ForthWord);
    fn end_of_define(&mut self, _: &ForthWord);
    fn do_create(&mut self, _: &ForthWord);
    fn do_immediate(&mut self, _: &ForthWord);
    fn do_colon(&mut self, word: &ForthWord);
    fn do_exit(&mut self, _: &ForthWord);
    fn do_toR(&mut self, _: &ForthWord);
    fn do_fromR(&mut self, _: &ForthWord);
}

impl Vocabulary for ForthCore<'_> {
    fn define_const(&mut self, _: &ForthWord) {
        //self.state = CoreState::CustomInit;
        print!("define a const ");
        self.create();
        self.words.last_word().func = Self::do_const;
        self.words.last_word().wtype = WordType::Const;
        let const_value = self.pop();
        self.compile_lit(const_value as usize);
    }
    fn define_word(&mut self, _: &ForthWord) {
        self.state = CoreState::Custom;
        print!("define a new word ");
        self.create();
        self.words.last_word().func = Self::do_colon;
    }

    fn do_words(&mut self, _: &ForthWord) {
        self.words.list_words();
    }
    fn do_core(&mut self, _: &ForthWord) {
        println!("{:?}", self);
    }
    fn do_toR(&mut self, _: &ForthWord) {
        let v = self.pop() as usize;
        self.return_stack.push(v);
    }

    fn do_fromR(&mut self, _: &ForthWord) {
        let v = self.return_stack.pop().unwrap() as i32;
        self.push(v);
    }
    fn end_of_define(&mut self, _: &ForthWord) {
        self.state = CoreState::Normal;
        self.compile("exit");
    }

    fn do_immediate(&mut self, _: &ForthWord) {
        self.words.last_word().immediate = true;
    }

    fn do_create(&mut self, _: &ForthWord) {
        self.create();
    }
    fn do_exit(&mut self, _: &ForthWord) {
        // pop from param stack
        //self.param.pop().unwrap();
    }

    // runtime of const
    fn do_const(&mut self, word: &ForthWord) {
        self.push(word.defines[0] as i32);
    }

    // runtime of colon defination
    fn do_colon(&mut self, word: &ForthWord) {
        trace!("do_colon");
        //let mut iter = word.defines.iter();
        //        while let Some(&d) = iter.next() {
        self.IP = 0;
        while self.IP < word.defines.len() {
            let i = self.IP;
            self.IP = i + 1;
            let item = word.defines[i];
            if item == LITERAL {
                // LIT
                let lit = word.defines[self.IP];
                self.push(lit as i32);
                self.IP = self.IP + 1;
            } else if item == IF {
                if self.pop() == 0 {
                    // jump to ELSE/THEN
                    let then_pos = word.defines[self.IP];
                    trace!("will jump to {}", then_pos);
                    self.IP = then_pos;
                } else {
                    self.IP = self.IP + 1;
                }
            } else if item == JMP {
                    let then_pos = word.defines[self.IP];
                    trace!("will jump to {}", then_pos);
                    self.IP = then_pos;
            } else {
                self.call_by_pos(item);
            }
        }
    }
}

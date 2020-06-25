use crate::dictionary::Dictionary;
use crate::word::ForthWord;
use std::io::{stdout,Write};
use crate::primv::Primv;
use crate::stack::Stack;


pub type Defines = Vec<usize>;
const LITERAL: usize = 9999;

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
enum CoreState {
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
}
#[derive(Clone, Copy, Debug)]
pub enum WordType {
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
        println!("DOLIT {:?}", self);
        // how to get next word from defines that use lit?
        let caller_pos = self.param.last().unwrap();
        let caller = &self.words[*caller_pos];
        println!("Caller: {:?}", caller);
        let caller_defs = &caller.defines;
        let n = caller_defs[0];
        self.push(n as i32);
        println!("push {} to data stack", n);
    } */

    fn do_exit(&mut self, _: &ForthWord) {
        // pop from param stack
        //self.param.pop().unwrap();
    }
    fn do_colon(&mut self, word: &ForthWord) {
        println!("do_colon");
        let mut iter = word.defines.iter();
        while let Some(&d) = iter.next() {
            if d == LITERAL {
                // LIT
                let lit = iter.next().unwrap();
                self.push(*lit as i32);
            } else {
                self.call_by_pos(d);
            }
        }
    }

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
        }
    }



    fn create(&mut self) {
        // fetch name from input and create a dict entry
        let name = self.get_next();

        let word = ForthWord {
            name,
            func: Self::do_colon,
            defines: vec![],
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.add(word);
    }



    fn immediate(&mut self) {
        self.words.last_word().immediate = true;
    }

    fn do_const(&mut self, word: &ForthWord) {
        self.push(word.defines[0] as i32);
    }
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

    fn end_of_define(&mut self, _: &ForthWord) {
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
        //let _len = self.words.len();
        //println!("len: {}, pos: {}", _len, pos);
        //let func = self.words[pos].func;
        let word = self.words.get_by_pos(pos).clone();
        let func = word.func;
        //let defines = &self.words[pos].defines.clone();
        println!("ForthWord: {:?}", word);

        func(self, &word);
    }
    fn call_by_name(&'a mut self, name: &str) {
        let pos = self.words.find(name);

        match pos {
            Some(w) => {
                self.call_by_pos(w);
            }

            None => println!("[{}] word not found", name),
        }
    }

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
        //println!("got input {:?}", text);
        self.text = text;

        return self.text.len();
        //self.input = Some(text.split_whitespace());
        //.to_string()
    }

    fn get_next(&mut self) -> String {
        loop {
            match self.text.pop() {
                Some(n) => return n,
                None => {
                    self.readline();
                   // println!("in loop");
                }
            }
        }
    }
    pub fn interpret(&mut self) {
        loop {
            let tz = self.get_next();

        //    println!("got token {}", tz);
            self.parse_word(&tz);
        }
    }

    fn parse_word(&mut self, token: &str) {
        match self.words.find(token) {
            Some(pos) => {
                match self.get_state() {
                    CoreState::Normal => self.call_by_pos(pos),

                    CoreState::Custom => {
                        let immediate = self.words.get_by_pos(pos).immediate;
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

                                //self.compile("lit");
                                self.compile_lit(LITERAL);
                                self.compile_lit(n as usize);
                                println!("compile {} literal to word", n);
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
}

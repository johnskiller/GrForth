use crate::primv::Primv;
use crate::stack::Stack;
use std::fmt;

pub type Defines = Vec<usize>;
const LITERAL: usize = 9999;

#[derive(Clone)]
pub struct ForthWord<'a> {
    name: String,
    func: fn(&mut ForthCore<'a>, defines: &ForthWord),
    defines: Defines,
    wtype: WordType,
    immediate: bool,
}

impl<'a> fmt::Display for ForthWord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            ":{:<8} defines:{:?} {:?} immediate?{}",
            self.name, self.defines, self.wtype, self.immediate
        )
    }
}

impl<'a> fmt::Debug for ForthWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(
            f,
            "{:<8} {:?} {:?} immediate?{}",
            self.name, self.wtype, self.defines, self.immediate
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
pub struct ForthCore<'a> {
    stack: Stack,
    words: Vec<ForthWord<'a>>,
    state: CoreState,
    return_stack: Vec<usize>,
    input: Option<std::str::SplitWhitespace<'a>>,
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
            func: Self::do_colon,
            defines,
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.push(udw);
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

    pub fn init_dict() -> Vec<ForthWord<'a>> {
        let mut dict = Vec::<ForthWord>::new();
        let mut add_inner_word = |name: &str, func, immediate |  {
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
    pub fn new(dict: Vec<ForthWord<'a>>) -> ForthCore<'a> {
        ForthCore {
            stack: Stack::new(),
            //v: Vec::new(),
            words: dict,
            state: CoreState::Normal,
            return_stack: Vec::<usize>::new(),
            input: None,
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

    fn create(&mut self ) {
        // create  blank word
        let word = ForthWord {
            name:"UNKNOWN".to_string(),
            func: Self::do_colon,
            defines: vec![],
            wtype: WordType::Dict,
            immediate: false,
        };
        self.words.push(word);
    }

    fn last_word(&mut self) -> &mut ForthWord<'a> {
        let len = self.words.len();
        &mut self.words[len - 1]
    }

    fn immediate(&mut self) {
        self.last_word().immediate = true;
    }

    fn do_const(&mut self, word: &ForthWord) {
        self.push(word.defines[0] as i32);
    }
    fn define_const(&mut self, _: &ForthWord) {
        self.state = CoreState::CustomInit;
        print!("define a const ");
        self.create();
        self.last_word().func = Self::do_const;
        self.last_word().wtype = WordType::Const;
        let const_value = self.pop(); 
        self.compile_lit(const_value as usize);
    }
    fn define_word(&mut self, _: &ForthWord) {
        self.state = CoreState::CustomInit;
        print!("define a new word ");
        self.create();
        self.last_word().func = Self::do_colon;
    }

    fn do_words(&mut self, _: &ForthWord) {
        for (i, w) in self.words.iter().enumerate() {
           
            println!("{:>4}: {:?}", i, w);
        }
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
        let word = self.words[pos].clone();
        let func = word.func;
        //let defines = &self.words[pos].defines.clone();
        println!("ForthWord: {:?}", self.words[pos]);

        func(self, &word);
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

    fn get_state(&self) -> &CoreState {
        &self.state
    }

    fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }
    pub fn interpret(&mut self, s: String) {
        //let tokenizer = Tokenizer::new(s);
        let tokenizer = s.split_whitespace();
        //self.input = Some(tokenizer);
        //let mut new_word = String::new();
        //let mut w_list = Vec::<&str>::new();

        //let t = self.input.as_mut().unwrap();
        for token in tokenizer { //} =   t.next() {
            self.parse_word(token);
        }


    }

    fn parse_word(&mut self, token: &str ) {
        match self.find(token) {
            Some(pos) => {
                match self.get_state() {
                    CoreState::Normal => self.call_by_pos(pos),
                    CoreState::CustomInit => {
                        println!("{} redefined", token);
                        let new_word = String::from(token); // duplicate word, redefine
                        self.last_word().name = new_word;
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
                        //self.add_udw(new_word.clone(), Vec::<&str>::new());
                        self.last_word().name = new_word;
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

                                //self.compile("lit");
                                self.compile_lit(LITERAL);
                                self.compile_lit(n as usize);
                                println!("compile {} literal to word",n);
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
        self.last_word().defines.push(val);
    }

    pub fn push(&mut self, d: i32) {
        self.stack.push(d);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop()
    }
}

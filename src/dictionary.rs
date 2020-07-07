use std::error::Error;
use crate::core::ForthCore;
use crate::core::Vocabulary;
use crate::word::{ForthWord, WordType};

use std::collections::HashMap;
use std::fmt;

struct MyError {
    msg:String,
}


pub type WFunc<'a> = fn(&mut ForthCore<'a>) -> ();
//pub type WFunc<'a> = fn(&mut ForthCore<'a>) -> Result<(),&'static str>;

pub enum DefineItem<'a> {
    Func(WFunc<'a>),
    Lit(i32),
    Addr(usize), // position of defined word
}
impl<'a> fmt::Debug for DefineItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match *self {
            DefineItem::Func(func) => write!(f, "func point:{:X}", func as u64),
            DefineItem::Lit(n) => write!(f, "LITERAL {}", n),
            DefineItem::Addr(a) => write!(f, "Addr of word: {}",a),
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    ADD,
    MINUS,
    MUL,
    DIV,
    MOD,
    DUP,
    SWAP,
    ROT,
    OVER,
    DICT,
}

pub struct Dictionary<'a> {
    //words: Vec<ForthWord<'a>>,
    defines: Vec<DefineItem<'a>>,
    index: HashMap<String, ForthWord<'a>>,
    //tod: usize, // top of defines
    last_word: Option<String>,
}

impl<'a> fmt::Debug for Dictionary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}  last:{:?}", 
        self.defines, self.last_word)
    }
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Self {
        Dictionary {
            //words: vec![],
            defines: vec![],
            index: HashMap::new(),
            //tod: 0,
            last_word: None,
        }
    }
    /* 
    pub fn get_by_pos(&self, pos: usize) -> &ForthWord<'a> {
        &self.words[pos]
    }
    */
    /*
    pub fn add(&mut self, w: ForthWord<'a>) {
        self.words.push(w);
    }*/

    pub fn find(&self, name: &str) -> Option<&ForthWord<'a>> {
        let word = self.index.get(name);
        word
    }
    /*
    pub fn find(&self, name: &str) -> Option<usize> {
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
    */

    /*
    pub fn find_last_word(&mut self) -> &mut ForthWord<'a> {
        let word = self.words.last().unwrap();
        word.as_mut();
        //let len = self.words.len();
        //mut self.words[len - 1]
    }
    */

    fn last_word(&mut self) -> &mut ForthWord<'a> {
        match &self.last_word {
            Some(k) => self.index.get_mut(k).unwrap(),
            None => panic!("Last is none"),
        }
        //let last = last_key.clone();
        //self.index.get_mut(&last_key).unwrap()
    }
    pub fn set_last_immediate(&mut self) {
        let mut word = self.last_word();
        
        word.immediate = true;
        //self.words.push(word);
    }

    pub fn set_last_func(&mut self, func: WFunc<'a>) {
        let mut word = self.last_word();
        word.func = func;
        //self.words.push(word);
    }
    pub fn set_last_type(&mut self, wtype: WordType) {
        //let mut word = self.words.pop().unwrap();
        let mut word = self.last_word();
        word.wtype = wtype;
        //self.words.push(word);
    }
    pub fn list_words(&self) {
        //for (i, w) in self.words.iter().enumerate() {
        for (i,w) in self.index.iter().enumerate() {
            println!("{:>4}: {:?}", i, w.1);
        }
    }
    pub fn create_word(&mut self, name: String) {
        let word = ForthWord {
            name: String::from(name),
            func: ForthCore::do_colon,
            //defines: vec![],
            wtype: WordType::Dict,
            immediate: false,
            define_ptr: self.defines.len(),
        };
        //let name = &word.name;
        //let last = word.name.clone();
        self.last_word = Some(word.name.clone());
        self.index.insert(word.name.clone(), word);
        //self.words.add(word);
    }
    pub fn create_primv_word(&mut self, word: ForthWord<'a>) {
        self.index.insert(word.name.clone(), word);
    }
    pub fn compile(&mut self, name: &str) {
        match self.find(name) {
            Some(w) => {
                match w.wtype {
                WordType::Primv => {
                let func = DefineItem::Func(w.func);
                self.defines.push(func);
                },
                WordType::Dict => {
                    let def = DefineItem::Addr(w.define_ptr);
                    self.defines.push(def);
                },
                _ => {panic!("wrong word type")},
            }},
            None => println!("{} not found", name),
        }
    }

    pub fn compile_lit(&mut self, val: i32) {
        self.defines.push(DefineItem::Lit(val));
    }

    pub fn get_lit(&mut self, pos: usize) -> i32 {
        match self.defines[pos] {
            DefineItem::Lit(n) => n,
            _ => panic!("Wrong item in defines"),
        }
    }
    fn init(&mut self) {}

    pub fn tod(&self) -> usize {
        self.defines.len()
    }

    pub fn put_lit(&mut self, pos: usize, to_val: usize) {
        self.defines[pos] = DefineItem::Lit(to_val as i32);
    }

    pub fn get_define(&self, pos: usize) -> &DefineItem<'a> {
        &self.defines[pos]
    }

    pub fn get_define_len(&self) -> usize {
        self.defines.len()
    }
}

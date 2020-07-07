use crate::core::ForthCore;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WordType {
    Primv,
    Dict,
    Const,
    Var,
}

#[derive(Clone)]
pub struct ForthWord<'a> {
    pub name: String,
    pub func: fn(&mut ForthCore<'a>),
    //pub defines: Defines,
    pub wtype: WordType,
    pub immediate: bool,
    pub define_ptr: usize,
}

impl<'a> fmt::Display for ForthWord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            ":{:<8} func:{:X} {:?} immed?{}",
            self.name, self.func as u64, self.wtype, self.immediate
        )
    }
}

impl<'a> fmt::Debug for ForthWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d_ptr = match self.wtype {
            WordType::Dict => format!("Dict  d_ptr:{}",self.define_ptr),
            _ => "Primv".to_string(),
        };
        write!(
            f,
            "{:<8}   func:{:X}  immed?{:<5} {}",
            self.name,  self.func as u64, self.immediate,d_ptr
        )
    }
}
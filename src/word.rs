use crate::core::Defines;
use crate::core::ForthCore;
use std::fmt;


#[derive(Clone, Copy, Debug)]
pub enum WordType {
    Primv,
    Dict,
    Lit,
    Imed, //immediate
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
        write!(
            f,
            "{:<8}  d_prt:{} func:{:X} {:?} immed?{}",
            self.name, self.define_ptr, self.func as u64, self.wtype, self.immediate
        )
    }
}
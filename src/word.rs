use crate::core::Defines;
use crate::core::ForthCore;
use std::fmt;


#[derive(Clone, Copy, Debug)]
pub enum WordType {
    Internal,
    Dict,
    Lit,
    Imed, //immediate
    Const,
    Var,
}

#[derive(Clone)]
pub struct ForthWord<'a> {
    pub name: String,
    pub func: fn(&mut ForthCore<'a>, defines: &ForthWord),
    pub defines: Defines,
    pub wtype: WordType,
    pub immediate: bool,
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
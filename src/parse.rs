
use std::fmt;
use ast::{Ast, Repeater, Flags};

pub struct Error {
    pub pos: uint,
    pub msg: String,
}
impl fmt::Show for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "syntax error near position {}: {}",
               self.pos, self.msg)
    }
}
pub fn err<T>(msg: &str, pos: uint) -> Result<T, Error> {
    Err(Error {
        pos: pos, //self.chari,
        msg: msg.to_string(),
    })
}

pub fn parse(src: &str) -> Result<Ast, Error> { fail!("not implemented") }



// below is taken from libregex


/// Represents the abstract syntax of a regular expression.
/// It is showable so that error messages resulting from a bug can provide
/// useful information.
/// It is cloneable so that expressions can be repeated for the counted
/// repetition feature. (No other copying is done.)
///
/// Note that this representation prevents one from reproducing the regex as
/// it was typed. (But it could be used to reproduce an equivalent regex.)
#[deriving(Show, Clone)]
pub enum ReAst {
    ReNothing,
    ReLiteral(char, Flags),
    ReDot(Flags),
    ReAstClass(Vec<(char, char)>, Flags),
    ReBegin(Flags),
    ReEnd(Flags),
    ReWordBoundary(Flags),
    ReCapture(uint, Option<String>, Box<Ast>),
    // Represent concatenation as a flat vector to avoid blowing the
    // stack in the compiler.
    ReCat(Vec<Ast>),
    ReAlt(Box<Ast>, Box<Ast>),
    ReRep(Box<Ast>, Repeater, /*Greed*/),
}





use std::fmt;

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

#[deriving(Show,Clone)]
pub enum Ast {
	Empty,				// the empty string, ε

    Lit(char, Flags),
    Dot(Flags),
    Cls(Vec<(char, char)>, Flags),

	Seq(Vec<Ast>), 			// sequence, e1 followed by e2 ...
	Alt(Vec<Ast>),			// ordered choice, e1 / e2 ...
	Rep(Box<Ast>, Repeater),// e*, e+, e?
	And(Box<Ast>),			// &e
	Not(Box<Ast>)			// !e
}


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

#[deriving(Show, PartialEq, Clone)]
pub enum Repeater {
    ZeroOne,
    ZeroMore,
    OneMore,
}

/// Flags represents all options that can be twiddled by a user in an
/// expression.
pub type Flags = u8;

pub const FLAG_EMPTY:      u8 = 0;
pub const FLAG_NOCASE:     u8 = 1 << 0; // i
//pub const FLAG_MULTI:      u8 = 1 << 1; // m
//pub const FLAG_DOTNL:      u8 = 1 << 2; // s
//pub const FLAG_SWAP_GREED: u8 = 1 << 3; // U
pub const FLAG_NEGATED:    u8 = 1 << 4; // char class or not word boundary

fn parse(src: &str) -> Result<Ast, Error> { fail!("not implemented") }

fn err<T>(msg: &str) -> Result<T, Error> {
    Err(Error {
        pos: 0, //self.chari,
        msg: msg.to_string(),
    })
}



use ast::{Ast, Flags, Nil,Lit,Dot,Cls,Seq,Alt,Rep,And,Not,Cap};
use code::*;


#[deriving(Clone)]
pub struct Program {
    /// A sequence of instructions.
    pub insts: Vec<Opcode>,
    // /// If the regular expression requires a literal prefix in order to have a
    // /// match, that prefix is stored here. (It's used in the VM to implement
    // /// an optimization.)
    // pub prefix: String,
}

impl Program {
    /// Compiles a Regex given its AST.
    pub fn new(ast: Ast) -> Program { //(Program, Vec<Option<String>>)
        let mut c = Compiler {
            insts: Vec::with_capacity(100),
            //names: Vec::with_capacity(10),
        };

        //c.insts.push(IOpenCapture(0));
        c.compile(ast);
        //c.insts.push(ICloseCapture(1));
        c.insts.push(IEnd);

        //...

        Program {
        	insts: c.insts
        }
    }
}
struct Compiler {
	insts: Vec<Opcode>,
	//names: Vec<Option<String>>  // named groups
}
impl Compiler {
	fn compile(&mut self, ast: Ast) {
		match ast {
			//Nil => {/*ε, no opcode needed*/}
			Lit(s, flags) => {
				for ch in s.as_slice().chars() {
					self.push(IChar(ch, flags));
				}
			}
			Dot(flags) => { self.push(IAny(flags)); }
			Cls(cls, flags) => { fail!("char classes not implemented"); }
			Seq(es) => {
				for e in es.into_iter() {
					self.compile(e);
				}
			}
			Alt(es) => {}
			Rep(e, rep) => {}
			And(e) => {}
			Not(e) => {}
			Cap(num, name, e) => {}

    		//Nil,              // the empty string, ε
    		//Lit(char, Flags),
    		//Dot(Flags),
    		//Cls(Vec<(char, char)>, Flags),
    		//Seq(Vec<Ast>),      // sequence, e1 followed by e2 ...
    		//Alt(Vec<Ast>),      // ordered choice, e1 / e2 ...
    		//Rep(Box<Ast>, Repeater),// e*, e+, e?
    		//And(Box<Ast>),      // &e, lookahead predicate
    		//Not(Box<Ast>),      // !e, neg-lookahead pred
    		//// match beginning-of-input, eoi, or wordbound (libregex)
    		//// Begin(Flags), End(Flags), WordBoundary(Flags),
    		//Cap(uint, Option<String>, Box<Ast>), // numbered, optionally named, capture


			_ => { fail!("not implemented: {}", ast) }
		}
	}

    /// Appends the given instruction to the program.
    #[inline]
    fn push(&mut self, x: Opcode) {
        self.insts.push(x)
    }

//    /// Appends an *empty* `Split` instruction to the program and returns
//    /// the index of that instruction. (The index can then be used to "patch"
//    /// the actual locations of the split in later.)
//    #[inline]
//    fn empty_split(&mut self) -> InstIdx {
//        self.insts.push(Split(0, 0));
//        self.insts.len() - 1
//    }
//
//    /// Sets the left and right locations of a `Split` instruction at index
//    /// `i` to `pc1` and `pc2`, respectively.
//    /// If the instruction at index `i` isn't a `Split` instruction, then
//    /// `fail!` is called.
//    #[inline]
//    fn set_split(&mut self, i: InstIdx, pc1: InstIdx, pc2: InstIdx) {
//        let split = self.insts.get_mut(i);
//        match *split {
//            Split(_, _) => *split = Split(pc1, pc2),
//            _ => fail!("BUG: Invalid split index."),
//        }
//    }
//
//    /// Appends an *empty* `Jump` instruction to the program and returns the
//    /// index of that instruction.
//    #[inline]
//    fn empty_jump(&mut self) -> InstIdx {
//        self.insts.push(Jump(0));
//        self.insts.len() - 1
//    }
//
//    /// Sets the location of a `Jump` instruction at index `i` to `pc`.
//    /// If the instruction at index `i` isn't a `Jump` instruction, then
//    /// `fail!` is called.
//    #[inline]
//    fn set_jump(&mut self, i: InstIdx, pc: InstIdx) {
//        let jmp = self.insts.get_mut(i);
//        match *jmp {
//            Jump(_) => *jmp = Jump(pc),
//            _ => fail!("BUG: Invalid jump index."),
//        }
//    }
}

/*
#[deriving(Show, Clone)]
enum Inst {
	Match,
	Any,
	One(char),
	Cls(Vec<(char,char)>, Flags),
	Seq(Vec<Inst>),
	Alt(Vec<Inst>),
	Rep(Box<Inst>),

	Save(uint),
	Jump(Idx),
	Split(Idx,Idx)
}

type Idx = uint;

// example below from libregex


#[deriving(Show, Clone)]
pub enum ReInst {
    // When a Match instruction is executed, the current thread is successful.
    ReMatch,

    // The OneChar instruction matches a literal character.
    // The flags indicate whether to do a case insensitive match.
    ReOneChar(char, Flags),

    // The CharClass instruction tries to match one input character against
    // the range of characters given.
    // The flags indicate whether to do a case insensitive match and whether
    // the character class is negated or not.
    ReCharClass(Vec<(char, char)>, Flags),

    // Matches any character except new lines.
    // The flags indicate whether to include the '\n' character.
    ReAny(Flags),

    // Matches the beginning of the string, consumes no characters.
    // The flags indicate whether it matches if the preceding character
    // is a new line.
    //EmptyBegin(Flags),

    // Matches the end of the string, consumes no characters.
    // The flags indicate whether it matches if the proceeding character
    // is a new line.
    //EmptyEnd(Flags),

    // Matches a word boundary (\w on one side and \W \A or \z on the other),
    // and consumes no character.
    // The flags indicate whether this matches a word boundary or something
    // that isn't a word boundary.
    //EmptyWordBoundary(Flags),

    // Saves the current position in the input string to the Nth save slot.
    ReSave(uint),

    // Jumps to the instruction at the index given.
    ReJump(Idx),

    // Jumps to the instruction at the first index given. If that leads to
    // a failing state, then the instruction at the second index given is
    // tried.
    ReSplit(Idx, Idx),
}

/// Program represents a compiled regular expression. Once an expression is
/// compiled, its representation is immutable and will never change.
///
/// All of the data in a compiled expression is wrapped in "MaybeStatic" or
/// "MaybeOwned" types so that a `Program` can be represented as static data.
/// (This makes it convenient and efficient for use with the `regex!` macro.)
#[deriving(Clone)]
pub struct Program {
    /// A sequence of instructions.
    pub insts: Vec<Inst>,
    // /// If the regular expression requires a literal prefix in order to have a
    // /// match, that prefix is stored here. (It's used in the VM to implement
    // /// an optimization.)
    // pub prefix: String,
}

impl Program {
    /// Compiles a Regex given its AST.
    pub fn new(ast: Ast) -> Program { //(Program, Vec<Option<String>>)
        let mut c = Compiler {
            insts: Vec::with_capacity(100),
            names: Vec::with_capacity(10),
        };

        c.insts.push(Save(0));
        c.compile(ast);
        c.insts.push(Save(1));
        c.insts.push(Match);

        //...

        Program {
        	insts: c.insts
        }
    }
}
struct Compiler {
	insts: Vec<Inst>,
	names: Vec<Option<String>>  // klk named groups
}
impl Compiler {
	fn compile(&self, ast: Ast) -> Ast { Empty }
}*/
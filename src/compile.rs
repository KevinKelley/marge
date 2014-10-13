
use parse::{Ast, Flags, Empty};

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
}

type Idx = uint;

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
	names: Vec<Option<String>>
}
impl Compiler {
	fn compile(&self, ast: Ast) -> Ast { Empty }
}
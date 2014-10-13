
use std::fmt;

pub struct Error {
    pub pos: uint,
    pub msg: String,
}
impl fmt::Show for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Regex syntax error near position {}: {}",
               self.pos, self.msg)
    }
}

struct CharSet {
	hugely_wrong: Vec<char>
}
enum Ast {
	Empty,				// the empty string, ε
	Term(String), 		// any terminal symbol
	Nonterm(String),	// any nonterminal symbol
	// or
	Literal(String),
	CharClass(CharSet),
	NonTerminal(String),		//

	Sequence(Box<Ast>,Box<Ast>), 	// sequence, e1 followed by e2
	Choice(Box<Ast>,Box<Ast>),		// ordered choice, e1 / e2
	ZeroOrMore(Box<Ast>),			// zero-or-more, e*
	OneOrMore(Box<Ast>),			// one-or-more, e+
	Optional(Box<Ast>),				// zero-or-one, e?
	AndPredicate(Box<Ast>),			// &e
	NotPredicate(Box<Ast>)			// !e
}

//! license MIT
//!
//! This is basically Lua's LPEG, which is similar to Russ Cox's RE2;
//! patterns are compiled to a small language for a VM (Pike VM) that parses in linear time...
//! see the papers, starting with Russ Cox's; then the LPEG one.
//! Andrew? Gallant's libregex which is now part of rust distro, is where I stole
//! whatever's particularly nice here.
//!


#![feature(macro_rules, phase)]
#![feature(struct_variant)]
#![allow(dead_code, unused_variable)]

// Unicode tables for character classes are defined in libunicode
extern crate unicode;

pub use parse::Error;
//pub use std::collections::HashMap;

mod ast;
mod parse;
mod compile;
mod capture;
mod peg;
mod vm;

// parse a string to an AST
// compile the AST to a Program
// recognize the Program against an input string, (true or false)
// match the Program against an input, (vec<Capture>)
// run the Program with an input, an environemt of functions, and a state-stack,
//     producing side effects and a final top-of-stack value.



// pattern <- grammar / simplepatt
// grammar <- (nonterminal ’<-’ sp simplepatt)+
// simplepatt <- alternative (’/’ sp alternative)*
// alternative <- ([!&]? sp suffix)+
// suffix <- primary ([*+?] sp)*
// primary <- ’(’ sp pattern ’)’ sp / ’.’ sp / literal /
// charclass / nonterminal !’<-’
// literal <- [’] (![’] .)* [’] sp
// charclass <- ’[’ (!’]’ (. ’-’ . / .))* ’]’ sp
// nonterminal <- [a-zA-Z]+ sp
// sp <- [ \t\n]*

/// from lua's lpeg, top level pattern may be a grammar or a simple pattern.
/// if simple pattern, like "[a-z]+", it's a succinct way of writing a grammar
/// like "S <- [a-z]+".
enum Pattern {
	PGrammar(Grammar),
	PSimplePat(SimplePattern)
}
struct Grammar {
	start: Symbol,
	patterns: Vec<(Symbol,SimplePattern)> // or HashMap<Symbol, SimplePattern> // or
}

struct SimplePattern {
	alternatives: Vec<Alternative>	// alt_1 / ... / alt_n
}
struct Alternative {
	elems: Vec<Elem>
}
struct Elem {
	predicate: Option<Lookahead>,
	suffix: Suffix
}
enum Lookahead {
	Positive,	// &x
	Negative	// !x
}
struct Suffix {
	primary: Primary,
	repetition: Option<RepetitionQualifier>
}
enum Primary {
	PPattern(SimplePattern),
	PAnyOneChar, // .
	PLiteral(String),
	PCharClass(CharClass),
	PNonterminal(Symbol)
}
enum RepetitionQualifier {
	ZeroOrMore,  	// *
	OneOrMore,		// +
	ZeroOrOne		// ?
}
struct CharClass {
	items: Vec<CharClassItem>
}
enum CharClassItem {
	CChar(char),
	CRange(char,char)
}
struct Symbol(String);

fn test() {
// pattern <- grammar / simplepatt
// sym("pattern") ::= simple(alt(nonterm("grammar"), nonterm("simplepatt")))
	let pattern = simple(
		vec!(
			Alternative{elems: vec!(elem(nonterm("grammar"   ))) },
			Alternative{elems: vec!(elem(nonterm("simplepatt"))) },
		)
	);

// grammar <- (nonterminal ’<-’ sp simplepatt)+
	let grammar = simple(
		vec!(
			Alternative {elems: vec!(
				Elem{
					predicate:None,
					suffix:Suffix{
						primary:PPattern(
							simple(
								vec!(
									Alternative {elems: vec!(
										elem(nonterm("nonterminal")),
										elem(lit("<-")),
										elem(nonterm("sp")),
										elem(nonterm("simplepatt")),
									)}
								)
							)
						),
						repetition:Some(OneOrMore)}
				}
			)}
		)
	);
	let simplepatt = simple(vec!());
	let alternative = simple(vec!());
	let suffix = simple(vec!());
	let primary = simple(vec!());
	let charclass = simple(vec!());
	let literal = simple(vec!());
	let charclass = simple(vec!());
	let nonterminal = simple(vec!());
	let sp = simple(vec!());

	let g: Pattern =
		PGrammar(Grammar{
			start: Symbol("Start".into_string()),
			patterns: vec!(
				(sym("pattern"), pattern),
				(sym("grammar"), grammar),
				(sym("simplepatt"), simplepatt),
				(sym("alternative"), alternative),
				(sym("suffix"), suffix),
				(sym("primary"), primary),
				//(sym("charclass"), charclass),
				(sym("literal"), literal),
				(sym("charclass"), charclass),
				(sym("nonterminal"), nonterminal),
				(sym("sp"), sp)
			)
		});
}
// pattern <- grammar / simplepatt
//sym("pattern") ::= simple(alt(nonterm("grammar"), nonterm("simplepatt")));

// grammar <- (nonterminal ’<-’ sp simplepatt)+
//sym("grammar") ::= plus(seq(nonterm("nonterminal"), lit("<-"), nonterm("sp"), nonterm("simplepatt")));

// simplepatt <- alternative (’/’ sp alternative)*
//sym("simplepatt") ::= seq(nonterm("alternative"), star(seq(lit("/"), nonterm("sp"), nonterm("alternative"))));

// alternative <- ([!&]? sp suffix)+
//sym("alternative") ::= plus(seq(opt(oneof("!&")), nonterm("sp"), nonterm("suffix")));

// suffix <- primary ([*+?] sp)*
//sym("suffix") ::= seq(nonterm("primary"), star(seq(oneof("*+?"), nonterm("sp"))));

// primary <- ’(’ sp pattern ’)’ sp / ’.’ sp / literal / oneof / nonterminal !’<-’
//sym("primary") ::=
//	alt(
//		seq(lit("("), nonterm("sp"), nonterm("pattern"), lit(")"), nonterm("sp")),
//		alt(
//			seq(dot, nonterm("sp")),
//			alt(
//				nonterm("literal"),
//				alt(
//					nonterm("oneof"),
//					seq(nonterm("nonterminal"), not(lit("<-")))
//				)
//			)
//		)
//	);

// literal <- [’] (![’] .)* [’] sp
//sym("literal") ::= seq(lit("'"), star(seq(not(lit("'")), dot)), lit("'"), nonterm("sp"));

// charclass <- ’[’ (!’]’ (. ’-’ . / .))* ’]’ sp
//sym("charclass") ::= seq(lit("["), star( alt(seq(dot,lit("-"),dot), dot) ), lit("]"), nonterm("sp"));

// nonterminal <- [a-zA-Z]+ sp
//sym("nonterminal") ::= seq(plus(charclass("a-zA-Z")), nonterm("sp"));

// sp <- [ \t\n]*
//sym("sp") ::= star(oneof(" \t\n"));

fn sym(s: &str) -> Symbol { Symbol(s.into_string()) }

fn nonterm(s:&str) -> Primary { PNonterminal(sym(s)) }
fn lit(s: &str) -> Primary { PLiteral(s.into_string()) }
fn simple(alts: Vec<Alternative>) -> SimplePattern { SimplePattern { alternatives: alts } }
fn elem(p: Primary) -> Elem { Elem { predicate:None, suffix:Suffix{primary:p, repetition:None } } }

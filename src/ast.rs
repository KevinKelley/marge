
//use capture::CapKind;

#[deriving(Show,Clone)]
struct CharSet {
    ranges: Vec<(char,char)>
}
#[deriving(Show,Clone)]
pub enum Ast {
    Nil,              // the empty string, ε
    Lit(String, Flags),
    Dot(Flags),
    Cls(Vec<(char, char)>, Flags),
    Seq(Vec<Ast>),      // sequence, e1 followed by e2 ...
    Alt(Vec<Ast>),      // ordered choice, e1 / e2 ...
    Rep(Box<Ast>, Repeater),// e*, e+, e?
    And(Box<Ast>),      // &e, lookahead predicate
    Not(Box<Ast>),      // !e, neg-lookahead pred

    Cap(uint, Option<String>, Box<Ast>), // numbered, optionally named, capture

    // lpeg
    TChar(char), TSet(CharSet), TAny,  /* standard PEG elements */
    TTrue, TFalse,
    TRep(Box<Ast>),
    TSeq(Box<Ast>,Box<Ast>), TChoice(Box<Ast>,Box<Ast>),
    TNot(Box<Ast>), TAnd(Box<Ast>),
    TCall,
    TOpenCall,
    TRule(Box<Ast>,Box<Ast>),  /* sib1 is rule's pattern, sib2 is 'next' rule */
    TGrammar(Box<Ast>),  /* sib1 is initial (and first) rule */
    TBehind(Box<Ast>),  /* match behind */
    TCapture(Box<Ast>),  /* regular capture */
    TRunTime(Box<Ast>)  /* run-time capture */
    // number of siblings for each tree */
    //  0, 0, 0,  /* char, set, any */
    //  0, 0,   /* true, false */
    //  1,    /* rep */
    //  2, 2,   /* seq, choice */
    //  1, 1,   /* not, and */
    //  0, 0, 2, 1,  /* call, opencall, rule, grammar */
    //  1,  /* behind */
    //  1, 1  /* capture, runtime capture */
}

#[deriving(Show, PartialEq, Clone)]
pub enum Repeater {
    ZeroOne,
    ZeroMore,
    OneMore,
}

pub type Flags = u8;

pub const FLAG_NORMAL:     u8 = 0;
pub const FLAG_NOCASE:     u8 = 1 << 0; // i
//pub const FLAG_MULTI:      u8 = 1 << 1; // m
//pub const FLAG_DOTNL:      u8 = 1 << 2; // s
//pub const FLAG_SWAP_GREED: u8 = 1 << 3; // U
pub const FLAG_NEGATED:    u8 = 1 << 4; // char class or not word boundary



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

fn mk_peg_grammar() -> Ast {
    let pattern = sp();
    let grammar = sp();
    let alternative = sp();
    let simplepatt = sp();
    let letter = Cls(vec!(('a','z'), ('A','Z')), FLAG_NORMAL);
    let nonterm = seq(some(letter), sp());
    let charclass = seq4(
        lit("["),
        many(seq(not(lit("]")),
                 alt(Seq(vec!(dot(), lit("-"), dot())),
                     dot())
            )),
        lit("]"),
        sp());
    let literal = seq4(
        lit("'"),
        many(seq(not(lit("'")), dot())),
        lit("'"),
        sp()
        );
    let primary = alt5(
        seq5(lit("("), sp(), pattern, lit(")"), sp()),
        seq(dot(), sp()),
        literal,
        charclass,
        seq(nonterm.clone(), not(lit("<-")))
        );
    let suffix = seq(
        primary,
        many(seq(alt3(lit("*"),lit("+"),lit("?")), sp())) );
    let alternative = some(seq3(
            opt(alt(lit("!"),lit("&"))),
            sp(),
            suffix));
    let simplepatt = alt(alternative.clone(), many(seq3(lit("/"),sp(),alternative)));
    let grammar = some(seq4(
        nonterm, lit("<-"), sp(), simplepatt.clone()
        ));
    let pattern = alt(grammar, simplepatt);

    pattern
}
fn many(ast: Ast) -> Ast { Rep(box ast, ZeroMore) }
fn some(ast: Ast) -> Ast { Rep(box ast, OneMore) }
fn opt(ast: Ast) -> Ast { Rep(box ast, ZeroOne) }
fn seq(a1:Ast, a2:Ast) -> Ast { Seq(vec!(a1,a2)) }
fn seq3(a1:Ast, a2:Ast, a3:Ast) -> Ast { Seq(vec!(a1,a2,a3)) }
fn seq4(a1:Ast, a2:Ast, a3:Ast, a4:Ast) -> Ast { Seq(vec!(a1,a2,a3,a4)) }
fn seq5(a1:Ast, a2:Ast, a3:Ast, a4:Ast, a5:Ast) -> Ast { Seq(vec!(a1,a2,a3,a4,a5)) }
fn alt(a1:Ast, a2:Ast) -> Ast { Alt(vec!(a1,a2)) }
fn alt3(a1:Ast, a2:Ast, a3:Ast) -> Ast { Alt(vec!(a1,a2,a3)) }
fn alt4(a1:Ast, a2:Ast, a3:Ast, a4:Ast) -> Ast { Alt(vec!(a1,a2,a3,a4)) }
fn alt5(a1:Ast, a2:Ast, a3:Ast, a4:Ast, a5:Ast) -> Ast { Alt(vec!(a1,a2,a3,a4,a5)) }
fn lit(s: &str) -> Ast { Lit(s.to_string(), FLAG_NORMAL) }
fn not(ast: Ast) -> Ast { Not(box ast) }
fn dot() -> Ast { Dot(FLAG_NORMAL) }
fn sp() -> Ast { many(Alt(vec!(lit(" "),lit("\t"),lit("\n")))) }


// below from lpeg


// 'Tree' is what the Lua impl calls its AST.
// Tree is produced by parsing (from text) a PEG grammar, yielding an Ast.
// (or, by manually building it in code, for a bootstrap grammar).
// The Ast tree is then compiled into a sequence (Vec) of Opcodes.
// This program can be executed by a virtual machine, or could be
// compiled further to native code (see libregex).

/*
** types of trees
*/
//enum TTag {
//  TChar = 0, TSet, TAny,  /* standard PEG elements */
//  TTrue, TFalse,
//  TRep,
//  TSeq, TChoice,
//  TNot, TAnd,
//  TCall,
//  TOpenCall,
//  TRule,  /* sib1 is rule's pattern, sib2 is 'next' rule */
//  TGrammar,  /* sib1 is initial (and first) rule */
//  TBehind,  /* match behind */
//  TCapture,  /* regular capture */
//  TRunTime  /* run-time capture */
//}

/* number of siblings for each tree */
//extern const byte numsiblings[];


/*
** Tree trees
** The first sibling of a tree (if there is one) is immediately after
** the tree.  A reference to a second sibling (ps) is its position
** relative to the position of the tree itself.  A key in ktable
** uses the (unique) address of the original tree that created that
** entry. NULL means no data.
*/
//struct TTree {
//  tag: TTag,
//  cap: CapKind,  /* kind of capture (if it is a capture) */
//  key: u16,  /* key in ktable for Lua data (0 if no key) */
//  //union {
//  //  int ps;  /* occasional second sibling */
//  //  int n;  /* occasional counter */
//  //} u,
//}


/*
** A complete pattern has its tree plus, if already compiled,
** its corresponding code
*/
//typedef struct Pattern {
//  union Instruction *code;
//  int codesize;
//  TTree tree[1];
//} Pattern;


/* number of siblings for each tree */
//extern const byte numsiblings[];

/* access to siblings */
//#define sib1(t)         ((t) + 1)
//#define sib2(t)         ((t) + (t)->u.ps)

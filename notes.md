##Wikipedia: Parsing Expression Grammar

###Syntax

Formally, a parsing expression grammar consists of:
- A finite set N of nonterminal symbols.
- A finite set Σ of terminal symbols that is disjoint from N.
- A finite set P of parsing rules.
- An expression eS termed the starting expression.

Each parsing rule in P has the form A ← e, where A is a nonterminal symbol and e is a parsing expression. A parsing expression is a hierarchical expression similar to a regular expression, which is constructed in the following fashion:
1.  An atomic parsing expression consists of:
	- any terminal symbol,
	- any nonterminal symbol, or
	- the empty string ε.
2.  Given any existing parsing expressions e, e1, and e2, a new parsing expression can be constructed using the following operators:
	- Sequence: e1 e2
	- Ordered choice: e1 / e2
	- Zero-or-more: e*
	- One-or-more: e+
	- Optional: e?
	- And-predicate: &e
	- Not-predicate: !e



##A Text Pattern-Matching Tool based on Parsing Expression Grammars --Roberto Ierusalimschy1

###PEG grammar of PEG grammar

	grammar <- (nonterminal ’<-’ sp pattern)+
	pattern	 <- alternative (’/’ sp alternative)*
	alternative <- ([!&]? sp suffix)+
	suffix 	<- primary ([*+?] sp)*
	primary	<- ’(’ sp pattern ’)’ sp / ’.’ sp / literal /
		   charclass / nonterminal !’<-’
	literal	<- [’] (![’] .)* [’] sp
	charclass <- ’[’ (!’]’ (. ’-’ . / .))* ’]’ sp
	nonterminal <- [a-zA-Z]+ sp
	sp <- [ \t\n]*


LPEG grammar (focusses more on pattern matching)

	pattern <- grammar / simplepatt
	grammar <- (nonterminal ’<-’ sp simplepatt)+
	simplepatt <- alternative (’/’ sp alternative)*
	alternative <- ... (as before)

LPEG Parsing Machine

Figure 2. basic instructions for the parsing machine:

	p, i, e, c 		Char x, S[i] = x 	⇒ p + 1, i + 1, e, c
	p, i, e, c 		Char x, S[i] 6= x 	⇒ Fail, i, e, c
	p, i, e, c 		Jump l 				⇒ p + l, i, e, c
	p, i, e, c 		Choice l 			⇒ p + 1, i, (p + l, i, c) : e, c
	p, i, e, ci 	Call l 				⇒ p + l, i, (p + 1) : e, c
	p0, i, p1:e c 	Return 				⇒ p1, i, e, c
	p, i, h:e, c 	Commit l 			⇒ p + l, i, e, c
	p, i, e, c 		Capture k 			⇒ p + 1, i, e, (i, p) : c
	p, i, e, c 		Fail 				⇒ Fail, i, e, c
	Fail, i, p:e, c any 				⇒ Fail, i, e, c
	Fail,i0, (p,i1,c1):e,c0	any 		⇒ p, i1, e, c1


Figure 3. extra instructions for the parsing machine:

	p, i, e, c 		Charset X, S[i] ∈ X 	⇒ hp + 1, i + 1, e, c
	p, i, e, c 		Charset X, S[i] 6∈ X 	⇒ hFail, i, e, c
	p, i, e, c 		Any n, i + n ≤ |S| 	⇒ hp + 1, i + n, e, c
	p, i, e, c 		Any n, i + n > |S| 	⇒ hFail, i, e, c
	p0,i0,(p1,i1,c1):e,c0 	PartialCommit l 	⇒ hp0 + l, i0, (p1, i0, c0) : e, c0
	p, i, e, c 		Span X, S[i] ∈ X 	⇒ hp, i + 1, e, c
	p, i, e, c 		Span X, S[i] 6∈ X 	⇒ hp + 1, i, e, c
	p, i, h : e, c 		FailTwice 		⇒ hFail, i, e, c
	p0,i0,(p1,i1,c1):e,c0 	BackCommit l 		⇒ hp0 + l, i1, e, c1





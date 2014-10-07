
PEG grammar of PEG grammar

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

p, i, e, c 		Char x, S[i] = x 	⇒ p + 1, i + 1, e, c
p, i, e, c 		Char x, S[i] 6= x 	⇒ Fail, i, e, c
p, i, e, c 		Jump l 			⇒ p + l, i, e, c
p, i, e, c 		Choice l 		⇒ p + 1, i, (p + l, i, c) : e, c
p, i, e, ci 		Call l 			⇒ p + l, i, (p + 1) : e, c
p0, i, p1 : e, c 	Return 			⇒ p1, i, e, c
p, i, h : e, c 		Commit l 		⇒ p + l, i, e, c
p, i, e, c 		Capture k 		⇒ p + 1, i, e, (i, p) : c
p, i, e, c 		Fail 			⇒ Fail, i, e, c
Fail, i, p : e, c 	any 			⇒ Fail, i, e, c
Fail,i0, (p,i1,c1):e,c0	any 			⇒ p, i1, e, c1

Figure 2. basic instructions for the parsing machine


p, i, e, c 		Charset X, S[i] ∈ X 	⇒ hp + 1, i + 1, e, c
p, i, e, c 		Charset X, S[i] 6∈ X 	⇒ hFail, i, e, c
p, i, e, c 		Any n, i + n ≤ |S| 	⇒ hp + 1, i + n, e, c
p, i, e, c 		Any n, i + n > |S| 	⇒ hFail, i, e, c
p0,i0,(p1,i1,c1):e,c0 	PartialCommit l 	⇒ hp0 + l, i0, (p1, i0, c0) : e, c0
p, i, e, c 		Span X, S[i] ∈ X 	⇒ hp, i + 1, e, c
p, i, e, c 		Span X, S[i] 6∈ X 	⇒ hp + 1, i, e, c
p, i, h : e, c 		FailTwice 		⇒ hFail, i, e, c
p0,i0,(p1,i1,c1):e,c0 	BackCommit l 		⇒ hp0 + l, i1, e, c1

Figure 3. extra instructions for the parsing machine




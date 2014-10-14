
//use capture::Capture;

struct CodeIdx(uint);
struct CharNum(uint);
struct Capture(CharNum, CodeIdx);
struct CapIdx(uint);

enum StackEntry {
  ReturnTo(Option<CodeIdx>),
  AlternateTo(CodeIdx, CharNum, CapIdx)
}
struct State {
  // current instruction index, or FAIL (-1 or None)
  p: Option<CodeIdx>,
  // current subject position (char index in subject string) (NOT byte index)
  i: CharNum,
  // stack-entry: either (uint) return-pos, or (next-pos, subject-pos, cap-list)
  e: StackEntry,
  // Capture pointer?
  c: CapIdx
}
struct VmState(
  // current instruction index, or FAIL (-1 or None)
  Option<CodeIdx>,
  // current subject position (char index in subject string) (NOT byte index)
  CharNum,
  // stack-entry: either (uint) return-pos, or (next-pos, subject-pos, cap-list)
  StackEntry,
  // Capture pointer?
  CapIdx
);

struct Vm {
  program: Vec<Opcode>,
  text: Vec<char>,
  stack: Vec<State>,
  captures: Vec<Capture>
}
#[allow(unused_mut)]
impl Vm {
  fn new(program: Vec<Opcode>, text: Vec<char>) -> Vm {
    Vm {
      program: program,
      text: text,
      stack: vec!(),
      captures: vec!()
    }
  }

  //Figure 2. basic instructions for the parsing machine:
  //
  //  p,i,e,c       Char x,S[i] = x   ⇒ p+1,i+1,e,c
  //  p,i,e,c       Char x,S[i] 6= x  ⇒ Fail,i,e,c
  //  p,i,e,c       Jump l            ⇒ p+l,i,e,c
  //  p,i,e,c       Choice l          ⇒ p+1,i,(p+l,i,c):e,c
  //  p,i,e,ci      Call l            ⇒ p+l,i,(p+1):e,c
  //  p0,i,p1:e c   Return            ⇒ p1,i,e,c
  //  p,i,h:e,c     Commit l          ⇒ p+l,i,e,c
  //  p,i,e,c       Capture k         ⇒ p+1,i,e,(i,p) : c
  //  p,i,e,c       Fail              ⇒ Fail,i,e,c
  //  Fail,i,p:e,c  any               ⇒ Fail,i,e,c
  //  Fail,i0,(p,i1,c1):e,c0 any      ⇒ p,i1,e,c1

  fn step(&self, VmState(p,i,e,c): VmState) -> VmState {
    match (p,i,e,c) {
      (None,_,_,_) => VmState(None,i,e,c), // infinite halt-loop
      (Some(CodeIdx(pc)),CharNum(i),e,c) => {
        let op = self.program[pc];
        match op {
          IChar(x) if x == self.text[i] => VmState(Some(CodeIdx(pc+1)),CharNum(i+1),e,c),
          IChar(x) if x != self.text[i] => VmState(None,CharNum(i),e,c),
          _ => VmState(p,CharNum(i),e,c)
        }
      }
    }
//    let op = self.program[p];
//    match op {
//      Some(IAny) => { VmState(p,i,e,c) }
//      _ => { VmState(p,i,e,c) }
//    }
  }
  /// match a string input, and return number of characters (not bytes) matched.
  /// should this be non-self method that creates an internal private Vm to run?
  fn do_match(&mut self, input: &str) -> Option<CharNum> {

    let (mut p,mut i,mut e,mut c) = (
      Some(CodeIdx(0)), // start at the beginning of code
      CharNum(0),       // and beginning of source
      ReturnTo(None),   // nowhere to return to when leaving
      CapIdx(0)         // no captures yet
    );
    'vm: loop {
      let (p,i,e,c) = match p {
        None => break 'vm,
        Some(CodeIdx(pc)) => {
          let op = self.program[pc];
          match op {
            IAny => { (p,i,e,c) }
            IChar(x) => { (p,i,e,c) }
            //...
            _ => { (p,i,e,c) }
          }
        }
      };
    };
    None
  }
}

/* Virtual Machine's instructions */
enum Opcode {
  IAny,             // if no char, fail
  IChar(char),      // if char != aux, fail
  ISet,             // if char not in buff, fail
  ITestAny,         // in no char, jump to 'offset'
  ITestChar,        // if char != aux, jump to 'offset'
  ITestSet,         // if char not in buff, jump to 'offset'
  ISpan,            // read a span of chars in buff
  IBehind,          // walk back 'aux' characters (fail if not possible)
  IRet,             // return from a rule
  IEnd,             // end of pattern
  IChoice,          // stack a choice; next fail will jump to 'offset'
  IJmp,             // jump to 'offset'
  ICall,            // call rule at 'offset'
  IOpenCall,        // call rule number 'key' (must be closed to a ICall)
  ICommit,          // pop choice and jump to 'offset'
  IPartialCommit,   // update top choice to current position and jump
  IBackCommit,      // "fails" but jump to its own 'offset'
  IFailTwice,       // pop one choice and then fail
  IFail,            // go back to saved state on choice and jump to saved offset
  IGiveup,          // internal use
  IFullCapture,     // complete capture of last 'off' chars
  IOpenCapture,     // start a capture
  ICloseCapture,
  ICloseRunTime
}

//typedef union Instruction {
//  struct Inst {
//    byte code;
//    byte aux;
//    short key;
//  } i;
//  int offset;
//  byte buff[1];
//} Instruction;
//enum Instruction {
//  Inst {
//    code: u8,
//    aux: u8,
//    key: u16,
//  },
//  Offset { off: u32 },
//  Buff { buffer: [u8, ..1] }
//}
struct Instruction {
	code: Opcode,
	aux: u8,
	key: u16
}

//int getposition (lua_State *L, int t, int i);
//void printpatt (Instruction *p, int n);
//const char *match (lua_State *L, const char *o, const char *s, const char *e,
//                   Instruction *op, Capture *capture, int ptop);
//int verify (lua_State *L, Instruction *op, const Instruction *p,
//            Instruction *e, int postable, int rule);
//void checkrule (lua_State *L, Instruction *op, int from, int to,
//                int postable, int rule);


/*
** {======================================================
** Virtual Machine
** =======================================================
*/


//struct Stack {
//  const char *s;  /* saved position (or NULL for calls) */
//  const Instruction *p;  /* next instruction */
//  int caplevel;
//} Stack;
struct Stack<'a> {
  s: uint,   /* saved position (or NULL for calls) */
  p: &'a Instruction,  /* next instruction */
  caplevel: i32
}



/*
** Opcode interpreter
*/
//const char *match (lua_State *L, const char *o, const char *s, const char *e,
//                   Instruction *op, Capture *capture, int ptop)
unsafe fn do_match (
	//lua_State *L,
	o: *const char,  // origin i think
	s: *const char,  // start, ptr to curr char
	e: *const char,  // end, ptr to EOI
	op: *const Instruction,
	capture: *const Capture,
	ptop: int
) {
  //Stack stackbase[INITBACK];
  //Stack *stacklimit = stackbase + INITBACK;
  //Stack *stack = stackbase;  /* point to first empty slot in stack */
  //int capsize = INITCAPSIZE;
  //int captop = 0;  /* point to first empty slot in captures */
  //int ndyncap = 0;  /* number of dynamic captures (in Lua stack) */
  //const Instruction *p = op;  /* current instruction */
  //stack->p = &giveup; stack->s = s; stack->caplevel = 0; stack++;
  //lua_pushlightuserdata(L, stackbase);
  let p = op;

  loop {
//#if defined(DEBUG)
//      printf("s: |%s| stck:%d, dyncaps:%d, caps:%d  ",
//             s, stack - getstackbase(L, ptop), ndyncap, captop);
//      printinst(op, p);
//      printcaplist(capture, capture + captop);
//#endif
    //assert!(stackidx(ptop) + ndyncap == lua_gettop(L) && ndyncap <= captop);
    //switch ((Opcode)p->i.code) {
	match (*p).code {
      IEnd => {
        //assert(stack == getstackbase(L, ptop) + 1);
        //capture[captop].kind = Cclose;
        //capture[captop].s = NULL;
        //return s;
      }
//      IGiveup => {
//        assert(stack == getstackbase(L, ptop));
//        return NULL;
//      }
//      IRet => {
//        assert(stack > getstackbase(L, ptop) && (stack - 1)->s == NULL);
//        p = (--stack)->p;
//        continue;
//      }
//      IAny => {
//        if (s < e) { p++; s++; }
//        else goto fail;
//        continue;
//      }
//      ITestAny => {
//        if (s < e) p += 2;
//        else p += getoffset(p);
//        continue;
//      }
//      IChar => {
//        if ((byte)*s == p->i.aux && s < e) { p++; s++; }
//        else goto fail;
//        continue;
//      }
//      ITestChar => {
//        if ((byte)*s == p->i.aux && s < e) p += 2;
//        else p += getoffset(p);
//        continue;
//      }
//      ISet => {
//        int c = (byte)*s;
//        if (testchar((p+1)->buff, c) && s < e)
//          { p += CHARSETINSTSIZE; s++; }
//        else goto fail;
//        continue;
//      }
//      ITestSet => {
//        int c = (byte)*s;
//        if (testchar((p + 2)->buff, c) && s < e)
//          p += 1 + CHARSETINSTSIZE;
//        else p += getoffset(p);
//        continue;
//      }
//      IBehind => {
//        int n = p->i.aux;
//        if (n > s - o) goto fail;
//        s -= n; p++;
//        continue;
//      }
//      ISpan => {
//        for (; s < e; s++) {
//          int c = (byte)*s;
//          if (!testchar((p+1)->buff, c)) break;
//        }
//        p += CHARSETINSTSIZE;
//        continue;
//      }
//      IJmp => {
//        p += getoffset(p);
//        continue;
//      }
//      IChoice => {
//        if (stack == stacklimit)
//          stack = doublestack(L, &stacklimit, ptop);
//        stack->p = p + getoffset(p);
//        stack->s = s;
//        stack->caplevel = captop;
//        stack++;
//        p += 2;
//        continue;
//      }
//      ICall => {
//        if (stack == stacklimit)
//          stack = doublestack(L, &stacklimit, ptop);
//        stack->s = NULL;
//        stack->p = p + 2;  /* save return address */
//        stack++;
//        p += getoffset(p);
//        continue;
//      }
//      ICommit => {
//        assert(stack > getstackbase(L, ptop) && (stack - 1)->s != NULL);
//        stack--;
//        p += getoffset(p);
//        continue;
//      }
//      IPartialCommit => {
//        assert(stack > getstackbase(L, ptop) && (stack - 1)->s != NULL);
//        (stack - 1)->s = s;
//        (stack - 1)->caplevel = captop;
//        p += getoffset(p);
//        continue;
//      }
//      IBackCommit => {
//        assert(stack > getstackbase(L, ptop) && (stack - 1)->s != NULL);
//        s = (--stack)->s;
//        captop = stack->caplevel;
//        p += getoffset(p);
//        continue;
//      }
//      case IFailTwice:
//        assert(stack > getstackbase(L, ptop));
//        stack--;
//        /* go through */
//      case IFail:
//      fail: { /* pattern failed: try to backtrack */
//        do {  /* remove pending calls */
//          assert(stack > getstackbase(L, ptop));
//          s = (--stack)->s;
//        } while (s == NULL);
//        if (ndyncap > 0)  /* is there matchtime captures? */
//          ndyncap -= removedyncap(L, capture, stack->caplevel, captop);
//        captop = stack->caplevel;
//        p = stack->p;
//        continue;
//      }
//      ICloseRunTime => {
//        CapState cs;
//        int rem, res, n;
//        int fr = lua_gettop(L) + 1;  /* stack index of first result */
//        cs.s = o; cs.L = L; cs.ocap = capture; cs.ptop = ptop;
//        n = runtimecap(&cs, capture + captop, s, &rem);  /* call function */
//        captop -= n;  /* remove nested captures */
//        fr -= rem;  /* 'rem' items were popped from Lua stack */
//        res = resdyncaptures(L, fr, s - o, e - o);  /* get result */
//        if (res == -1)  /* fail? */
//          goto fail;
//        s = o + res;  /* else update current position */
//        n = lua_gettop(L) - fr + 1;  /* number of new captures */
//        ndyncap += n - rem;  /* update number of dynamic captures */
//        if (n > 0) {  /* any new capture? */
//          if ((captop += n + 2) >= capsize) {
//            capture = doublecap(L, capture, captop, ptop);
//            capsize = 2 * captop;
//          }
//          /* add new captures to 'capture' list */
//          adddyncaptures(s, capture + captop - n - 2, n, fr);
//        }
//        p++;
//        continue;
//      }
//      ICloseCapture => {
//        const char *s1 = s;
//        assert(captop > 0);
//        /* if possible, turn capture into a full capture */
//        if (capture[captop - 1].siz == 0 &&
//            s1 - capture[captop - 1].s < UCHAR_MAX) {
//          capture[captop - 1].siz = s1 - capture[captop - 1].s + 1;
//          p++;
//          continue;
//        }
//        else {
//          capture[captop].siz = 1;  /* mark entry as closed */
//          capture[captop].s = s;
//          goto pushcapture;
//        }
//      }
//      case IOpenCapture:
//        capture[captop].siz = 0;  /* mark entry as open */
//        capture[captop].s = s;
//        goto pushcapture;
//      case IFullCapture:
//        capture[captop].siz = getoff(p) + 1;  /* save capture size */
//        capture[captop].s = s - getoff(p);
//        /* goto pushcapture; */
//      pushcapture: {
//        capture[captop].idx = p->i.key;
//        capture[captop].kind = getkind(p);
//        if (++captop >= capsize) {
//          capture = doublecap(L, capture, captop, ptop);
//          capsize = 2 * captop;
//        }
//        p++;
//        continue;
//      }
      _ => { assert!("bad dood" == "true"); }
    }
  }
}

/* }====================================================== */


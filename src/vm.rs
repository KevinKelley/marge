
use code::*; // didn't feel like listing them

//use capture::Capture;

struct CapLevel(pub uint);
struct StackIdx(pub uint);


enum StackEntry {
  ReturnTo(Option<CodeIdx>),
  AlternateTo(CodeIdx, CharNum, CapLevel)
}
struct State {
  // current instruction index, or FAIL (None)
  p: Option<CodeIdx>,
  // current subject position (char index in subject string)
  // (NOT byte index; this represents number of unicode chars matched so far)
  i: CharNum,
  // stack-entry: either (uint) return-pos, or (next-pos, subject-pos, cap-list)
  // this StackIdx is actually the index of first available slot on stack;
  // that is, the number of entries in the stack.
  e: StackIdx,
  // track the current capture-count; when backtracking, pop the capture stack
  // back to a recorded level.  Or else I'm completely reading it wrong...
  c: CapLevel
}

struct VmState(
  // current instruction index, or FAIL (None)
  Option<CodeIdx>,
  // current subject position (char index in subject string) (NOT byte index)
  CharNum,
  // stack-entry: either (uint) return-pos, or (next-pos, subject-pos, cap-list).
  // this index is redundant if we're using a growable stack vec:
  //   StackIdx should always == TOS (stack.len()-1); and
  //   stack should never be empty when machine is running:
  //   (bottom of stack should be an entry with ReturnTo(None)).
  //   (this comment may not actually be true)
  StackIdx,
  // Capture pointer? -- captures are a tuple of (CodeIdx,CharNum); and
  // 'captures' is a Vec<Capture>; so CapLevel should really be CapCount and
  // should always equal captures.len().  I think.  There's some weird stuff
  // in the Lua implementation about "dynamic" captures, and also some stuff
  // about keeping the capture list in the LuaState so it's accessible to Lua code.
  // But I'm not sure that's what "dynamic captures" means, I think that's more
  // about speculative capturing in context of possible backtracking... would
  // mean need to possible "uncapture" when backtracking past a capture.
  // I don't quite get it yet.
  CapLevel
);

/// Vm maintains the environment, and operates the virtual machine.
/// Abstractly the machine is a pure state-machine operating on
/// 4 registers:
///   p: the next instruction (opcode) to execute.  Program counter.
///   i: current character in the input stream.
///   e: the call-stack; the 'entry' at TOS indicates where to return to
///   c: capture-stack, tracking positions where rules match.
/// Operational semantics of the VM is as in tables below: given 4 registers
/// and a current opcode, return the updated register values.
struct Vm {
  program: Vec<Opcode>,
  text: Vec<char>,
  stack: Vec<StackEntry>,
  captures: Vec<Capture>
}
#[allow(unused_mut)]
impl Vm {
  fn new(program: Vec<Opcode>) -> Vm {
    Vm {
      program: program,
      text: vec!(),
      stack: vec!(),
      captures: vec!()
    }
  }

  //Figure 2. basic instructions for the parsing machine:
  //
  //  p,i,e,c       Char x,S[i] = x   ⇒ p+1,i+1,e,c
  //  p,i,e,c       Char x,S[i] != x  ⇒ Fail,i,e,c
  //  p,i,e,c       Jump o            ⇒ p+o,i,e,c
  //  p,i,e,c       Choice o          ⇒ p+1,i,(p+o,i,c):e,c
  //  p,i,e,ci      Call o            ⇒ p+o,i,(p+1):e,c
  //  p0,i,p1:e c   Return            ⇒ p1,i,e,c
  //  p,i,h:e,c     Commit o          ⇒ p+o,i,e,c
  //  p,i,e,c       Capture k         ⇒ p+1,i,e,(i,p):c
  //  p,i,e,c       Fail              ⇒ Fail,i,e,c
  //  Fail,i,p:e,c  any               ⇒ Fail,i,e,c
  //  Fail,i0,(p,i1,c1):e,c0  any     ⇒ p,i1,e,c1
  //
  // Note: p:e means take the value in that register to be a stack,
  // expect at least one value on the stack; assign TOS to p and
  // remainder to e.  (i,p):c is similar, but value on top of stack
  // would need to be a tuple of 2 elements.


  // NOTE on operation of lpeg vm:
  // seems that the where I'm using an enum to encode the three alternate
  // meanings of the p register:
  //   None = Fail,
  //   ReturnTo(pc) = return from a ICall,
  //   AlternateTo(pc,i,c) = return from a choice and take the alternate,
  //     using the stored 'i' == charpos and 'c' == numcaptures.
  //
  // The lpeg C implementation can't do that; so there, the three conditions are:
  // Fail: p == NULL
  //   keep popping stack. i and c ignored (but use i, aka S, for failure location?),
  // Return from a ICall: i == NULL
  //   p is stored pc, i (aka S or s) == NULL, use current captures;
  // Return from IChoice and take alternate: otherwise.
  //   p is stored pc, i is stored S, c is stored capturecount.


  fn step(&mut self, VmState(p,i,e,c): VmState) -> VmState {

    match (p,i,e,c) {

      //(None, _, ReturnTo(CodeIdx(pc)), _) => VmState(Fail, i, e,c),
      //(None, _, AlternateTo(CodeIdx(pc), CharNum(i1), CapLevel(c1)), _) => VmState(p, i1, e,c),
      (None,_,StackIdx(sp),_) if sp > 0 => { // if sp < 1, stack was empty and we're hosed
        let tos = self.stack.pop();
        let sp = sp - 1;
        assert!(sp == self.stack.len() && sp > 0);  // can't run with an empty stack,
                                                    // or machine won't know where to return
        match tos {
          Some(ReturnTo(_))
            => return VmState(None, i, StackIdx(sp), c),
          Some(AlternateTo(CodeIdx(pc), CharNum(i1), CapLevel(c1)))
            => return VmState(p, CharNum(i1), StackIdx(sp), CapLevel(c1)),
          None
            => unreachable!() //fail!("popped an invalid entry from vm stack!")
        }
      }
      (None,_,_,_) /*sp == 0*/ => fail!("vm stack shouldn't have been empty!"),

      (Some(CodeIdx(pc)), CharNum(ip), StackIdx(sp), CapLevel(cap)) => {
        let op = self.program[pc];
        match op {

          //  p,i,e,c       Char x,S[i] = x   ⇒ p+1,i+1,e,c
          IChar(ch, flags) if ch == self.text[ip] => {
            print!("IChar {} ...\n", ch);
            return VmState(Some(CodeIdx(pc+1)),CharNum(ip+1),e,c)
          }
          //  p,i,e,c       Char x,S[i] != x  ⇒ Fail,i,e,c
          IChar(ch, flags) if ch != self.text[ip] => {
            return VmState(None,i,e,c)
          }
          //  p,i,e,c       Jump l            ⇒ p+l,i,e,c
          IJmp(offset) => {
            let dest = (pc as int + offset) as uint;
            assert!(dest < self.program.len());
            return VmState(Some(CodeIdx(dest)),i,e,c)
          }
          //  p,i,e,c       Choice l          ⇒ p+1,i,(p+l,i,c):e,c
          IChoice(offset) => {
            let dest2 = (pc as int + offset) as uint;
            let e2 = AlternateTo(CodeIdx(dest2), i, c);
            self.stack.push(e2);
            let sp = sp + 1;
            assert!(sp == self.stack.len());
            assert!(dest2 < self.program.len());
            return VmState(Some(CodeIdx(pc+1)),i,StackIdx(sp),c)
          }
          //  p,i,e,ci      Call l            ⇒ p+l,i,(p+1):e,c
          ICall(offset) => {
            let e2 = ReturnTo(Some(CodeIdx((pc as int + offset) as uint)));
            self.stack.push(e2);
            let sp = sp + 1;
            assert!(sp == self.stack.len());
            return VmState(Some(CodeIdx(pc+1)),i,StackIdx(sp),c)
          }
          //  p0,i,p1:e c   Return            ⇒ p1,i,e,c
          IRet => {
            let tos = self.stack.pop();
            let sp = sp - 1;
            assert!(sp == self.stack.len() && sp > 0);
            match tos {
              Some(ReturnTo(dest))
                => return VmState(dest, i, StackIdx(sp), c),
              _ => unreachable!() //fail!("popped an invalid entry from vm stack!")
            }
          }
          //  p,i,h:e,c     Commit l          ⇒ p+l,i,e,c
          ICommit(offset) => {
            let dest = (pc as int + offset) as uint;
            let _tos = self.stack.pop();
            let sp = sp - 1;
            assert!(sp == self.stack.len() && sp > 0); // can't run with an empty stack
            return VmState(Some(CodeIdx(dest)), i, StackIdx(sp), c)
          }
          //  p,i,e,c       Capture k         ⇒ p+1,i,e,(i,p):c
          IFullCapture(k) => {
            self.captures.push(Capture(CharNum(ip),CodeIdx(pc)));
            let cap = cap + 1;
            assert!(cap == self.captures.len());
            return VmState(Some(CodeIdx(pc+1)), i, e, CapLevel(cap))
          }
          IAny(count) => {
            let next_ip = ip + count as uint;
            if next_ip <= self.text.len() {
              return VmState(Some(CodeIdx(pc+1)), CharNum(next_ip), e,c)
            }
            else {
              // goto fail
              //return VmState(None, i,e,c)
            }
          }
          IEnd => {
            // push capture?  --I don't think it's a capture unless
            // you explicitly capture it.  Normal execution will
            // have the char-pos pointing to where we are now in the
            // input, so can verify that it's all been processed.

            // IEnd should be the last instruction in the program;
            // to execute it just walk on.  outer runner will see
            // that we've got past the program without failing,
            // and will therefore extract any matches or whatever.
            // Maybe by, if there are captures, return them; if not,
            // return the current-char-num to indicate where in the
            // input that the parse completed.  Presumably this would
            // normally be enforced to match the input-length; but
            // outer caller may choose different semantics if appropriate.
            return VmState(Some(CodeIdx(pc+1)),i,e,c)
          }
          IFail => {
            // FIXME: fail needs to pop stack and clear relevant captures
            //return VmState(None,i,e,c)
          }
          _ => {
            return VmState(p,i,e,c)
          }
        }
      }
    }

    // fall out of match to fail

    //case IFail:
    /* pattern failed: try to backtrack */
    //do {  /* remove pending calls */
    //  assert(stack > getstackbase(L, ptop));
    //  s = (--stack)->s;
    //} while (s == NULL);
    //if (ndyncap > 0)  /* is there matchtime captures? */
    //  ndyncap -= removedyncap(L, capture, stack->caplevel, captop);
    //captop = stack->caplevel;
    //p = stack->p;
    //continue;
    let sp = match e {
      StackIdx(sp) => sp
    };
    'fail: loop {
      assert!(sp == self.stack.len() && sp > 0);
      let (tos, sp) = (self.stack.pop(), sp-1);
      match tos {
        Some(AlternateTo(dest, CharNum(cn1), CapLevel(cl1)))
          => {
            // TODO: what's all that dyncap stuff?  deal with it.
            return VmState(Some(dest), CharNum(cn1), StackIdx(sp), CapLevel(cl1))
          }
        _ => { continue 'fail; }
      };
    }
  }

  /// match a string input, and return number of characters (not bytes) matched.
  /// should this be non-self method that creates an internal private Vm to run?
  fn do_match(&mut self, input: &str) -> Option<CharNum> {

    self.text = input.chars().collect();
    self.stack.clear();
    self.captures.clear();

    self.stack.push(ReturnTo(None));

    // initial state for the parsing-machine
    let mut state = VmState(
      Some(CodeIdx(0)), // start at the beginning of code
      CharNum(0),       // and beginning of source
      StackIdx(self.stack.len()),
      CapLevel(self.captures.len())
    );

    'vm: loop {
      state = self.step(state);
      match state {
        // a program has only one "End" instruction, its last;
        // nested grammars can compose, inner "Return"ing to outer
        // when successful;
        // a parse will either fail (eventually leaving Fail (None) in 'p'),
        // or succeed, in which case the program counter will point past the
        // "End" instruction.
        VmState(Some(CodeIdx(pc)),CharNum(i),_,_) if pc == self.program.len() => {
          return Some(CharNum(i));
      }
        VmState(None,_,_,_) => { break 'vm; }
        _ => {}
      }
    }
    None
  }
}

#[test]
fn t1() {
  let code = vec!(IChar('a'),IChar('n'),IChar('a'), IEnd);
  let mut vm = Vm::new(code);
  let result = vm.do_match("ana");
  assert!(result.unwrap() == CharNum(3));
}


// stuff from lpeg below

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


// //struct Stack {
// //  const char *s;  /* saved position (or NULL for calls) */
// //  const Instruction *p;  /* next instruction */
// //  int caplevel;
// //} Stack;
// struct Stack<'a> {
//   s: uint,   /* saved position (or NULL for calls) */
//   p: &'a Instruction,  /* next instruction */
//   caplevel: i32
// }



/*
** Opcode interpreter
*/
//const char *match (lua_State *L, const char *o, const char *s, const char *e,
//                   Instruction *op, Capture *capture, int ptop)
unsafe fn do_match_slightly_hacked_lua_example (
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

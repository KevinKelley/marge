
use capture::Capture;

/* Virtual Machine's instructions */
enum Opcode {
  IAny,             // if no char, fail
  IChar,            // if char != aux, fail
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
	o: *const char,
	s: *const char,
	e: *const char,
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


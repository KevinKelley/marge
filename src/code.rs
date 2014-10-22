
use ast::Flags;
//use capture::{Capture};

// Virtual Machine's instructions
//
// Note, 'offset' is always relative to that instruction.
//
#[deriving(Eq,PartialEq,Show,Clone)]
pub enum Opcode {
  IAny(Flags),             // if no char, fail
  IChar(char, Flags),      // if char != aux, fail
  //ISet,             // if char not in buff, fail
  //ITestAny,         // in no char, jump to 'offset'
  //ITestChar,        // if char != aux, jump to 'offset'
  //ITestSet,         // if char not in buff, jump to 'offset'
  //ISpan,            // read a span of chars in buff
  //IBehind,          // walk back 'aux' characters (fail if not possible)
  IRet,             // return from a rule
  IEnd,             // end of pattern
  IChoice(int),     // stack a choice; next fail will jump to 'offset'
  IJmp(int),        // jump to 'offset'
  ICall(int),       // call rule at 'offset'
  //IOpenCall,        // call rule number 'key' (must be closed to a ICall)
  ICommit(int),     // pop choice and jump to 'offset'
  //IPartialCommit,   // update top choice to current position and jump
  //IBackCommit,      // "fails" but jump to its own 'offset'
  //IFailTwice,       // pop one choice and then fail
  IFail,            // go back to saved state on choice and jump to saved offset
  //IGiveup,          // internal use
  IFullCapture(int),// complete capture of last 'off' chars
  //IOpenCapture,     // start a capture
  //ICloseCapture,
  //ICloseRunTime
}

#[deriving(Eq,PartialEq,Show,Clone)]
pub struct CodeIdx(pub uint);

#[deriving(Eq,PartialEq,Show,Clone)]
pub struct CharNum(pub uint);

#[deriving(Eq,PartialEq,Show,Clone)]
pub struct Capture(pub CharNum, pub CodeIdx);

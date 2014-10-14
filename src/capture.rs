
/* kinds of captures */
pub enum CapKind {
  Cclose, Cposition, Cconst, Cbackref, Carg, Csimple, Ctable, Cfunction,
  Cquery, Cstring, Cnum, Csubst, Cfold, Cruntime, Cgroup
}


pub struct Capture {
  s: *const char,  /* subject position */
  idx: u16,  /* extra info about capture (group name, arg index, etc.) */
  kind: CapKind,  /* kind of capture */
  siz: u8,  /* size of full capture + 1 (0 = not a full capture) */
}


pub struct CapState {
  cap: *const Capture,  /* current capture */
  ocap: *const Capture,  /* (original) capture list */
  //lua_State *L;
  ptop: i32,  /* index of last argument to 'match' */
  s: *const char,  /* original string */
  valuecached: int  /* value stored in cache slot */
}


//int runtimecap (CapState *cs, Capture *close, const char *s, int *rem);
//int getcaptures (lua_State *L, const char *s, const char *r, int ptop);
//int finddyncap (Capture *cap, Capture *last);

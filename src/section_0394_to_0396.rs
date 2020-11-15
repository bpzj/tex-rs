//! @ A slightly subtle point arises here: When the parameter delimiter ends
//! with `\.{\#\{}', the token list will have a left brace both before and
//! after the |end_match|\kern-.4pt. Only one of these should affect the
//! |align_state|, but both will be scanned, so we must make a correction.
//!
//! @<Advance \(r)|r|; |goto found| if the parameter delimiter has been fully...@>=
//! begin r:=link(r);
//! if (info(r)>=match_token)and(info(r)<=end_match_token) then
//!   begin if cur_tok<left_brace_limit then decr(align_state);
//!   goto found;
//!   end
//! else goto continue;
//! end
//!
//! @ @<Report an extra right brace and |goto continue|@>=
//! begin back_input; print_err("Argument of "); sprint_cs(warning_index);
//! @.Argument of \\x has...@>
//! print(" has an extra }");
//! help6("I've run across a `}' that doesn't seem to match anything.")@/
//!   ("For example, `\def\a#1{...}' and `\a}' would produce")@/
//!   ("this error. If you simply proceed now, the `\par' that")@/
//!   ("I've just inserted will cause me to report a runaway")@/
//!   ("argument that might be the root of the problem. But if")@/
//!   ("your `}' was spurious, just type `2' and it will go away.");
//! incr(align_state); long_state:=call; cur_tok:=par_token; ins_error;
//! goto continue;
//! end {a white lie; the \.{\\par} won't always trigger a runaway}
//!
//! @ If |long_state=outer_call|, a runaway argument has already been reported.
//!
//! @<Report a runaway argument and abort@>=
//! begin if long_state=call then
//!   begin runaway; print_err("Paragraph ended before ");
//! @.Paragraph ended before...@>
//!   sprint_cs(warning_index); print(" was complete");
//!   help3("I suspect you've forgotten a `}', causing me to apply this")@/
//!     ("control sequence to too much text. How can we recover?")@/
//!     ("My plan is to forget the whole thing and hope for the best.");
//!   back_error;
//!   end;
//! pstack[n]:=link(temp_head); align_state:=align_state-unbalance;
//! for m:=0 to n do flush_list(pstack[m]);
//! return;
//! end
//!

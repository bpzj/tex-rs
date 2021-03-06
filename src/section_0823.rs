//! @ As the algorithm runs, it maintains a set of six delta-like registers
//! for the length of the line following the first active breakpoint to the
//! current position in the given hlist. When it makes a pass through the
//! active list, it also maintains a similar set of six registers for the
//! length following the active breakpoint of current interest. A third set
//! holds the length of an empty line (namely, the sum of \.{\\leftskip} and
//! \.{\\rightskip}); and a fourth set is used to create new delta nodes.
//!
//! When we pass a delta node we want to do operations like
//! $$\hbox{\ignorespaces|for
//! k:=1 to 6 do cur_active_width[k]:=cur_active_width[k]+mem[q+k].sc|};$$ and we
//! want to do this without the overhead of |for| loops. The |do_all_six|
//! macro makes such six-tuples convenient.
//!
//! @d do_all_six(#)==#(1);#(2);#(3);#(4);#(5);#(6)
//!
//! @<Glob...@>=
//! @!active_width:array[1..6] of scaled;
//!   {distance from first active node to~|cur_p|}
//! @!cur_active_width:array[1..6] of scaled; {distance from current active node}
//! @!background:array[1..6] of scaled; {length of an ``empty'' line}
//! @!break_width:array[1..6] of scaled; {length being computed after current break}
//!

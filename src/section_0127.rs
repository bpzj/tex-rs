//! @ Empirical tests show that the routine in this section performs a
//! node-merging operation about 0.75 times per allocation, on the average,
//! after which it finds that |r>p+1| about 95\pct! of the time.
//
// @<Try to allocate...@>=
macro_rules! Try_to_allocate_within_node_p_and_its_physical_successors_and_goto_found_if_allocation_was_possible {
    ($globals:expr, $p:expr, $q:expr, $r:expr, $s:expr, $t:expr, $lbl_found:lifetime) => {{
        // q:=p+node_size(p); {find the physical successor}
        /// find the physical successor
        {
            $q = $p + node_size!($globals, $p);
        }
        // @^inner loop@>
        // while is_empty(q) do {merge node |p| with node |q|}
        /// merge node `p` with node `q`
        while is_empty!($globals, $q) {
            // begin t:=rlink(q);
            // if q=rover then rover:=t;
            // llink(t):=llink(q); rlink(llink(q)):=t;@/
            // q:=q+node_size(q);
            // end;
            todo!("merge");
        }
        // r:=q-s;
        $r = $q as integer - $s;
        // if r>p+1 then @<Allocate from the top of node |p| and |goto found|@>;
        if $r > ($p + 1) as integer {
            Allocate_from_the_top_of_node_p_and_goto_found!($globals, $p, $r, $lbl_found);
        }
        // if r=p then if rlink(p)<>p then
        //   @<Allocate entire node |p| and |goto found|@>;
        if $r == $p as integer && rlink!($globals, $p) != $p {
            todo!();
        }
        // node_size(p):=q-p {reset the size in case it grew}
        node_size!($globals, $p) = $q - $p;

        use crate::section_0124::empty_flag;
    }}
}

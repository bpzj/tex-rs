//! @ We've already defined the special variable |loc==cur_input.loc_field|
//! in our discussion of basic input-output routines. The other components of
//! |cur_input| are defined in the same way:

// @d state==cur_input.state_field {current scanner state}
/// current scanner state
macro_rules! state {
    ($globals:expr) => {
        $globals.cur_input.state_field
    };
}

// @d index==cur_input.index_field {reference for buffer information}
/// reference for buffer information
#[allow(unused_macros)]
macro_rules! index {
    ($globals:expr) => {
        $globals.cur_input.index_field
    };
}
// @d start==cur_input.start_field {starting position in |buffer|}
// @d limit==cur_input.limit_field {end of current line in |buffer|}
/// end of current line in `buffer`
macro_rules! limit {
    ($globals:expr) => {
        $globals.cur_input.limit_field
    };
}
// @d name==cur_input.name_field {name of the current file}
macro_rules! name {
    ($globals:expr) => {
        $globals.cur_input.name_field
    };
}
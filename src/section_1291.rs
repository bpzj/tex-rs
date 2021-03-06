//! ` `

// @d show_code=0 { \.{\\show} }
// @d show_box_code=1 { \.{\\showbox} }
// @d show_the_code=2 { \.{\\showthe} }
// @d show_lists=3 { \.{\\showlists} }
pub(crate) enum show_kind {
    /// `\show`
    show_code = 0,
    /// `\showbox`
    show_box_code = 1,
    /// `\showthe`
    show_the_code = 2,
    /// `\showlists`
    show_lists = 3,
}

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1291(globals: &mut TeXGlobals) {
    // primitive("show",xray,show_code);
    primitive(globals, strpool_str!("show"), xray, show_kind::show_code as _);
    // @!@:show_}{\.{\\show} primitive@>
    // primitive("showbox",xray,show_box_code);
    primitive(globals, strpool_str!("showbox"), xray, show_kind::show_box_code as _);
    // @!@:show_box_}{\.{\\showbox} primitive@>
    // primitive("showthe",xray,show_the_code);
    primitive(globals, strpool_str!("showthe"), xray, show_kind::show_the_code as _);
    // @!@:show_the_}{\.{\\showthe} primitive@>
    // primitive("showlists",xray,show_lists);
    primitive(globals, strpool_str!("showlists"), xray, show_kind::show_lists as _);
    // @!@:show_lists_}{\.{\\showlists} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::xray;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}

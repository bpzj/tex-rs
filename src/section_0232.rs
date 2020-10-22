//! @ We initialize most things to null or undefined values. An undefined font
//! is represented by the internal code |font_base|.
//!
//! However, the character code tables are given initial values based on the
//! conventional interpretation of ASCII code. These initial values should
//! not be changed when \TeX\ is adapted for use with non-English languages;
//! all changes to the initialization conventions should be made in format
//! packages, not in \TeX\ itself, so that global interchange of formats is
//! possible.
//
// @d null_font==font_base
// @d var_code==@'70000 {math code meaning ``use the current family''}
//
// @<Initialize table entries...@>=
#[distributed_slice(INIT_TBLENTRY)]
#[allow(unused_variables)]
pub(crate) fn initialize_table_entries_done_by_initex_only_0232(globals: &mut TeXGlobals) {
    // par_shape_ptr:=null; eq_type(par_shape_loc):=shape_ref;
    // eq_level(par_shape_loc):=level_one;@/
    // for k:=output_routine_loc to toks_base+255 do
    //   eqtb[k]:=eqtb[undefined_control_sequence];
    // box(0):=null; eq_type(box_base):=box_ref; eq_level(box_base):=level_one;
    // for k:=box_base+1 to box_base+255 do eqtb[k]:=eqtb[box_base];
    // cur_font:=null_font; eq_type(cur_font_loc):=data;
    // eq_level(cur_font_loc):=level_one;@/
    // for k:=math_font_base to math_font_base+47 do eqtb[k]:=eqtb[cur_font_loc];
    // equiv(cat_code_base):=0; eq_type(cat_code_base):=data;
    // eq_level(cat_code_base):=level_one;@/
    // for k:=cat_code_base+1 to int_base-1 do eqtb[k]:=eqtb[cat_code_base];
    // for k:=0 to 255 do
    for k_u8 in 0..=255u8 {
        let k = ASCII_code_literal!(k_u8);
        // begin cat_code(k):=other_char; math_code(k):=hi(k); sf_code(k):=1000;
        cat_code!(globals, k) = other_char as _;
        math_code!(globals, k) = hi!(k_u8 as halfword) as _;
        sf_code!(globals, k) = 1000;
        // end;
    }
    // cat_code(carriage_return):=car_ret; cat_code(" "):=spacer;
    // cat_code("\"):=escape; cat_code("%"):=comment;
    // cat_code(invalid_code):=invalid_char; cat_code(null_code):=ignore;
    // for k:="0" to "9" do math_code(k):=hi(k+var_code);
    // for k:="A" to "Z" do
    //   begin cat_code(k):=letter; cat_code(k+"a"-"A"):=letter;@/
    //   math_code(k):=hi(k+var_code+@"100);
    //   math_code(k+"a"-"A"):=hi(k+"a"-"A"+var_code+@"100);@/
    //   lc_code(k):=k+"a"-"A"; lc_code(k+"a"-"A"):=k+"a"-"A";@/
    //   uc_code(k):=k; uc_code(k+"a"-"A"):=k;@/
    //   sf_code(k):=999;
    //   end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use crate::section_0018::ASCII_code;
use crate::section_0113::halfword;
use crate::section_0207::other_char;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}

//! @ Region 3 of |eqtb| contains the 256 \.{\\skip} registers, as well as the
//! glue parameters defined here. It is important that the ``muskip''
//! parameters have larger numbers than the others.
//
// @d line_skip_code=0 {interline glue if |baseline_skip| is infeasible}
// @d baseline_skip_code=1 {desired glue between baselines}
// @d par_skip_code=2 {extra glue just above a paragraph}
// @d above_display_skip_code=3 {extra glue just above displayed math}
// @d below_display_skip_code=4 {extra glue just below displayed math}
// @d above_display_short_skip_code=5
//   {glue above displayed math following short lines}
// @d below_display_short_skip_code=6
//   {glue below displayed math following short lines}
// @d left_skip_code=7 {glue at left of justified lines}
// @d right_skip_code=8 {glue at right of justified lines}
// @d top_skip_code=9 {glue at top of main pages}
// @d split_top_skip_code=10 {glue at top of split pages}
// @d tab_skip_code=11 {glue between aligned entries}
// @d space_skip_code=12 {glue between words (if not |zero_glue|)}
/// glue between words (if not `zero_glue`)
pub(crate) const space_skip_code: halfword = 12;
// @d xspace_skip_code=13 {glue after sentences (if not |zero_glue|)}
// @d par_fill_skip_code=14 {glue on last line of paragraph}
// @d thin_mu_skip_code=15 {thin space in math formula}
// @d med_mu_skip_code=16 {medium space in math formula}
// @d thick_mu_skip_code=17 {thick space in math formula}
// @d glue_pars=18 {total number of glue parameters}
/// total number of glue parameters
pub(crate) type glue_pars_TYPENUM = U18;
pub(crate) const glue_pars: halfword = glue_pars_TYPENUM::U16;
// @d skip_base=glue_base+glue_pars {table of 256 ``skip'' registers}
/// table of 256 ``skip'' registers
pub(crate) type skip_base_TYPENUM = typenum::op!(glue_base_TYPENUM + glue_pars_TYPENUM);
pub(crate) const skip_base: word = skip_base_TYPENUM::U32;
// @d mu_skip_base=skip_base+256 {table of 256 ``muskip'' registers}
/// table of 256 "muskip" registers
pub(crate) type mu_skip_base_TYPENUM = typenum::op!(skip_base_TYPENUM + U256);
pub(crate) const mu_skip_base: word = mu_skip_base_TYPENUM::U32;
// @d local_base=mu_skip_base+256 {beginning of region 4}
/// beginning of region 4
pub(crate) type local_base_TYPENUM = typenum::op!(mu_skip_base_TYPENUM + U256);
pub(crate) const local_base: word = local_base_TYPENUM::U32;
// @#
// @d skip(#)==equiv(skip_base+#) {|mem| location of glue specification}
/// `mem` location of glue specification
macro_rules! skip {
    ($globals:expr, $ptr:expr) => {
        equiv!($globals, crate::section_0224::skip_base as crate::pascal::word +
            $ptr as crate::pascal::word)
    }
}
// @d mu_skip(#)==equiv(mu_skip_base+#) {|mem| location of math glue spec}
/// `mem` location of math glue spec
macro_rules! mu_skip {
    ($globals:expr, $ptr:expr) => {
        equiv!($globals, crate::section_0224::mu_skip_base as crate::pascal::word +
            $ptr as crate::pascal::word)
    }
}
// @d glue_par(#)==equiv(glue_base+#) {|mem| location of glue specification}
/// `mem` location of glue specification
macro_rules! glue_par {
    ($globals:expr, $ptr:expr) => {
        equiv!($globals, crate::section_0222::glue_base as crate::pascal::word +
            $ptr as crate::pascal::word)
    }
}
// @d line_skip==glue_par(line_skip_code)
// @d baseline_skip==glue_par(baseline_skip_code)
// @d par_skip==glue_par(par_skip_code)
// @d above_display_skip==glue_par(above_display_skip_code)
// @d below_display_skip==glue_par(below_display_skip_code)
// @d above_display_short_skip==glue_par(above_display_short_skip_code)
// @d below_display_short_skip==glue_par(below_display_short_skip_code)
// @d left_skip==glue_par(left_skip_code)
// @d right_skip==glue_par(right_skip_code)
// @d top_skip==glue_par(top_skip_code)
// @d split_top_skip==glue_par(split_top_skip_code)
// @d tab_skip==glue_par(tab_skip_code)
// @d space_skip==glue_par(space_skip_code)
macro_rules! space_skip {
    ($globals:expr) => {
        glue_par!($globals, crate::section_0224::space_skip_code)
    }
}
// @d xspace_skip==glue_par(xspace_skip_code)
// @d par_fill_skip==glue_par(par_fill_skip_code)
// @d thin_mu_skip==glue_par(thin_mu_skip_code)
// @d med_mu_skip==glue_par(med_mu_skip_code)
// @d thick_mu_skip==glue_par(thick_mu_skip_code)
//
// @<Current |mem| equivalent of glue parameter number |n|@>=glue_par(n)
//

use crate::pascal::word;
use crate::section_0113::halfword;
use crate::section_0222::glue_base_TYPENUM;
use typenum::Unsigned;
use typenum::{U18, U256};

//! @ When \TeX\ ``packages'' a list into a box, it needs to calculate the
//! proportionality ratio by which the glue inside the box should stretch
//! or shrink. This calculation does not affect \TeX's decision making,
//! so the precise details of rounding, etc., in the glue calculation are not
//! of critical importance for the consistency of results on different computers.
//!
//! We shall use the type |glue_ratio| for such proportionality ratios.
//! A glue ratio should take the same amount of memory as an
//! |integer| (usually 32 bits) if it is to blend smoothly with \TeX's
//! other data structures. Thus |glue_ratio| should be equivalent to
//! |short_real| in some implementations of \PASCAL. Alternatively,
//! it is possible to deal with glue ratios using nothing but fixed-point
//! arithmetic; see {\sl TUGboat \bf3},1 (March 1982), 10--27. (But the
//! routines cited there must be modified to allow negative glue ratios.)
//! @^system dependencies@>
//
// @d set_glue_ratio_zero(#) == #:=0.0 {store the representation of zero ratio}
// @d set_glue_ratio_one(#) == #:=1.0 {store the representation of unit ratio}
// @d float(#) == # {convert from |glue_ratio| to type |real|}
// @d unfloat(#) == # {convert from |real| to type |glue_ratio|}
// @d float_constant(#) == #.0 {convert |integer| constant to |real|}
//
// @<Types...@>=
// @!glue_ratio=real; {one-word representation of a glue expansion factor}

pub type glue_ratio = real;

use crate::pascal::real;

//! @ The third.
//! @^system dependencies@>
//
// @p procedure end_name;
#[allow(unused_variables)]
pub(crate) fn end_name(globals: &mut TeXGlobals) {
    // begin if str_ptr+3>max_strings then
    //   overflow("number of strings",max_strings-init_str_ptr);
    // @:TeX capacity exceeded number of strings}{\quad number of strings@>
    // if area_delimiter=0 then cur_area:=""
    if globals.area_delimiter.is_zero() {
        globals.cur_area = strpool_str!("");
    }
    // else  begin cur_area:=str_ptr;
    else {
        todo!();
        // str_start[str_ptr+1]:=str_start[str_ptr]+area_delimiter; incr(str_ptr);
        // end;
    }
    // if ext_delimiter=0 then
    if globals.ext_delimiter.is_zero() {
        // begin cur_ext:=""; cur_name:=make_string;
        globals.cur_ext = strpool_str!("");
        globals.cur_name = make_string(make_globals_string_view!(globals));
        // end
    }
    // else  begin cur_name:=str_ptr;
    else {
        todo!();
        // str_start[str_ptr+1]:=str_start[str_ptr]+ext_delimiter-area_delimiter-1;
        // incr(str_ptr); cur_ext:=make_string;
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobalsStringView;
use crate::section_0043::make_string;
use crate::section_0004::TeXGlobals;
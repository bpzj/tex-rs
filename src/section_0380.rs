//! @ Here is a recursive procedure that is \TeX's usual way to get the
//! next token of input. It has been slightly optimized to take account of
//! common cases.

// @p procedure get_x_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|,
//   and expands macros}
/// sets `cur_cmd`, `cur_chr`, `cur_tok`, and expands macros
pub(crate) fn get_x_token(globals: &mut TeXGlobals) {
    // label restart,done;
    // begin restart: get_next;
    region_forward_label! {
    |'done|
    {
    region_backward_label! {
    'restart <-
    {
        get_next(globals);
        // @^inner loop@>
        // if cur_cmd<=max_command then goto done;
        if globals.cur_cmd <= max_command {
            goto_forward_label!('done);
        }
        // if cur_cmd>=call then
        //   if cur_cmd<end_template then macro_call
        //   else  begin cur_cs:=frozen_endv; cur_cmd:=endv;
        //     goto done; {|cur_chr=null_list|}
        //     end
        // else expand;
        // goto restart;
        goto_backward_label!('restart);
    }
    |'restart|
    }
    }
    // done: if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
    // else cur_tok:=cs_token_flag+cur_cs;
    // end;
    'done <-
    }
    if globals.cur_cs == 0 {
        globals.cur_tok = (globals.cur_cmd as halfword * 0o400) + globals.cur_chr
    } else {
        globals.cur_tok = cs_token_flag + globals.cur_cs;
    }
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0209::max_command;
use crate::section_0289::cs_token_flag;
use crate::section_0341::get_next;
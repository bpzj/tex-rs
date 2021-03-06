//! ` `
//! @<Expand a nonmacro@>=
macro_rules! Expand_a_nonmacro {
    ($globals:expr) => {{
        // begin if tracing_commands>1 then show_cur_cmd_chr;
        if tracing_commands!($globals) > 1 {
            show_cur_cmd_chr($globals);
        }
        // case cur_cmd of
        if false {
            do_nothing!();
        }
        // top_bot_mark:@<Insert the \(a)appropriate mark text into the scanner@>;
        // expand_after:@<Expand the token after the next token@>;
        // no_expand:@<Suppress expansion of the next token@>;
        // cs_name:@<Manufacture a control sequence name@>;
        // convert:conv_toks; {this procedure is discussed in Part 27 below}
        // the:ins_the_toks; {this procedure is discussed in Part 27 below}
        // if_test:conditional; {this procedure is discussed in Part 28 below}
        else if $globals.cur_cmd == if_test {
            /// this procedure is discussed in Part 28 below
            conditional($globals)?;
        }
        // fi_or_else:@<Terminate the current conditional and skip to \.{\\fi}@>;
        else if $globals.cur_cmd == fi_or_else {
            Terminate_the_current_conditional_and_skip_to_fi!($globals);
        }
        // input:@<Initiate or terminate input from a file@>;
        // othercases @<Complain about an undefined macro@>
        else {
            trace_error_expr!("cur_cmd = {}", $globals.cur_cmd);
            Complain_about_an_undefined_macro!($globals);
        }
        // endcases;
        // end
        use crate::section_0210::if_test;
        use crate::section_0210::fi_or_else;
        use crate::section_0299::show_cur_cmd_chr;
        use crate::section_0498::conditional;
    }}
}
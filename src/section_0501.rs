//! ` `

// @<Either process \.{\\ifcase} or set |b|...@>=
macro_rules! Either_process_ifcase_or_set_b_to_the_value_of_a_boolean_condition {
    ($globals:expr, $this_if:expr, $b:expr) => {{
        // case this_if of
        if false {
            unreachable!();
        }
        // if_char_code, if_cat_code: @<Test if two characters match@>;
        // if_int_code, if_dim_code: @<Test relation between integers or dimensions@>;
        else if $this_if == if_int_code || $this_if == if_dim_code {
            Test_relation_between_integers_or_dimensions!($globals, $this_if, $b);
        }
        // if_odd_code: @<Test if an integer is odd@>;
        // if_vmode_code: b:=(abs(mode)=vmode);
        // if_hmode_code: b:=(abs(mode)=hmode);
        // if_mmode_code: b:=(abs(mode)=mmode);
        // if_inner_code: b:=(mode<0);
        // if_void_code, if_hbox_code, if_vbox_code: @<Test box register status@>;
        // ifx_code: @<Test if two tokens match@>;
        // if_eof_code: begin scan_four_bit_int; b:=(read_open[cur_val]=closed);
        //   end;
        // if_true_code: b:=true;
        else if $this_if == if_true_code {
            $b = true;
        }
        // if_false_code: b:=false;
        // if_case_code: @<Select the appropriate case
        //   and |return| or |goto common_ending|@>;
        // end {there are no other cases}
        else {
            trace_error_expr!("this_if = {}", $this_if.get());
            /// there are no other cases
            unreachable!();
        }
        use crate::section_0487::*;
    }}
}
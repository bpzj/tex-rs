//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0249 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // assign_dimen: if chr_code<scaled_base then
        if $cmd == assign_dimen {
            if $chr_code.get() < scaled_base {
                // print_length_param(chr_code-dimen_base)
                todo!("dimen");
            }
            // else  begin print_esc("dimen"); print_int(chr_code-scaled_base);
            else {
                print_esc($globals, strpool_str!("dimen"));
                print_int($globals, ($chr_code.get() - scaled_base) as integer);
            }
            // end;
            use crate::section_0247::scaled_base;
            true
        } else {
            false
        }
    }}
}
//! ` `

// @<Get the next non-blank non-sign token...@>=
macro_rules! Set_the_next_non_blank_non_sign_token__set_negative_appropriately {
    ($globals:expr, $negative:expr) => {{
        // negative:=false;
        $negative = false;
        // repeat @<Get the next non-blank non-call token@>;
        loop {
            Get_the_next_non_blank_non_call_token!($globals);
            // if cur_tok=other_token+"-" then
            if $globals.cur_tok.get() == other_token + b'-' as cur_tok_type_repr {
                // begin negative := not negative; cur_tok:=other_token+"+";
                $negative = !$negative;
                $globals.cur_tok = cur_tok_type::new(other_token + b'+' as cur_tok_type_repr);
                // end;
            }
            // until cur_tok<>other_token+"+"
            if $globals.cur_tok.get() != other_token + b'+' as cur_tok_type_repr {
                break;
            }
        }
        use crate::section_0289::other_token;
        use crate::section_0297::cur_tok_type;
        use crate::section_0297::cur_tok_type_repr;
    }}
}

//! @ We leave the |space_factor| unchanged if |sf_code(cur_chr)=0|; otherwise we
//! set it equal to |sf_code(cur_chr)|, except that it should never change
//! from a value less than 1000 to a value exceeding 1000. The most common
//! case is |sf_code(cur_chr)=1000|, so we want that case to be fast.
//!
//! The overall structure of the main loop is presented here. Some program labels
//! are inside the individual sections.
//! @^inner loop@>

// @d adjust_space_factor==@t@>@;@/
macro_rules! adjust_space_factor {
    ($globals:expr) => {
        // main_s:=sf_code(cur_chr);
        $globals.main_s = sf_code!($globals, $globals.cur_chr.into()) as _;
        // if main_s=1000 then space_factor:=1000
        if $globals.main_s == 1000 {
            space_factor!($globals) = 1000;
        }
        // else if main_s<1000 then
        else if $globals.main_s < 1000 {
            // begin if main_s>0 then space_factor:=main_s;
            if $globals.main_s > 0 {
                space_factor!($globals) = $globals.main_s as _;
            }
            // end
        }
        // else if space_factor<1000 then space_factor:=1000
        else if space_factor!($globals) < 1000 {
            space_factor!($globals) = 1000
        }
        // else space_factor:=main_s
        else {
            space_factor!($globals) = $globals.main_s as _;
        }
    }
}

// @<Append character |cur_chr|...@>=
macro_rules! Append_character_cur_chr_and_the_following_characters_if_any_to_the_current_hlist_in_the_current_font__goto_reswitch_when_a_non_character_has_been_fetched {
    ($globals:expr, $lbl_reswitch:lifetime, $lbl_big_switch:lifetime) => {{
        // adjust_space_factor;@/
        adjust_space_factor!($globals);
        // main_f:=cur_font;
        // bchar:=font_bchar[main_f]; false_bchar:=font_false_bchar[main_f];
        // if mode>0 then if language<>clang then fix_language;
        // fast_get_avail(lig_stack); font(lig_stack):=main_f; cur_l:=qi(cur_chr);
        fast_get_avail!($globals, $globals.lig_stack);
        // character(lig_stack):=cur_l;@/
        // cur_q:=tail;
        // if cancel_boundary then
        if $globals.cancel_boundary {
            // begin cancel_boundary:=false; main_k:=non_address;
            $globals.cancel_boundary = false;
            $globals.main_k = non_address;
            // end
        }
        // else main_k:=bchar_label[main_f];
        else {
            $globals.main_k = $globals.bchar_label[$globals.main_f];
        }
        enum main_loop_status_kind {
            main_loop_wrapup,
            main_loop_move(usize),
            main_loop_lookahead,
            main_lig_loop(usize),
            main_loop_move_lig
        }
        use main_loop_status_kind::*;
        let mut main_loop_status;
        // if main_k=non_address then goto main_loop_move+2; {no left boundary processing}
        if $globals.main_k == non_address {
            // no left boundary processing
            const _ : () = ();
            main_loop_status = main_loop_move(2);
        }
        else {
            // cur_r:=cur_l; cur_l:=non_char;
            $globals.cur_r = $globals.cur_l;
            $globals.cur_l = non_char;
            // goto main_lig_loop+1; {begin with cursor after left boundary}
            /// begin with cursor after left boundary
            const _ : () = ();
            main_loop_status = main_lig_loop(1);
        }
        // @#
        region_multipart! {
            ('main_loop_append, main_loop_status) {
                // main_loop_wrapup:@<Make a ligature node, if |ligature_present|;
                //   insert a null discretionary, if appropriate@>;
                main_loop_status_kind::main_loop_wrapup => {
                    todo!("wrapup");
                    unreachable!();
                },
                // main_loop_move:@<If the cursor is immediately followed by the right boundary,
                //   |goto reswitch|; if it's followed by an invalid character, |goto big_switch|;
                //   otherwise move the cursor one step to the right and |goto main_lig_loop|@>;
                main_loop_status_kind::main_loop_move(part_idx) => {
                    If_the_cursor_is_immediately_followed_by_the_right_boundary_goto_reswitch__if_its_followed_by_an_invalid_character__goto_big_switch__otherwise_move_the_cursor_one_step_to_the_right_and_goto_main_lig_loop!
                        ($globals, part_idx, main_loop_status, 'main_loop_append, $lbl_reswitch, $lbl_big_switch);
                    unreachable!();
                },
                // main_loop_lookahead:@<Look ahead for another character, or leave |lig_stack|
                //   empty if there's none there@>;
                main_loop_status_kind::main_loop_lookahead => {
                    todo!("lookahead");
                    unreachable!();
                },
                // main_lig_loop:@<If there's a ligature/kern command relevant to |cur_l| and
                //   |cur_r|, adjust the text appropriately; exit to |main_loop_wrapup|@>;
                main_loop_status_kind::main_lig_loop(mut part_idx) => {
                    If_there_s_a_ligature_kern_command_relevant_to_cur_l_and_cur_r__adjust_the_text_appropriately__exit_to_main_loop_wrapup!
                        ($globals, part_idx, 'main_loop_append, main_loop_status);
                    unreachable!();
                },
                // main_loop_move_lig:@<Move the cursor past a pseudo-ligature, then
                //   |goto main_loop_lookahead| or |main_lig_loop|@>
                main_loop_status_kind::main_loop_move_lig => {
                    todo!("move_lig");
                    unreachable!();
                },                
            }
        }
        
        use crate::section_0549::non_address;
        use crate::section_0549::non_char;
    }}
}
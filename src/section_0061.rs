//! @ Here is the very first thing that \TeX\ prints: a headline that identifies
//! the version number and format package. The |term_offset| variable is temporarily
//! incorrect, but the discrepancy is not serious since we assume that the banner
//! and format identifier together will occupy at most |max_print_line|
//! character positions.

// @<Initialize the output...@>=
macro_rules! Initialize_the_output_routines {
    ($globals:expr) => {
        wterm($globals, banner);
        // wterm(banner);
        // if format_ident=0 then wterm_ln(' (no format preloaded)')
        // else  begin slow_print(format_ident); print_ln;
        //   end;
        // update_terminal;

        use crate::section_0002::banner;
        use crate::section_0056::wterm;
    };
}

//! @ While we're at it, we might as well deal with similar routines that
//! will be needed later.
//!
//! @<Declare procedures that scan restricted classes of integers@>=
//! procedure scan_four_bit_int;
//! begin scan_int;
//! if (cur_val<0)or(cur_val>15) then
//!   begin print_err("Bad number");
//! @.Bad number@>
//!   help2("Since I expected to read a number between 0 and 15,")@/
//!     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
//!   end;
//! end;
//!

//! @ The |input_ln| function brings the next line of input from the specified
//! file into available positions of the buffer array and returns the value
//! |true|, unless the file has already been entirely read, in which case it
//! returns |false| and sets |last:=first|.  In general, the |ASCII_code|
//! numbers that represent the next line of the file are input into
//! |buffer[first]|, |buffer[first+1]|, \dots, |buffer[last-1]|; and the
//! global variable |last| is set equal to |first| plus the length of the
//! line. Trailing blanks are removed from the line; thus, either |last=first|
//! (in which case the line was entirely blank) or |buffer[last-1]<>" "|.
//!
//! An overflow error is given, however, if the normal actions of |input_ln|
//! would make |last>=buf_size|; this is done so that other parts of \TeX\
//! can safely look at the contents of |buffer[last+1]| without overstepping
//! the bounds of the |buffer| array. Upon entry to |input_ln|, the condition
//! |first<buf_size| will always hold, so that there is always room for an
//! ``empty'' line.
//!
//! The variable |max_buf_stack|, which is used to keep track of how large
//! the |buf_size| parameter must be to accommodate the present job, is
//! also kept up to date by |input_ln|.
//!
//! If the |bypass_eoln| parameter is |true|, |input_ln| will do a |get|
//! before looking at the first character of the line; this skips over
//! an |eoln| that was in |f^|. The procedure does not do a |get| when it
//! reaches the end of the line; therefore it can be used to acquire input
//! from the user's terminal as well as from ordinary text files.
//!
//! Standard \PASCAL\ says that a file should have |eoln| immediately
//! before |eof|, but \TeX\ needs only a weaker restriction: If |eof|
//! occurs in the middle of a line, the system function |eoln| should return
//! a |true| result (even though |f^| will be undefined).
//!
//! Since the inner loop of |input_ln| is part of \TeX's ``inner loop''---each
//! character of input comes in at this place---it is wise to reduce system
//! overhead by making use of special routines that read in an entire array
//! of characters at once, if such routines are available. The following
//! code uses standard \PASCAL\ to illustrate what needs to be done, but
//! finer tuning is often possible at well-developed \PASCAL\ sites.
//! @^inner loop@>
//!
//! @p function input_ln(var f:alpha_file;@!bypass_eoln:boolean):boolean;
//!   {inputs the next line or returns |false|}
//! var last_nonblank:0..buf_size; {|last| with trailing blanks removed}
//! begin if bypass_eoln then if not eof(f) then get(f);
//!   {input the first character of the line into |f^|}
//! last:=first; {cf.\ Matthew 19\thinspace:\thinspace30}
//! if eof(f) then input_ln:=false
//! else  begin last_nonblank:=first;
//!   while not eoln(f) do
//!     begin if last>=max_buf_stack then
//!       begin max_buf_stack:=last+1;
//!       if max_buf_stack=buf_size then
//!         @<Report overflow of the input buffer, and abort@>;
//!       end;
//!     buffer[last]:=xord[f^]; get(f); incr(last);
//!     if buffer[last-1]<>" " then last_nonblank:=last;
//!     end;
//!   last:=last_nonblank; input_ln:=true;
//!   end;
//! end;
//!

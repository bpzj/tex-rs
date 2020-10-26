//! @ Here is a routine that manufactures the output file names, assuming that
//! |job_name<>0|. It ignores and changes the current settings of |cur_area|
//! and |cur_ext|.
//
// @d pack_cur_name==pack_file_name(cur_name,cur_area,cur_ext)
//
// @p procedure pack_job_name(@!s:str_number); {|s = ".log"|, |".dvi"|, or
//   |format_extension|}
// begin cur_area:=""; cur_ext:=s;
// cur_name:=job_name; pack_cur_name;
// end;
//
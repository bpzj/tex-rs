//! @ We can print the symbolic name of an integer parameter as follows.
//!
//! @p procedure print_param(@!n:integer);
//! begin case n of
//! pretolerance_code:print_esc("pretolerance");
//! tolerance_code:print_esc("tolerance");
//! line_penalty_code:print_esc("linepenalty");
//! hyphen_penalty_code:print_esc("hyphenpenalty");
//! ex_hyphen_penalty_code:print_esc("exhyphenpenalty");
//! club_penalty_code:print_esc("clubpenalty");
//! widow_penalty_code:print_esc("widowpenalty");
//! display_widow_penalty_code:print_esc("displaywidowpenalty");
//! broken_penalty_code:print_esc("brokenpenalty");
//! bin_op_penalty_code:print_esc("binoppenalty");
//! rel_penalty_code:print_esc("relpenalty");
//! pre_display_penalty_code:print_esc("predisplaypenalty");
//! post_display_penalty_code:print_esc("postdisplaypenalty");
//! inter_line_penalty_code:print_esc("interlinepenalty");
//! double_hyphen_demerits_code:print_esc("doublehyphendemerits");
//! final_hyphen_demerits_code:print_esc("finalhyphendemerits");
//! adj_demerits_code:print_esc("adjdemerits");
//! mag_code:print_esc("mag");
//! delimiter_factor_code:print_esc("delimiterfactor");
//! looseness_code:print_esc("looseness");
//! time_code:print_esc("time");
//! day_code:print_esc("day");
//! month_code:print_esc("month");
//! year_code:print_esc("year");
//! show_box_breadth_code:print_esc("showboxbreadth");
//! show_box_depth_code:print_esc("showboxdepth");
//! hbadness_code:print_esc("hbadness");
//! vbadness_code:print_esc("vbadness");
//! pausing_code:print_esc("pausing");
//! tracing_online_code:print_esc("tracingonline");
//! tracing_macros_code:print_esc("tracingmacros");
//! tracing_stats_code:print_esc("tracingstats");
//! tracing_paragraphs_code:print_esc("tracingparagraphs");
//! tracing_pages_code:print_esc("tracingpages");
//! tracing_output_code:print_esc("tracingoutput");
//! tracing_lost_chars_code:print_esc("tracinglostchars");
//! tracing_commands_code:print_esc("tracingcommands");
//! tracing_restores_code:print_esc("tracingrestores");
//! uc_hyph_code:print_esc("uchyph");
//! output_penalty_code:print_esc("outputpenalty");
//! max_dead_cycles_code:print_esc("maxdeadcycles");
//! hang_after_code:print_esc("hangafter");
//! floating_penalty_code:print_esc("floatingpenalty");
//! global_defs_code:print_esc("globaldefs");
//! cur_fam_code:print_esc("fam");
//! escape_char_code:print_esc("escapechar");
//! default_hyphen_char_code:print_esc("defaulthyphenchar");
//! default_skew_char_code:print_esc("defaultskewchar");
//! end_line_char_code:print_esc("endlinechar");
//! new_line_char_code:print_esc("newlinechar");
//! language_code:print_esc("language");
//! left_hyphen_min_code:print_esc("lefthyphenmin");
//! right_hyphen_min_code:print_esc("righthyphenmin");
//! holding_inserts_code:print_esc("holdinginserts");
//! error_context_lines_code:print_esc("errorcontextlines");
//! othercases print("[unknown integer parameter!]")
//! endcases;
//! end;
//!
//! @ The integer parameter names must be entered into the hash table.
//!
//! @<Put each...@>=
//! primitive("pretolerance",assign_int,int_base+pretolerance_code);@/
//! @!@:pretolerance_}{\.{\\pretolerance} primitive@>
//! primitive("tolerance",assign_int,int_base+tolerance_code);@/
//! @!@:tolerance_}{\.{\\tolerance} primitive@>
//! primitive("linepenalty",assign_int,int_base+line_penalty_code);@/
//! @!@:line_penalty_}{\.{\\linepenalty} primitive@>
//! primitive("hyphenpenalty",assign_int,int_base+hyphen_penalty_code);@/
//! @!@:hyphen_penalty_}{\.{\\hyphenpenalty} primitive@>
//! primitive("exhyphenpenalty",assign_int,int_base+ex_hyphen_penalty_code);@/
//! @!@:ex_hyphen_penalty_}{\.{\\exhyphenpenalty} primitive@>
//! primitive("clubpenalty",assign_int,int_base+club_penalty_code);@/
//! @!@:club_penalty_}{\.{\\clubpenalty} primitive@>
//! primitive("widowpenalty",assign_int,int_base+widow_penalty_code);@/
//! @!@:widow_penalty_}{\.{\\widowpenalty} primitive@>
//! primitive("displaywidowpenalty",
//!   assign_int,int_base+display_widow_penalty_code);@/
//! @!@:display_widow_penalty_}{\.{\\displaywidowpenalty} primitive@>
//! primitive("brokenpenalty",assign_int,int_base+broken_penalty_code);@/
//! @!@:broken_penalty_}{\.{\\brokenpenalty} primitive@>
//! primitive("binoppenalty",assign_int,int_base+bin_op_penalty_code);@/
//! @!@:bin_op_penalty_}{\.{\\binoppenalty} primitive@>
//! primitive("relpenalty",assign_int,int_base+rel_penalty_code);@/
//! @!@:rel_penalty_}{\.{\\relpenalty} primitive@>
//! primitive("predisplaypenalty",assign_int,int_base+pre_display_penalty_code);@/
//! @!@:pre_display_penalty_}{\.{\\predisplaypenalty} primitive@>
//! primitive("postdisplaypenalty",assign_int,int_base+post_display_penalty_code);@/
//! @!@:post_display_penalty_}{\.{\\postdisplaypenalty} primitive@>
//! primitive("interlinepenalty",assign_int,int_base+inter_line_penalty_code);@/
//! @!@:inter_line_penalty_}{\.{\\interlinepenalty} primitive@>
//! primitive("doublehyphendemerits",
//!   assign_int,int_base+double_hyphen_demerits_code);@/
//! @!@:double_hyphen_demerits_}{\.{\\doublehyphendemerits} primitive@>
//! primitive("finalhyphendemerits",
//!   assign_int,int_base+final_hyphen_demerits_code);@/
//! @!@:final_hyphen_demerits_}{\.{\\finalhyphendemerits} primitive@>
//! primitive("adjdemerits",assign_int,int_base+adj_demerits_code);@/
//! @!@:adj_demerits_}{\.{\\adjdemerits} primitive@>
//! primitive("mag",assign_int,int_base+mag_code);@/
//! @!@:mag_}{\.{\\mag} primitive@>
//! primitive("delimiterfactor",assign_int,int_base+delimiter_factor_code);@/
//! @!@:delimiter_factor_}{\.{\\delimiterfactor} primitive@>
//! primitive("looseness",assign_int,int_base+looseness_code);@/
//! @!@:looseness_}{\.{\\looseness} primitive@>
//! primitive("time",assign_int,int_base+time_code);@/
//! @!@:time_}{\.{\\time} primitive@>
//! primitive("day",assign_int,int_base+day_code);@/
//! @!@:day_}{\.{\\day} primitive@>
//! primitive("month",assign_int,int_base+month_code);@/
//! @!@:month_}{\.{\\month} primitive@>
//! primitive("year",assign_int,int_base+year_code);@/
//! @!@:year_}{\.{\\year} primitive@>
//! primitive("showboxbreadth",assign_int,int_base+show_box_breadth_code);@/
//! @!@:show_box_breadth_}{\.{\\showboxbreadth} primitive@>
//! primitive("showboxdepth",assign_int,int_base+show_box_depth_code);@/
//! @!@:show_box_depth_}{\.{\\showboxdepth} primitive@>
//! primitive("hbadness",assign_int,int_base+hbadness_code);@/
//! @!@:hbadness_}{\.{\\hbadness} primitive@>
//! primitive("vbadness",assign_int,int_base+vbadness_code);@/
//! @!@:vbadness_}{\.{\\vbadness} primitive@>
//! primitive("pausing",assign_int,int_base+pausing_code);@/
//! @!@:pausing_}{\.{\\pausing} primitive@>
//! primitive("tracingonline",assign_int,int_base+tracing_online_code);@/
//! @!@:tracing_online_}{\.{\\tracingonline} primitive@>
//! primitive("tracingmacros",assign_int,int_base+tracing_macros_code);@/
//! @!@:tracing_macros_}{\.{\\tracingmacros} primitive@>
//! primitive("tracingstats",assign_int,int_base+tracing_stats_code);@/
//! @!@:tracing_stats_}{\.{\\tracingstats} primitive@>
//! primitive("tracingparagraphs",assign_int,int_base+tracing_paragraphs_code);@/
//! @!@:tracing_paragraphs_}{\.{\\tracingparagraphs} primitive@>
//! primitive("tracingpages",assign_int,int_base+tracing_pages_code);@/
//! @!@:tracing_pages_}{\.{\\tracingpages} primitive@>
//! primitive("tracingoutput",assign_int,int_base+tracing_output_code);@/
//! @!@:tracing_output_}{\.{\\tracingoutput} primitive@>
//! primitive("tracinglostchars",assign_int,int_base+tracing_lost_chars_code);@/
//! @!@:tracing_lost_chars_}{\.{\\tracinglostchars} primitive@>
//! primitive("tracingcommands",assign_int,int_base+tracing_commands_code);@/
//! @!@:tracing_commands_}{\.{\\tracingcommands} primitive@>
//! primitive("tracingrestores",assign_int,int_base+tracing_restores_code);@/
//! @!@:tracing_restores_}{\.{\\tracingrestores} primitive@>
//! primitive("uchyph",assign_int,int_base+uc_hyph_code);@/
//! @!@:uc_hyph_}{\.{\\uchyph} primitive@>
//! primitive("outputpenalty",assign_int,int_base+output_penalty_code);@/
//! @!@:output_penalty_}{\.{\\outputpenalty} primitive@>
//! primitive("maxdeadcycles",assign_int,int_base+max_dead_cycles_code);@/
//! @!@:max_dead_cycles_}{\.{\\maxdeadcycles} primitive@>
//! primitive("hangafter",assign_int,int_base+hang_after_code);@/
//! @!@:hang_after_}{\.{\\hangafter} primitive@>
//! primitive("floatingpenalty",assign_int,int_base+floating_penalty_code);@/
//! @!@:floating_penalty_}{\.{\\floatingpenalty} primitive@>
//! primitive("globaldefs",assign_int,int_base+global_defs_code);@/
//! @!@:global_defs_}{\.{\\globaldefs} primitive@>
//! primitive("fam",assign_int,int_base+cur_fam_code);@/
//! @!@:fam_}{\.{\\fam} primitive@>
//! primitive("escapechar",assign_int,int_base+escape_char_code);@/
//! @!@:escape_char_}{\.{\\escapechar} primitive@>
//! primitive("defaulthyphenchar",assign_int,int_base+default_hyphen_char_code);@/
//! @!@:default_hyphen_char_}{\.{\\defaulthyphenchar} primitive@>
//! primitive("defaultskewchar",assign_int,int_base+default_skew_char_code);@/
//! @!@:default_skew_char_}{\.{\\defaultskewchar} primitive@>
//! primitive("endlinechar",assign_int,int_base+end_line_char_code);@/
//! @!@:end_line_char_}{\.{\\endlinechar} primitive@>
//! primitive("newlinechar",assign_int,int_base+new_line_char_code);@/
//! @!@:new_line_char_}{\.{\\newlinechar} primitive@>
//! primitive("language",assign_int,int_base+language_code);@/
//! @!@:language_}{\.{\\language} primitive@>
//! primitive("lefthyphenmin",assign_int,int_base+left_hyphen_min_code);@/
//! @!@:left_hyphen_min_}{\.{\\lefthyphenmin} primitive@>
//! primitive("righthyphenmin",assign_int,int_base+right_hyphen_min_code);@/
//! @!@:right_hyphen_min_}{\.{\\righthyphenmin} primitive@>
//! primitive("holdinginserts",assign_int,int_base+holding_inserts_code);@/
//! @!@:holding_inserts_}{\.{\\holdinginserts} primitive@>
//! primitive("errorcontextlines",assign_int,int_base+error_context_lines_code);@/
//! @!@:error_context_lines_}{\.{\\errorcontextlines} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_int: if chr_code<count_base then print_param(chr_code-int_base)
//!   else  begin print_esc("count"); print_int(chr_code-count_base);
//!     end;
//!
//! @ The integer parameters should really be initialized by a macro package;
//! the following initialization does the minimum to keep \TeX\ from
//! complete failure.
//! @^null delimiter@>
//!
//! @<Initialize table entries...@>=
//! for k:=int_base to del_code_base-1 do eqtb[k].int:=0;
//! mag:=1000; tolerance:=10000; hang_after:=1; max_dead_cycles:=25;
//! escape_char:="\"; end_line_char:=carriage_return;
//! for k:=0 to 255 do del_code(k):=-1;
//! del_code("."):=0; {this null delimiter is used in error recovery}
//!
//! @ The following procedure, which is called just before \TeX\ initializes its
//! input and output, establishes the initial values of the date and time.
//! @^system dependencies@>
//! Since standard \PASCAL\ cannot provide such information, something special
//! is needed. The program here simply specifies July 4, 1776, at noon; but
//! users probably want a better approximation to the truth.
//!
//! @p procedure fix_date_and_time;
//! begin time:=12*60; {minutes since midnight}
//! day:=4; {fourth day of the month}
//! month:=7; {seventh month of the year}
//! year:=1776; {Anno Domini}
//! end;
//!
//! @ @<Show equivalent |n|, in region 5@>=
//! begin if n<count_base then print_param(n-int_base)
//! else if  n<del_code_base then
//!   begin print_esc("count"); print_int(n-count_base);
//!   end
//! else  begin print_esc("delcode"); print_int(n-del_code_base);
//!   end;
//! print_char("="); print_int(eqtb[n].int);
//! end
//!
//! @ @<Set variable |c| to the current escape character@>=c:=escape_char
//!
//! @ @<Character |s| is the current new-line character@>=s=new_line_char
//!
//! @ \TeX\ is occasionally supposed to print diagnostic information that
//! goes only into the transcript file, unless |tracing_online| is positive.
//! Here are two routines that adjust the destination of print commands:
//!
//! @p procedure begin_diagnostic; {prepare to do some tracing}
//! begin old_setting:=selector;
//! if (tracing_online<=0)and(selector=term_and_log) then
//!   begin decr(selector);
//!   if history=spotless then history:=warning_issued;
//!   end;
//! end;
//! @#
//! procedure end_diagnostic(@!blank_line:boolean);
//!   {restore proper conditions after tracing}
//! begin print_nl("");
//! if blank_line then print_ln;
//! selector:=old_setting;
//! end;
//!
//! @ Of course we had better declare another global variable, if the previous
//! routines are going to work.
//!
//! @<Glob...@>=
//! @!old_setting:0..max_selector;
//!

//! @* \[19] Saving and restoring equivalents.
//! The nested structure provided by `$\.{\char'173}\ldots\.{\char'175}$' groups
//! in \TeX\ means that |eqtb| entries valid in outer groups should be saved
//! and restored later if they are overridden inside the braces. When a new |eqtb|
//! value is being assigned, the program therefore checks to see if the previous
//! entry belongs to an outer level. In such a case, the old value is placed
//! on the |save_stack| just before the new value enters |eqtb|. At the
//! end of a grouping level, i.e., when the right brace is sensed, the
//! |save_stack| is used to restore the outer values, and the inner ones are
//! destroyed.
//!
//! Entries on the |save_stack| are of type |memory_word|. The top item on
//! this stack is |save_stack[p]|, where |p=save_ptr-1|; it contains three
//! fields called |save_type|, |save_level|, and |save_index|, and it is
//! interpreted in one of four ways:
//!
//! \yskip\hangg 1) If |save_type(p)=restore_old_value|, then
//! |save_index(p)| is a location in |eqtb| whose current value should
//! be destroyed at the end of the current group and replaced by |save_stack[p-1]|.
//! Furthermore if |save_index(p)>=int_base|, then |save_level(p)|
//! should replace the corresponding entry in |xeq_level|.
//!
//! \yskip\hangg 2) If |save_type(p)=restore_zero|, then |save_index(p)|
//! is a location in |eqtb| whose current value should be destroyed at the end
//! of the current group, when it should be
//! replaced by the current value of |eqtb[undefined_control_sequence]|.
//!
//! \yskip\hangg 3) If |save_type(p)=insert_token|, then |save_index(p)|
//! is a token that should be inserted into \TeX's input when the current
//! group ends.
//!
//! \yskip\hangg 4) If |save_type(p)=level_boundary|, then |save_level(p)|
//! is a code explaining what kind of group we were previously in, and
//! |save_index(p)| points to the level boundary word at the bottom of
//! the entries for that group.
//!
//! @d save_type(#)==save_stack[#].hh.b0 {classifies a |save_stack| entry}
//! @d save_level(#)==save_stack[#].hh.b1
//!   {saved level for regions 5 and 6, or group code}
//! @d save_index(#)==save_stack[#].hh.rh
//!   {|eqtb| location or token or |save_stack| location}
//! @d restore_old_value=0 {|save_type| when a value should be restored later}
//! @d restore_zero=1 {|save_type| when an undefined entry should be restored}
//! @d insert_token=2 {|save_type| when a token is being saved for later use}
//! @d level_boundary=3 {|save_type| corresponding to beginning of group}
//!
//! @ Here are the group codes that are used to discriminate between different
//! kinds of groups. They allow \TeX\ to decide what special actions, if any,
//! should be performed when a group ends.
//! \def\grp{\.{\char'173...\char'175}}
//!
//! Some groups are not supposed to be ended by right braces. For example,
//! the `\.\$' that begins a math formula causes a |math_shift_group| to
//! be started, and this should be terminated by a matching `\.\$'. Similarly,
//! a group that starts with \.{\\left} should end with \.{\\right}, and
//! one that starts with \.{\\begingroup} should end with \.{\\endgroup}.
//!
//! @d bottom_level=0 {group code for the outside world}
//! @d simple_group=1 {group code for local structure only}
//! @d hbox_group=2 {code for `\.{\\hbox}\grp'}
//! @d adjusted_hbox_group=3 {code for `\.{\\hbox}\grp' in vertical mode}
//! @d vbox_group=4 {code for `\.{\\vbox}\grp'}
//! @d vtop_group=5 {code for `\.{\\vtop}\grp'}
//! @d align_group=6 {code for `\.{\\halign}\grp', `\.{\\valign}\grp'}
//! @d no_align_group=7 {code for `\.{\\noalign}\grp'}
//! @d output_group=8 {code for output routine}
//! @d math_group=9 {code for, e.g., `\.{\char'136}\grp'}
//! @d disc_group=10 {code for `\.{\\discretionary}\grp\grp\grp'}
//! @d insert_group=11 {code for `\.{\\insert}\grp', `\.{\\vadjust}\grp'}
//! @d vcenter_group=12 {code for `\.{\\vcenter}\grp'}
//! @d math_choice_group=13 {code for `\.{\\mathchoice}\grp\grp\grp\grp'}
//! @d semi_simple_group=14 {code for `\.{\\begingroup...\\endgroup}'}
//! @d math_shift_group=15 {code for `\.{\$...\$}'}
//! @d math_left_group=16 {code for `\.{\\left...\\right}'}
//! @d max_group_code=16
//!
//! @<Types...@>=
//! @!group_code=0..max_group_code; {|save_level| for a level boundary}
//!
//! @ The global variable |cur_group| keeps track of what sort of group we are
//! currently in. Another global variable, |cur_boundary|, points to the
//! topmost |level_boundary| word.  And |cur_level| is the current depth of
//! nesting. The routines are designed to preserve the condition that no entry
//! in the |save_stack| or in |eqtb| ever has a level greater than |cur_level|.
//!
//! @ @<Glob...@>=
//! @!save_stack : array[0..save_size] of memory_word;
//! @!save_ptr : 0..save_size; {first unused entry on |save_stack|}
//! @!max_save_stack:0..save_size; {maximum usage of save stack}
//! @!cur_level: quarterword; {current nesting level for groups}
//! @!cur_group: group_code; {current group type}
//! @!cur_boundary: 0..save_size; {where the current level begins}
//!
//! @ At this time it might be a good idea for the reader to review the introduction
//! to |eqtb| that was given above just before the long lists of parameter names.
//! Recall that the ``outer level'' of the program is |level_one|, since
//! undefined control sequences are assumed to be ``defined'' at |level_zero|.
//!
//! @<Set init...@>=
//! save_ptr:=0; cur_level:=level_one; cur_group:=bottom_level; cur_boundary:=0;
//! max_save_stack:=0;
//!
//! @ The following macro is used to test if there is room for up to six more
//! entries on |save_stack|. By making a conservative test like this, we can
//! get by with testing for overflow in only a few places.
//!
//! @d check_full_save_stack==if save_ptr>max_save_stack then
//!   begin max_save_stack:=save_ptr;
//!   if max_save_stack>save_size-6 then overflow("save size",save_size);
//! @:TeX capacity exceeded save size}{\quad save size@>
//!   end
//!
//! @ Procedure |new_save_level| is called when a group begins. The
//! argument is a group identification code like `|hbox_group|'. After
//! calling this routine, it is safe to put five more entries on |save_stack|.
//!
//! In some cases integer-valued items are placed onto the
//! |save_stack| just below a |level_boundary| word, because this is a
//! convenient place to keep information that is supposed to ``pop up'' just
//! when the group has finished.
//! For example, when `\.{\\hbox to 100pt}\grp' is being treated, the 100pt
//! dimension is stored on |save_stack| just before |new_save_level| is
//! called.
//!
//! We use the notation |saved(k)| to stand for an integer item that
//! appears in location |save_ptr+k| of the save stack.
//!
//! @d saved(#)==save_stack[save_ptr+#].int
//!
//! @p procedure new_save_level(@!c:group_code); {begin a new level of grouping}
//! begin check_full_save_stack;
//! save_type(save_ptr):=level_boundary; save_level(save_ptr):=cur_group;
//! save_index(save_ptr):=cur_boundary;
//! if cur_level=max_quarterword then overflow("grouping levels",
//! @:TeX capacity exceeded grouping levels}{\quad grouping levels@>
//!   max_quarterword-min_quarterword);
//!   {quit if |(cur_level+1)| is too big to be stored in |eqtb|}
//! cur_boundary:=save_ptr; incr(cur_level); incr(save_ptr); cur_group:=c;
//! end;
//!
//! @ Just before an entry of |eqtb| is changed, the following procedure should
//! be called to update the other data structures properly. It is important
//! to keep in mind that reference counts in |mem| include references from
//! within |save_stack|, so these counts must be handled carefully.
//! @^reference counts@>
//!
//! @p procedure eq_destroy(@!w:memory_word); {gets ready to forget |w|}
//! var q:pointer; {|equiv| field of |w|}
//! begin case eq_type_field(w) of
//! call,long_call,outer_call,long_outer_call: delete_token_ref(equiv_field(w));
//! glue_ref: delete_glue_ref(equiv_field(w));
//! shape_ref: begin q:=equiv_field(w); {we need to free a \.{\\parshape} block}
//!   if q<>null then free_node(q,info(q)+info(q)+1);
//!   end; {such a block is |2n+1| words long, where |n=info(q)|}
//! box_ref: flush_node_list(equiv_field(w));
//! othercases do_nothing
//! endcases;
//! end;
//!
//! @ To save a value of |eqtb[p]| that was established at level |l|, we
//! can use the following subroutine.
//!
//! @p procedure eq_save(@!p:pointer;@!l:quarterword); {saves |eqtb[p]|}
//! begin check_full_save_stack;
//! if l=level_zero then save_type(save_ptr):=restore_zero
//! else  begin save_stack[save_ptr]:=eqtb[p]; incr(save_ptr);
//!   save_type(save_ptr):=restore_old_value;
//!   end;
//! save_level(save_ptr):=l; save_index(save_ptr):=p; incr(save_ptr);
//! end;
//!
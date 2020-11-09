//! @ The |expand| procedure and some other routines that construct token
//! lists find it convenient to use the following macros, which are valid only if
//! the variables |p| and |q| are reserved for token-list building.

// @d store_new_token(#)==begin q:=get_avail; link(p):=q; info(q):=#;
macro_rules! store_new_token {
    ($globals:expr, $val:expr, $p:expr, $q:expr) => {{
        $q = get_avail($globals);
        link!($globals, $p) = $q;
        #[cfg(not(feature = "unicode_support"))]
        {
            info!($globals, $q) = $val;
        }
        #[cfg(feature = "unicode_support")]
        {
            info!($globals, $q) = crate::unicode_support::register_info_value($globals, $val);
        }
        // p:=q; {|link(p)| is |null|}
        $p = $q;
        /// `link(p)` is `null`
        const _ : () = ();
        // end
        use crate::section_0120::get_avail;
    }}
}

// @d fast_store_new_token(#)==begin fast_get_avail(q); link(p):=q; info(q):=#;
//   p:=q; {|link(p)| is |null|}
//   end
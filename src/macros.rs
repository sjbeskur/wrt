
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ( $( $k:expr => $v:expr ),+ $(,)? ) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($k, $v); )+
        map
    }};
}

#[macro_export]
macro_rules! apply_transform {
    ( $T:expr , $p:expr ) => {
        $T.apply_to_point(&$p)
    };
}

#[macro_export]
macro_rules! compose {
    ( $T_ab:expr , $T_bc:expr ) => {
        $T_ab.compose(&$T_bc)
    };
}

#[macro_export]
macro_rules! to_frame {
    ( $p:expr , from: $T_from:expr , to: $T_to:expr ) => {
        $T_to.inverse().unwrap().apply_to_point(&$T_from.apply_to_point(&$p))
    };
}

#[macro_export]
macro_rules! rotate {
    // allow natural DSL: `rotate!(A wrt B)` where A and B are identifiers
    ( $A:ident wrt $B:ident ) => {
        $A.rotate_wrt(&$B)
    };
    // angle form: `rotate!(A, by 90.0, wrt B)` where angle is degrees (f32/f64/int)
    ( $A:ident , by $angle:expr , wrt $B:ident ) => {
        $A.rotated_by_z_degrees($angle as f64).rotate_wrt(&$B)
    };
    // allow expression form with an explicit comma: `rotate!(expr, wrt expr)`
    ( $A:expr , wrt $B:expr ) => {
        $A.rotate_wrt(&$B)
    };
    // expression angle form: `rotate!(expr, by expr, wrt expr)`
    ( $A:expr , by $angle:expr , wrt $B:expr ) => {
        $A.rotated_by_z_degrees($angle as f64).rotate_wrt(&$B)
    };
}

#[macro_export]
macro_rules! transform {
    // DSL: `transform!(rotate A wrt B)` where A and B are identifiers
    ( rotate $A:ident wrt $B:ident ) => {
        crate::rotate!($A wrt $B)
    };
    // expression form with comma: `transform!(rotate expr, wrt expr)`
    ( rotate $A:expr , wrt $B:expr ) => {
        crate::rotate!($A , wrt $B)
    };
    ( rotate $A:ident , by $angle:expr , wrt $B:ident ) => {
        crate::rotate!($A , by $angle , wrt $B)
    };
    ( rotate $A:expr , by $angle:expr , wrt $B:expr ) => {
        crate::rotate!($A , by $angle , wrt $B)
    };
    ( $op:ident $A:expr , wrt $B:expr ) => {
        compile_error!(concat!("Unsupported transform op: ", stringify!($op)));
    };
}


mod tests {
    
    #[test]
    fn hashmap_macro_works() {
        let m = hashmap!{"a" => 1, "b" => 2};
        assert_eq!(m.get("a"), Some(&1));
        assert_eq!(m.get("b"), Some(&2));
        assert_eq!(m.len(), 2);
    }

}
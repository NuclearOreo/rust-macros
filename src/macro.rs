#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = HashSet::new();
            $(
                temp_vec.insert($x);
            )*
            temp_vec
        }
    };
}

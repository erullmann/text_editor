// macros
//

#[macro_export]
macro_rules! ctrl {
    ( $x:tt ) => {
        {
            ((($x as u8) & 0x1f) as char)
        }
    };
}


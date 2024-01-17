macro_rules! csi {
    ($name:ident, $value:expr) => {
        pub struct $name;

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "\x1b[{}", $value)
            }
        }
    };
}

pub mod alternate;
pub mod clear;
pub mod cursor;

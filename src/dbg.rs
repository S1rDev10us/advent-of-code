#[macro_export]
macro_rules! display {
    ($struct:ident, $self:ident, $func:tt) => {
        impl std::fmt::Display for $struct {
            fn fmt(&$self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    {$func}
                )
            }
        }
    };
}

pub use display;

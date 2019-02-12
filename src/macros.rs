#[macro_export]
macro_rules! error_exit {
    ($l:literal $(,$arg:expr)*) => {
        {
            eprintln!($l, $($arg),*);
            std::process::exit(1);
        }
    };
}

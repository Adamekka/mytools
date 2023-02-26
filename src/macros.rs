#![allow(unused_macros)]

macro_rules! pretty_panic {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        print!("{}", "Error: ".red().bold());
        println!("{}", format!($msg));
        #[cfg(debug_assertions)]
        panic!();
        #[cfg(not(debug_assertions))]
        std::process::exit(1);
    };
}

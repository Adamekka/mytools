#![allow(unused_macros)]

pub extern crate owo_colors;

#[macro_export]
macro_rules! pretty_panic {
    ($msg:expr) => {
        use mytools::macros::owo_colors::OwoColorize;
        print!("{}", "Error: ".red().bold());
        println!("{}", format!($msg));
        #[cfg(debug_assertions)]
        panic!();
        #[cfg(not(debug_assertions))]
        std::process::exit(1);
    };
}

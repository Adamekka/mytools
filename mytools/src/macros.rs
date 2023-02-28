#![allow(unused_macros)]

pub extern crate owo_colors;

/// A macro to print a pretty panic message.
///
/// # Examples
///
/// ```should_panic
/// #[macro_use]
/// extern crate mytools;
///
/// fn main() {
///    pretty_panic!("Error message");
/// }
/// ```
///
/// # Output
///
/// ```text
/// Error: Error message
/// ```
///
/// "Error: " is red and bold.
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

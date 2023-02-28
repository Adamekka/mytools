#![allow(unused_macros)]

pub extern crate owo_colors;
pub extern crate question;

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

/// A macro to ask a yes/no question.
///
/// - If the answer is yes, program will continue
/// - If the answer is no, program will panic
///
/// Macro contains a loop that will continue to ask the question until the user
/// enters 'y' or 'n'.
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate mytools;
///
/// fn main() {
///    question_yes_no!("Do you want to continue?");
/// }
/// ```
///
/// # Output
///
/// ```text
/// Do you want to continue? (y/n)
/// ```
#[macro_export]
macro_rules! question_yes_no {
    ($text:expr) => {
        let answer: Option<mytools::macros::question::Answer>;

        answer = mytools::macros::question::Question::new($text)
            .yes_no()
            .show_defaults()
            .until_acceptable()
            .ask();

        match answer {
            Some(mytools::macros::question::Answer::YES) => {}
            Some(mytools::macros::question::Answer::NO) => {
                mytools::pretty_panic!("Aborting");
            }
            Some(mytools::macros::question::Answer::RESPONSE(_)) => {
                unreachable!("Something went wrong");
            }
            None => {
                unreachable!("Something went wrong");
            }
        }
    };
}

/// A macro to print a pretty warning message.
///
/// # Examples
///
/// ```no_run
/// #[macro_use]
/// extern crate mytools;
///
/// fn main() {
///    warn!("Warning message");
/// }
/// ```
///
/// # Output
///
/// ```text
/// Warning: Warning message
/// ```
///
/// "Warning: " is yellow and bold.
#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        use mytools::macros::owo_colors::OwoColorize;
        print!("{}", "Warning: ".yellow().bold());
        println!("{}", format!($msg));
    };
}

/// Get the home folder of the current user
///
/// # Examples
///
/// ```no_run
/// extern crate mytools;
///
/// let home = mytools::env::get_home_folder();
/// println!("Home folder: {home}");
/// ```
///
/// # Output
///
/// ```text
/// /home/username
/// ```
///
/// # Panics
///
/// - If $HOME environment variable isn't set on Unix
/// - If $USERPROFILE environment variable isn't set on Windows
///
/// > Note: WebAssembly isn't supported
pub fn get_home_folder() -> String {
    #[cfg(target_family = "unix")]
    return std::env::var("HOME").expect("$HOME environment variable isn't set");

    #[cfg(target_family = "windows")]
    return std::env::var("USERPROFILE").expect("$USERPROFILE environment variable isn't set");

    #[cfg(target_family = "wasm")]
    pretty_panic!("WebAssembly isn't supported");
}

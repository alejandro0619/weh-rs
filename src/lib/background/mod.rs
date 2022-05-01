use std::process::Command;

/// .
///
/// # Examples
///
/// ```
/// use weh_lib::background::set_background;
///
/// set_background();
/// ```
///
/// # Panics
///
/// Panics if .
pub fn set_background() {
    Command::new("feh")
        .args(["--bg-fill", "fewwis but nord.png"])
        .status()
        .expect("err");
}

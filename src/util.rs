#[cfg(windows)]
pub const LINE_ENDING: &'static str = "\r\n";

#[cfg(windows)]
pub const LINE_ENDING2: &'static str = "\r\n\r\n";

#[cfg(not(windows))]
pub const LINE_ENDING: &'static str = "\n";

#[cfg(not(windows))]
pub const LINE_ENDING2: &'static str = "\n\n";

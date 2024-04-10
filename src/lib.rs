// alice currently only supports UNIX-like systems but we might as well make the structure for other systems
#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use self::unix as system;

pub mod alternate;
pub mod arg;
pub mod editor;
pub mod escape;
pub mod event;
pub mod highlight;
pub mod input;
pub mod raw;

fn digits(number: usize) -> usize {
    number.checked_ilog10().unwrap_or(0) as usize + 1
}

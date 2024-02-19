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
pub mod input;
pub mod raw;

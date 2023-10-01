// alice currently only supports UNIX-like systems but we might as well make the structure for other systems
#[cfg(unix)]
mod unix;
#[cfg(unix)]
use self::unix as system;

pub mod buffer;
pub mod event;
pub mod input;
pub mod raw;

// alice currently only supports UNIX-like systems but we might as well make the structure for other platforms
#[cfg_attr(unix, path = "unix/mod.rs")]
mod system;

pub mod buffer;
pub mod event;
pub mod input;
pub mod raw;

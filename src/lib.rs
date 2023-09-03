// alice currently only supports UNIX-like systems but we might as well make the structure for other platforms
#[cfg_attr(unix, path = "unix.rs")]
mod system;

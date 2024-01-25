mod builder;
mod config;
mod group;
mod single;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use builder::*;
pub use config::*;
pub use group::*;
pub use single::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct ReadFormatHead {
    pub members_len: u64,  // u64 nr;
    pub time_enabled: u64, // u64 time_enabled;
    pub time_running: u64, // u64 time_running;
                           // ReadFormatValue values[nr];
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct ReadFormatValue {
    pub event_count: u64, // u64 value;
    pub event_id: u64,    // u64 id;
}

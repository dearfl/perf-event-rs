use crate::syscall;
use crate::syscall::bindings::{perf_event_attr, perf_event_ioctls};
use crate::syscall::{ioctl, perf_event_open};
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::os::fd::{AsRawFd, FromRawFd};

mod attr;
mod builder;
mod event;

use crate::infra::result::WrapResult;
pub use attr::*;
#[allow(unused_imports)]
pub use builder::*;
pub use event::*;

pub struct Counting {
    // TODO
    raw_attr: Box<perf_event_attr>,
    file: File,
}

impl Counting {
    unsafe fn new(
        attr: CountingAttr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> Result<Self, Error> {
        let raw_attr = Box::new(attr.into_raw());
        let i32 = unsafe { perf_event_open(&*raw_attr as *const _, pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => Self {
                raw_attr,
                file: File::from_raw_fd(fd),
            }
            .wrap_ok(),
        }
    }

    pub fn get_count(&mut self) -> io::Result<usize> {
        let mut buf = [0_u8; std::mem::size_of::<usize>()];

        match self.file.read_exact(&mut buf) {
            Ok(()) => usize::from_le_bytes(buf).wrap_ok(),
            Err(e) => Err(e),
        }
    }

    fn perf_event_ioctl(&self, op: perf_event_ioctls) -> io::Result<()> {
        let i32 = unsafe { ioctl(self.file.as_raw_fd() as libc::c_int, op as libc::c_ulong, 0) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    fn perf_event_ioctl_with_arg<A>(&self, op: perf_event_ioctls, arg: A) -> io::Result<()> {
        let i32 = unsafe {
            ioctl(
                self.file.as_raw_fd() as libc::c_int,
                op as libc::c_ulong,
                arg,
            )
        };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_ENABLE)
    }

    pub fn enable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_ENABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_DISABLE)
    }

    pub fn disable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_DISABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    /*
        // TODO
        pub fn refresh(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_REFRESH)
            todo!()
        }
    */

    pub fn reset_count(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_RESET)
    }

    pub fn reset_count_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_RESET,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PERIOD, &new)
    }

    pub fn set_output(&self, new: File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, raw_fd)
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, -1i64)
    }

    /*
        // TODO
        pub fn set_filter(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_FILTER)
            todo!()
        }
    */

    pub fn id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_ID, &mut id)?;
        Ok(id)
    }

    /*
        // TODO
        pub fn set_bpf(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_BPF)
            todo!()
        }

        // TODO: sampling mode only
        pub fn pause_output(&self) -> io::Result<()> {
            self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 1i32)
        }

        // TODO: sampling mode only
        pub fn resume_output(&self) -> io::Result<()> {
            self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 0i32)
        }

        // TODO
        pub fn query_bpf(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_QUERY_BPF)
            todo!()
        }

        // TODO
        pub fn modify_attributes(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_MODIFY_ATTRIBUTES)
            todo!()
        }
    */
}

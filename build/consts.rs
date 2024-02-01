// [(major, patch_level, is_selected)]
#[rustfmt::skip]
pub const LINUX_FEATURE_VERSIONS: [(usize, usize, bool); 27] = [
    (6, 3, cfg!(feature = "linux-6.3" )),
    (6, 0, cfg!(feature = "linux-6.0" )),
    (5,16, cfg!(feature = "linux-5.16")),
    (5,13, cfg!(feature = "linux-5.13")),
    (5,12, cfg!(feature = "linux-5.12")),
    (5,11, cfg!(feature = "linux-5.11")),
    (5, 9, cfg!(feature = "linux-5.9" )),
    (5, 8, cfg!(feature = "linux-5.8" )),
    (5, 7, cfg!(feature = "linux-5.7" )),
    (5, 5, cfg!(feature = "linux-5.5" )),
    (5, 4, cfg!(feature = "linux-5.4" )),
    (5, 1, cfg!(feature = "linux-5.1" )),
    (4,17, cfg!(feature = "linux-4.17")),
    (4,16, cfg!(feature = "linux-4.16")),
    (4,14, cfg!(feature = "linux-4.14")),
    (4,12, cfg!(feature = "linux-4.12")),
    (4,10, cfg!(feature = "linux-4.10")),
    (4, 8, cfg!(feature = "linux-4.8" )),
    (4, 7, cfg!(feature = "linux-4.7" )),
    (4, 4, cfg!(feature = "linux-4.4" )),
    (4, 3, cfg!(feature = "linux-4.3" )),
    (4, 2, cfg!(feature = "linux-4.2" )),
    (4, 1, cfg!(feature = "linux-4.1" )),
    (3,19, cfg!(feature = "linux-3.19")),
    (3,16, cfg!(feature = "linux-3.16")),
    (3,13, cfg!(feature = "linux-3.13")),
    (3,12, cfg!(feature = "linux-3.12")),
];

// [(major, patch_level, enum_entry)]
#[rustfmt::skip]
pub const IOCTLS: [(usize, usize, &str); 5] = [
    (3,12, "PERF_EVENT_IOCTL_ID                = PERF_EVENT_IOC_ID,"               ),
    (4, 1, "PERF_EVENT_IOCTL_SET_BPF           = PERF_EVENT_IOC_SET_BPF,"          ),
    (4, 7, "PERF_EVENT_IOCTL_PAUSE_OUTPUT      = PERF_EVENT_IOC_PAUSE_OUTPUT,"     ),
    (4,16, "PERF_EVENT_IOCTL_QUERY_BPF         = PERF_EVENT_IOC_QUERY_BPF,"        ),
    (4,17, "PERF_EVENT_IOCTL_MODIFY_ATTRIBUTES = PERF_EVENT_IOC_MODIFY_ATTRIBUTES,"),
];

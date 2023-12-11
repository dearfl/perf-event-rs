use crate::syscall::bindings::*;

/// Select the fields contained in `sample::Body`
#[derive(Debug, Clone, Default)]
pub struct SampleRecordFields {
    pub sample_id: bool,   // PERF_SAMPLE_IDENTIFIER
    pub ip: bool,          // PERF_SAMPLE_IP
    pub pid_and_tid: bool, // PERF_SAMPLE_TID
    pub time: bool,        // PERF_SAMPLE_TIME
    pub addr: bool,        // PERF_SAMPLE_ADDR
    pub id: bool,          // PERF_SAMPLE_ID
    pub stream_id: bool,   // PERF_SAMPLE_STREAM_ID
    pub cpu: bool,         // PERF_SAMPLE_CPU
    pub period: bool,      // PERF_SAMPLE_PERIOD
    pub v: bool,           // PERF_SAMPLE_READ

    /// Wrap `sample_max_stack` with `Some` to enable this field
    pub ips: Option<u16>, // PERF_SAMPLE_CALLCHAIN

    pub data_raw: bool, // PERF_SAMPLE_RAW

    /// Wrap `sample_regs_user` with `Some` to enable this field
    pub abi_and_regs_user: Option<u64>, // PERF_SAMPLE_REGS_USER

    /// Wrap `sample_stack_user` with `Some` to enable this field
    pub data_stack_user: Option<u32>, // PERF_SAMPLE_STACK_USER

    pub data_src: bool,    // PERF_SAMPLE_DATA_SRC
    pub transaction: bool, // PERF_SAMPLE_TRANSACTION

    /// Wrap `sample_regs_intr` with `Some` to enable this field
    pub abi_and_regs_intr: Option<u64>, // PERF_SAMPLE_REGS_INTR

    pub phys_addr: bool,      // PERF_SAMPLE_PHYS_ADDR
    pub cgroup: bool,         // PERF_SAMPLE_CGROUP
    pub data_page_size: bool, // PERF_SAMPLE_DATA_PAGE_SIZE
    pub code_page_size: bool, // PERF_SAMPLE_CODE_PAGE_SIZE
}

impl SampleRecordFields {
    pub(crate) const fn as_sample_type(&self) -> u64 {
        macro_rules! gen {
            ($(
                $(@#[$attr: meta])*
                $cond: expr,
                $mask: expr
            )+) => {
                let mut sample_type = 0_u64;
                $(
                    $(#[$attr])*
                    {
                        if $cond {
                            sample_type |= $mask as u64;
                        }
                    }
                )+
                sample_type
            };
        }

        //| PERF_SAMPLE_BRANCH_STACK // TODO: Not all hardware supports this feature
        //| PERF_SAMPLE_WEIGHT // FIX: this will lead to "Invalid Argument"

        gen! {
            self.sample_id                  , PERF_SAMPLE_IDENTIFIER
            self.ip                         , PERF_SAMPLE_IP
            self.pid_and_tid                , PERF_SAMPLE_TID
            self.time                       , PERF_SAMPLE_TIME
            self.addr                       , PERF_SAMPLE_ADDR
            self.id                         , PERF_SAMPLE_ID
            self.stream_id                  , PERF_SAMPLE_STREAM_ID
            self.cpu                        , PERF_SAMPLE_CPU
            self.period                     , PERF_SAMPLE_PERIOD
            self.v                          , PERF_SAMPLE_READ
            self.ips.is_some()              , PERF_SAMPLE_CALLCHAIN
            self.data_raw                   , PERF_SAMPLE_RAW
            self.abi_and_regs_user.is_some(), PERF_SAMPLE_REGS_USER
            self.data_stack_user.is_some()  , PERF_SAMPLE_STACK_USER
            self.data_src                   , PERF_SAMPLE_DATA_SRC
            self.transaction                , PERF_SAMPLE_TRANSACTION
            self.abi_and_regs_intr.is_some(), PERF_SAMPLE_REGS_INTR
            self.phys_addr                  , PERF_SAMPLE_PHYS_ADDR
            @#[cfg(feature = "linux-5.7")]
            self.cgroup                     , PERF_SAMPLE_CGROUP
            @#[cfg(feature = "linux-5.11")]
            self.data_page_size             , PERF_SAMPLE_DATA_PAGE_SIZE
            @#[cfg(feature = "linux-5.11")]
            self.code_page_size             , PERF_SAMPLE_CODE_PAGE_SIZE
        }
    }
}

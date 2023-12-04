mod extra_config;

use crate::counting::attr::extra_config::ExtraConfig;
use crate::perf_event::RawAttr;
use crate::{Event, EventScope};
use std::fmt::Debug;

pub use extra_config::*;

#[derive(Debug)]
pub struct Attr {
    raw_attr: RawAttr,
}

impl Attr {
    // TODO: more options are needed
    pub fn new(
        event: impl Into<Event>,
        scopes: impl IntoIterator<Item = EventScope>,
        extra_config: ExtraConfig,
    ) -> Self {
        use crate::syscall::bindings::*;

        let mut raw_attr = RawAttr {
            type_: 0,
            size: std::mem::size_of::<RawAttr>() as libc::__u32,
            config: 0,
            __bindgen_anon_1: perf_event_attr__bindgen_ty_1::default(), // not use in counting mode
            sample_type: 0,                                             // ditto
            read_format: {
                #[allow(unused_mut)]
                #[allow(clippy::identity_op)] // for readable
                let mut read_format = 0
                    | perf_event_read_format_PERF_FORMAT_TOTAL_TIME_ENABLED
                    | perf_event_read_format_PERF_FORMAT_TOTAL_TIME_RUNNING
                    | perf_event_read_format_PERF_FORMAT_ID
                    | perf_event_read_format_PERF_FORMAT_GROUP;

                // WARN: set for read_format align, event_lost should not be used.
                #[cfg(feature = "kernel-6.0")]
                {
                    read_format |= perf_event_read_format_PERF_FORMAT_LOST;
                }

                read_format
            } as _,
            _bitfield_align_1: [],
            _bitfield_1: __BindgenBitfieldUnit::new([0u8; 8usize]), // set latter via raw_attr.set_...
            __bindgen_anon_2: perf_event_attr__bindgen_ty_2::default(), // not use in counting mode
            bp_type: 0,                                             // ditto
            __bindgen_anon_3: perf_event_attr__bindgen_ty_3::default(), // ditto
            __bindgen_anon_4: perf_event_attr__bindgen_ty_4::default(), // ditto
            branch_sample_type: 0,                                  // ditto
            sample_regs_user: 0,                                    // ditto
            sample_stack_user: 0,                                   // ditto
            clockid: 0,                                             // ditto
            sample_regs_intr: 0,                                    // ditto
            aux_watermark: 0,                                       // ditto
            sample_max_stack: 0,                                    // ditto
            __reserved_2: 0,
            #[cfg(feature = "kernel-5.5")]
            aux_sample_size: 0, // not use in counting mode
            __reserved_3: 0,
            #[cfg(feature = "kernel-5.13")]
            sig_data: 0, // not use in counting mode
            #[cfg(feature = "kernel-6.2")]
            config3: 0, // TODO: missing docs in manual
        };

        raw_attr.set_disabled(1);
        raw_attr.set_inherit(extra_config.inherit as _);
        raw_attr.set_pinned(extra_config.pinned as _);
        raw_attr.set_exclusive(extra_config.exclusive as _);

        raw_attr.set_exclude_user(1);
        raw_attr.set_exclude_kernel(1);
        raw_attr.set_exclude_hv(1);
        raw_attr.set_exclude_idle(1);

        raw_attr.set_mmap(0); // not use in counting mode
        raw_attr.set_comm(0); // ditto
        raw_attr.set_freq(0); // ditto
        raw_attr.set_inherit_stat(extra_config.inherit_stat as _);
        raw_attr.set_enable_on_exec(extra_config.enable_on_exec as _);
        raw_attr.set_task(0); // not use in counting mode
        raw_attr.set_watermark(0); // ditto
        raw_attr.set_precise_ip(0); // ditto
        raw_attr.set_mmap_data(0); // ditto
        raw_attr.set_sample_id_all(0); // ditto

        raw_attr.set_exclude_host(1);
        raw_attr.set_exclude_guest(1);
        raw_attr.set_exclude_callchain_kernel(1);
        raw_attr.set_exclude_callchain_user(1);

        raw_attr.set_mmap2(0); // not use in counting mode
        raw_attr.set_comm_exec(0); // ditto
        raw_attr.set_use_clockid(0); // ditto
        raw_attr.set_context_switch(0); // ditto
        raw_attr.set_write_backward(0); // ditto
        raw_attr.set_namespaces(0); // ditto
        raw_attr.set_ksymbol(0); // ditto
        raw_attr.set_bpf_event(0); // ditto
        #[cfg(feature = "kernel-5.4")]
        raw_attr.set_aux_output(0); // ditto
        #[cfg(feature = "kernel-5.7")]
        raw_attr.set_cgroup(0); // ditto
        #[cfg(feature = "kernel-5.8")]
        raw_attr.set_text_poke(0); // ditto
        #[cfg(feature = "kernel-5.12")]
        raw_attr.set_build_id(0); // ditto
        #[cfg(feature = "kernel-5.13")]
        raw_attr.set_inherit_thread(extra_config.inherit_thread as _);
        #[cfg(feature = "kernel-5.13")]
        raw_attr.set_remove_on_exec(extra_config.remove_on_exec as _);
        #[cfg(feature = "kernel-5.13")]
        raw_attr.set_sigtrap(0); // not use in counting mode

        use EventScope::*;
        scopes.into_iter().for_each(|scope| match scope {
            User => raw_attr.set_exclude_user(0),
            Kernel => raw_attr.set_exclude_kernel(0),
            Hv => raw_attr.set_exclude_hv(0),
            Idle => raw_attr.set_exclude_idle(0),
            Host => raw_attr.set_exclude_host(0),
            Guest => raw_attr.set_exclude_guest(0),
            CallchainKernel => raw_attr.set_exclude_callchain_kernel(0),
            CallchainUser => raw_attr.set_exclude_callchain_user(0),
        });

        match event.into() {
            Event::Hw(ev) if ev.is_cache_event() => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_HW_CACHE;
                raw_attr.config = ev.into_u64();
            }
            Event::Hw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_HARDWARE;
                raw_attr.config = ev.into_u64();
            }
            Event::Sw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_SOFTWARE;
                raw_attr.config = ev.into_u64();
            }
            Event::Raw(ev) => {
                raw_attr.type_ = perf_type_id_PERF_TYPE_RAW;
                raw_attr.config = ev.into_u64();
            }
        }

        Self { raw_attr }
    }

    /// Construct from a raw `perf_event_attr` struct.
    /// # Safety
    /// The `raw_attr` argument must be a properly initialized
    /// `perf_event_attr` struct for counting mode.
    pub const unsafe fn from_raw(raw_attr: RawAttr) -> Self {
        Self { raw_attr }
    }

    pub const fn into_raw(self) -> RawAttr {
        self.raw_attr
    }

    pub const fn as_raw(&self) -> &RawAttr {
        &self.raw_attr
    }
}

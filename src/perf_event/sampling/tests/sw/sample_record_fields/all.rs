use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraConfig, OverflowBy, SampleRecordFields};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages)
}

fn gen_attr(extra_config: ExtraConfig) -> Attr {
    let event = SwEvent::CpuClock;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Attr::new(event, scopes, overflow_by, &extra_config, [])
}

#[test]
fn test() {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields = SampleRecordFields {
        sample_id: true,
        ip: true,
        pid_and_tid: true,
        time: true,
        addr: true,
        id: true,
        stream_id: true,
        cpu: true,
        period: true,
        v: true,
        ips: Some(1),
        data_raw: true,
        abi_and_regs_user: Some(1),
        data_stack_user: Some(2_u32.pow(3)),
        data_src: true,
        transaction: true,
        abi_and_regs_intr: Some(1),
        phys_addr: true,
        cgroup: true,
        data_page_size: true,
        code_page_size: true,
    };
    let builder = gen_builder();
    let attr = gen_attr(extra_config);

    let sampling = builder.build_sampling(&attr).unwrap();
    sampling.enable().unwrap();
    cpu_workload();
    sampling.disable().unwrap();

    let mut sample_count = 0_usize;
    for Record { body, .. } in sampling {
        if let RecordBody::Sample(body) = body {
            assert!(body.sample_id.is_some());
            assert!(body.ip.is_some());
            assert!(body.pid.is_some());
            assert!(body.tid.is_some());
            assert!(body.time.is_some());
            assert!(body.addr.is_some());
            assert!(body.id.is_some());
            assert!(body.stream_id.is_some());
            assert!(body.cpu.is_some());
            assert!(body.period.is_some());
            assert!(body.v.is_some());
            assert!(body.ips.is_some());
            assert!(body.data_raw.is_some());
            assert!(body.abi_and_regs_user.is_some());
            assert!(body.data_stack_user.is_some());
            assert!(body.data_src.is_some());
            assert!(body.transaction.is_some());
            assert!(body.abi_and_regs_intr.is_some());
            assert!(body.phys_addr.is_some());
            assert!(body.cgroup.is_some());
            assert!(body.data_page_size.is_some());
            assert!(body.code_page_size.is_some());
            sample_count += 1;
        }
    }
    assert!(sample_count > 0);
}
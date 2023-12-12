use crate::sampling::record::{Record, RecordBody};
use crate::sampling::{Attr, ExtraConfig, OverflowBy};
use crate::test::cpu_workload;
use crate::{Builder, EventScope, SwEvent};

fn gen_builder() -> Builder {
    let mmap_pages = 1 + 512;
    Builder::new()
        .calling_process()
        .any_cpu()
        .mmap_pages(mmap_pages)
}

fn gen_attr(sample_stack_user: u32) -> Attr {
    let mut extra_config = ExtraConfig::default();
    extra_config.sample_record_fields.time = true;
    extra_config.sample_record_fields.data_stack_user = Some(sample_stack_user);

    let event = SwEvent::CpuClock;
    let scopes = [EventScope::User, EventScope::Host];
    let overflow_by = OverflowBy::Period(1000);
    Attr::new(event, scopes, overflow_by, &extra_config, [])
}

#[test]
fn test() {
    let builder = gen_builder();
    for i in 3..8 {
        let attr = gen_attr(2_u32.pow(i));
        let sampling = builder.build_sampling(&attr).unwrap();

        sampling.enable().unwrap();
        cpu_workload();
        sampling.disable().unwrap();

        let mut sample_count = 0_usize;
        let mut last_time = 0;
        for Record { body, .. } in sampling {
            if let RecordBody::Sample(sample) = body {
                assert!(sample.time.unwrap() >= last_time);
                last_time = sample.time.unwrap();
                sample_count += 1;
            }
        }
        assert!(sample_count > 0);
    }
}
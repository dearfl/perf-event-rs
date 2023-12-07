mod raw;

use crate::sampling::record::SampleId;
use std::ffi::CString;

#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub comm: CString,
    pub sample_id: Option<SampleId>,
}

type RawBody = raw::Body;

impl Body {
    pub unsafe fn from_ptr(ptr: *const u8, sample_id_all: bool) -> Self {
        let raw = (ptr as *const RawBody).as_ref().unwrap();

        Self {
            pid: *raw.pid(),
            tid: *raw.tid(),
            comm: CString::from_vec_unchecked(raw.comm().to_vec()),
            sample_id: sample_id_all.then(|| raw.sample_id().clone()),
        }
    }
}

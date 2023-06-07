use std::collections::HashMap;
use std::ffi::{c_void, CStr, CString};
use std::{mem, slice};
use std::alloc::{alloc, Layout};
use rust_kindling_test::adhesive::start;


fn main() {
    println!("Hello, world!");
    start()
}


// fn convert_event(cgo_event: &kindling::KindlingEventForGo) -> event::KindlingEvent {
//     let event = event::KindlingEvent {
//         source: Default::default(),
//         timestamp: cgo_event.timestamp,
//         name: unsafe { CStr::from_ptr(cgo_event.name).to_string_lossy().into_owned() },
//         category: Default::default(),
//         params_number: cgo_event.paramsNumber,
//         latency: cgo_event.latency,
//         ctx: event::Context {
//             thread_info: Option::from(event::Thread {
//                 pid: cgo_event.context.tinfo.pid,
//                 tid: cgo_event.context.tinfo.tid,
//                 uid: cgo_event.context.tinfo.uid,
//                 gid: cgo_event.context.tinfo.gid,
//                 comm: unsafe { CStr::from_ptr(cgo_event.context.tinfo.comm).to_string_lossy().into_owned() },
//                 container_id: unsafe { CStr::from_ptr(cgo_event.context.tinfo.containerId).to_string_lossy().into_owned() },
//             }),
//         fd_info: Option::from(event::Fd {
//             num: cgo_event.context.fdInfo.num,
//             filename: unsafe { CStr::from_ptr(cgo_event.context.fdInfo.filename).to_string_lossy().into_owned() },
//             directory: unsafe { CStr::from_ptr(cgo_event.context.fdInfo.directory).to_string_lossy().into_owned() },
//             role: false,
//             sport: cgo_event.context.fdInfo.sport,
//             dport: cgo_event.context.fdInfo.dport,
//             source: cgo_event.context.fdInfo.source,
//             destination: cgo_event.context.fdInfo.destination,
//         }),
//         },
//     };
//     event
// }
use std::collections::HashMap;
use std::ffi::{c_void, CStr, CString};
use std::{fmt, slice};
use std::sync::{Arc, Mutex};
use crate::cpuAnalyzer::{consume_cpu_event, consume_java_futex_event, CpuAnalyzer};
use crate::probeToRust::kindling_event::{catchSignalUp, event_params_for_subscribe, getCaptureStatistics, getEventsByInterval, initKindlingEventForGo, KindlingEventForGo, runForGo, startProfile, stopProfile, subEventForGo, SubEvent};

pub fn sub_event() {
    let subscribe_info = vec![
        SubEvent {
            Category: "".to_string(),
            Name: "tracepoint-cpu_analysis".to_string(),
            Params: Default::default(),
        },
    ];

    if subscribe_info.is_empty() {
        println!("No events are subscribed by cgo receiver. Please check your configuration.");
    } else {
        println!("The subscribed events are: {:?}", subscribe_info);
    }

    for value in subscribe_info {
        //to do. analyze params filed in the value
        let params_list = vec![
            event_params_for_subscribe {
                name: CString::new("terminator").expect("CString::new failed").into_raw(),
                value: CString::new("").expect("CString::new failed").into_raw()
            },
        ];

        let name = CString::new(value.Name.clone()).unwrap().into_raw();
        let category = CString::new(value.Category.clone()).unwrap().into_raw();
        let params = params_list.as_ptr() as *mut c_void;

        unsafe {
            subEventForGo(name, category, params);
            drop(CString::from_raw(name));
            drop(CString::from_raw(category));
        }
    }

}

pub fn getKindlingEvents(ca: &Arc<Mutex<CpuAnalyzer>>) {
    let mut count = 0;

    const KEY_VALUE_ARRAY_SIZE: usize = 16;

    let mut npKindlingEvent: Vec<KindlingEventForGo> = vec![KindlingEventForGo::default(); 1000];
    let npKindlingEventPtr: *mut KindlingEventForGo = npKindlingEvent.as_mut_slice().as_mut_ptr();
    let npKindlingEventVoidPtr: *mut std::ffi::c_void = npKindlingEventPtr as *mut std::ffi::c_void;

    // let mut npKindlingEvent: Vec<kindling::KindlingEventForGo> = vec![kindling::KindlingEventForGo::default(); 1000];
    // npKindlingEvent = npKindlingEvent.as_mut_ptr()as *mut std::ffi::c_void;
    unsafe {
        initKindlingEventForGo(1000, npKindlingEventVoidPtr);
    }

    loop {
        let res = unsafe { getEventsByInterval(100000000,npKindlingEventVoidPtr , &mut count as *mut _ as *mut libc::c_void) };
        if res == 0 {
            let events = unsafe {
                slice::from_raw_parts(npKindlingEvent.as_ptr(), count as usize)
            };            for i in 0..count {
                let event = &events[i];
                // let converted_event = convert_event(event);
                let ev_name = unsafe { CStr::from_ptr(event.name) };
                let ev_name_string = ev_name.to_str().expect("Invalid UTF-8");
                //println!("{:?}", event);
                match ev_name_string {
                    "cpu_analysis" => {
                        //println!("{:?}", event);
                        consume_cpu_event(event, ca)
                    },
                    "java_futex_info" => {
                        // 处理 pattern2 的逻辑
                        consume_java_futex_event(event, ca)
                    }
                    _ => {
                        // 默认情况，处理其他所有情况的逻辑
                    }
                }
            }
        }
        count = 0;
    }
}


pub fn start_profile() {
    if unsafe { startProfile() } == 0 {
        println!("start profile success!");
    }
}

pub fn stop_profile() {
    if unsafe { stopProfile() } == 0 {
        println!("stop profile success!");
    }
}

pub fn get_capture_statistics() {
    unsafe {
        getCaptureStatistics();
    }
}

pub fn catch_signal_up() {
    unsafe {
        catchSignalUp();
    }
}
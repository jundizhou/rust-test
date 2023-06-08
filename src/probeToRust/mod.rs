use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::probeToRust::kindling_event::{runForGo, startProfile};
use crate::probeToRust::rust_receiver::{sub_event, getKindlingEvents, get_capture_statistics, catch_signal_up};

mod kindling_event;
mod rust_receiver;

pub use kindling_event::KindlingEventForGo;
use crate::cpuAnalyzer::{CpuAnalyzer, print_all_event};


pub fn startProbeToRust() {
    // 初始化probe
    unsafe { runForGo() };
    unsafe { startProfile() };

    // 订阅事件
    sub_event();

    // 初始化on-off cpu分析器
    let cpu_analyzer = Arc::new(Mutex::new(CpuAnalyzer {
        cpu_pid_events: HashMap::new(),
    }));

    // 启动内核事件统计
    thread::spawn(move || {
        get_capture_statistics();
    });

    // 启动异常退出打印gdb日志
    thread::spawn(move || {
        catch_signal_up();
    });

    // 开始获取事件
    let cpu_analyzer_clone = Arc::clone(&cpu_analyzer);
    getKindlingEvents(&cpu_analyzer_clone);
}
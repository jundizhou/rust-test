use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::probeToRust::kindlingEvent::{runForGo, startProfile};
use crate::probeToRust::rustReceiver::{subEvent, getKindlingEvents};

mod event;
mod kindlingEvent;
mod rustReceiver;

pub use kindlingEvent::KindlingEventForGo;
use crate::cpuAnalyzer::{CpuAnalyzer, print_all_event};


pub fn startProbeToRust() {
    let mut sub_event = event::SubEvent {
        Category: "".to_string(),
        Name: "".to_string(),
        Params: Default::default(),
    };
    unsafe { runForGo() };
    unsafe { startProfile() };
    subEvent();

    let cpu_analyzer = Arc::new(Mutex::new(CpuAnalyzer {
        cpu_pid_events: HashMap::new(),
    }));

    // 启动新线程执行print_cpu_pid_events函数
    let cpu_analyzer_clone_print = Arc::clone(&cpu_analyzer);
    thread::spawn(move || {
        let mut start_time = Instant::now(); // 记录起始时间
        let interval = Duration::from_secs(10); // 指定间隔为10秒

        loop {
            // 等待间隔时间
            thread::sleep(interval);

            // 检查是否已经过了指定的间隔时间
            if start_time.elapsed() >= interval {
                // 执行要定期执行的代码
                print_all_event(&cpu_analyzer_clone_print);

                // 重置起始时间
                start_time = Instant::now();
            }
        }
    });

    // 启动新线程执行getKindlingEvents函数
    let cpu_analyzer_clone = Arc::clone(&cpu_analyzer);
    // thread::spawn(move || {
    //     getKindlingEvents(&cpu_analyzer_clone);
    // });
    getKindlingEvents(&cpu_analyzer_clone);
}
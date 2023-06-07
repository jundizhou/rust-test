use std::fmt;
use std::fmt::{Debug, Formatter};
use serde_derive::Serialize;
use serde_derive::Deserialize;
use crate::cpuAnalyzer::circle_queue::CircleQueue;
use crate::cpuAnalyzer::time_event::TimedEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuEvent {
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
    #[serde(rename = "typeSpecs")]
    pub type_specs: Vec<u64>,
    #[serde(rename = "runqLatency")]
    pub runq_latency: Vec<u64>,
    #[serde(rename = "timeType")]
    pub time_type: Vec<u8>,
    #[serde(rename = "onInfo")]
    pub on_info: String,
    #[serde(rename = "offInfo")]
    pub off_info: String,
    pub log: String,
    pub stack: String,
}

impl Default for CpuEvent {
    fn default() -> Self {
        CpuEvent {
            start_time: 0,
            end_time: 0,
            type_specs: Vec::new(),
            runq_latency: Vec::new(),
            time_type: Vec::new(),
            on_info: String::new(),
            off_info: String::new(),
            log: String::new(),
            stack: String::new(),
        }
    }
}

impl fmt::Display for CpuEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 根据需要定义打印格式
        write!(f, "CpuEvent: start_time={}, end_time={}, type_specs={:?}, runq_latency={:?}, time_type={:?}, on_info={}, off_info={}, log={}, stack={}",
               self.start_time, self.end_time, self.type_specs,
               self.runq_latency, self.time_type, self.on_info, self.off_info, self.log, self.stack)
    }
}

impl TimedEvent for CpuEvent {
    fn start_timestamp(&self) -> u64 {
        self.start_time
    }

    fn end_timestamp(&self) -> u64 {
        self.end_time
    }

    fn kind(&self) -> i32 {
        return 0;
    }
}

#[derive(Debug)]
pub struct TimeSegments {
    pub pid: u32,
    pub tid: u32,
    pub thread_name: String,
    pub base_time: u64,
    pub segments: CircleQueue,
}

impl TimeSegments {
    fn new(pid: u32, tid: u32, thread_name: String, base_time: u64, segments: CircleQueue) -> Self {
        TimeSegments {
            pid,
            tid,
            thread_name,
            base_time,
            segments,
        }
    }
    pub fn update_thread_name(&mut self, thread_name: String) {
        self.thread_name = thread_name;
    }
}

#[derive(Default, Debug)]
pub struct Segment {
    start_time: u64,
    end_time: u64,
    cpu_events: Vec<CpuEvent>,
    pub is_send: i32,
    index_timestamp: String,
}

impl Clone for Segment {
    fn clone(&self) -> Self {
        Segment {
            start_time: self.start_time,
            end_time: self.end_time,
            cpu_events: self.cpu_events.iter().cloned().collect(),
            is_send: self.is_send,
            index_timestamp: self.index_timestamp.clone(),
        }
    }
}

impl Segment {
    pub fn new(start_time: u64, end_time: u64) -> Self {
        Segment {
            start_time,
            end_time,
            cpu_events: Vec::new(),
            is_send: 0,
            index_timestamp: String::new(),
        }
    }
    pub fn put_timed_event(&mut self, event : CpuEvent) {
        self.cpu_events.push(event);

    }
}




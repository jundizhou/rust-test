use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use crate::cpuAnalyzer::cpu_event::{CpuEvent, TimeSegments};
use crate::probeToRust::KindlingEventForGo;
use crate::cpuAnalyzer::circle_queue::CircleQueue;
use crate::cpuAnalyzer::time_event::TimedEvent;
use crate::cpuAnalyzer::cpu_event::Segment;

const NANO_TO_SECONDS: u64 = 1_000_000_000;
const MAX_SEGMENT_SIZE: usize = 40;

pub struct CpuAnalyzer {
    pub cpu_pid_events: HashMap<u32, HashMap<u32, TimeSegments>>,
    pub lock: Mutex<()>,
}

pub fn print_all_event(cca: &Arc<Mutex<CpuAnalyzer>>) {
    let ca_guard = cca.lock().unwrap();
    ca_guard.print_cpu_pid_events();
}

pub fn consume_cpu_event(event: &KindlingEventForGo, cca: &Arc<Mutex<CpuAnalyzer>>) {
    let mut ev = CpuEvent::default();
    for i in 0..event.paramsNumber as usize {
        let user_attributes = event.userAttributes[i];
        match user_attributes.get_key() {
            Some("start_time") => ev.start_time = user_attributes.get_uint_value(),
            Some("end_time") => ev.end_time = user_attributes.get_uint_value(),
            Some("time_specs") => {
                let val = user_attributes.get_value().unwrap();
                ev.type_specs = read_u64_values(val);
            }
            Some("runq_latency") => {
                let val = user_attributes.get_value().unwrap();
                ev.runq_latency = read_u64_values(val);
            }
            Some("time_type") => {
                let val = user_attributes.get_value().unwrap();
                ev.time_type = read_u8_values(val);
            }
            Some("on_info") => ev.on_info = read_string_value(user_attributes.get_value()),
            Some("off_info") => ev.off_info = read_string_value(user_attributes.get_value()),
            Some("log") => ev.log = read_string_value(user_attributes.get_value()),
            Some("stack") => ev.stack = read_string_value(user_attributes.get_value()),
            _ => (),
        }
    }

    if ev.start_time < 1600000000000000000 {
        return;
    }

    println!("{}", ev);

    let mut ca_guard = cca.lock().unwrap();
    ca_guard.put_event_to_segments(
        event.get_pid(),
        event.get_tid(),
        event.get_comm(),
        ev,
    );
}

impl CpuAnalyzer {
    pub fn new() -> Self {
        CpuAnalyzer {
            cpu_pid_events: HashMap::new(),
            lock: Mutex::new(()),
        }
    }

    pub fn put_event_to_segments(&mut self, pid: u32, tid: u32, thread_name: String, event: CpuEvent) {
        let _lock = self.lock.lock().unwrap();

        let tid_cpu_events = self.cpu_pid_events.entry(pid).or_insert_with(HashMap::new);
        let time_segments = tid_cpu_events.entry(tid).or_insert_with(|| {
            let base_time = event.start_timestamp() / NANO_TO_SECONDS;
            let segments = create_initial_segments(base_time);
            TimeSegments {
                pid,
                tid,
                thread_name,
                base_time,
                segments,
            }
        });

        let end_offset = (event.end_timestamp() / NANO_TO_SECONDS - time_segments.base_time) as i32;
        if end_offset < 0 {
            return;
        }

        let start_offset = (event.start_timestamp() / NANO_TO_SECONDS - time_segments.base_time) as i32;
        let should_clear_segments = start_offset >= MAX_SEGMENT_SIZE as i32 || end_offset > MAX_SEGMENT_SIZE as i32;

        if should_clear_segments {
            if start_offset * 2 >= 3 * MAX_SEGMENT_SIZE as i32 {
                time_segments.segments.clear();
                time_segments.base_time = event.start_timestamp() / NANO_TO_SECONDS;
                let end_offset = end_offset - start_offset;
                let start_offset = 0;
                time_segments.segments = create_initial_segments(time_segments.base_time);
            } else {
                let clear_size = MAX_SEGMENT_SIZE / 2;
                time_segments.base_time += clear_size as u64;
                let mut start_offset = start_offset - clear_size as i32;
                let end_offset = end_offset - clear_size as i32;
                if start_offset < 0 {
                    start_offset = 0;
                }
                for i in 0..clear_size {
                    let moved_index = i + clear_size;
                    if let Some(segment) = time_segments.segments.get_by_index(moved_index) {
                        let mut cloned_segment = segment.clone();
                        cloned_segment.put_timed_event(event.clone());
                        cloned_segment.is_send = 0;
                        time_segments.segments.update_by_index(i, cloned_segment);
                    }
                    let segment_tmp = Segment::new(
                        (time_segments.base_time + (moved_index as u64)) * NANO_TO_SECONDS,
                        (time_segments.base_time + ((moved_index + 1) as u64)) * NANO_TO_SECONDS,
                    );
                    time_segments.segments.update_by_index(moved_index, segment_tmp);
                }
            }
        }

        for i in start_offset..=end_offset.min(MAX_SEGMENT_SIZE as i32 - 1) {
            if let Some(segment) = time_segments.segments.get_by_index(i as usize) {
                let mut cloned_segment = segment.clone();
                cloned_segment.put_timed_event(event.clone());
                cloned_segment.is_send = 0;
                time_segments.segments.update_by_index(i as usize, cloned_segment);
            }
        }
    }

    pub fn print_cpu_pid_events(&self) {
        println!("{:?}", self.cpu_pid_events);
    }
}

fn read_u64_values(val: &[u8]) -> Vec<u64> {
    let mut cursor = Cursor::new(val);
    let count = val.len() / 8;
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(cursor.read_u64::<LittleEndian>().unwrap());
    }
    values
}

fn read_u8_values(val: &[u8]) -> Vec<u8> {
    val.to_vec()
}

fn read_string_value(val: Option<&[u8]>) -> String {
    String::from_utf8_lossy(val.unwrap_or_default()).to_string()
}

fn create_initial_segments(base_time: u64) -> CircleQueue{
    let mut segments = CircleQueue::new(MAX_SEGMENT_SIZE);
    for i in 0..MAX_SEGMENT_SIZE {
        let segment = Segment::new(
            (base_time + (i as u64)) * NANO_TO_SECONDS,
            (base_time + (i as u64) + 1) * NANO_TO_SECONDS,
        );
        segments.update_by_index(i, segment);
    }
    segments
}
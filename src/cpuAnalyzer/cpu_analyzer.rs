use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::cpuAnalyzer::cpu_event::{CpuEvent, TimeSegments};
use crate::probeToRust::KindlingEventForGo;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use crate::cpuAnalyzer::circle_queue::CircleQueue;
use crate::cpuAnalyzer::time_event::TimedEvent;
use crate::cpuAnalyzer::cpu_event::Segment;

const NANO_TO_SECONDS: u64 = 1_000_000_000;
const MAX_SEGMENT_SIZE: usize = 40;


pub struct CpuAnalyzer  {
    pub cpu_pid_events: HashMap<u32, HashMap<u32, TimeSegments>>,
    pub lock: Mutex<()>, // Assuming you have a mutex for synchronization
    // cfg: Config,     // Assuming you have a Config struct defined
}

pub fn print_all_event(cca: &Arc<Mutex<CpuAnalyzer>>){
    let mut ca_guard = cca.lock().unwrap();
    ca_guard.print_cpu_pid_events();
}

pub fn consume_cpu_event(event: &KindlingEventForGo, cca: &Arc<Mutex<CpuAnalyzer>>) {
    let mut ev = CpuEvent::default();
    for i in 0..event.paramsNumber as usize {
        let user_attributes = event.userAttributes[i];
        //println!("{:?}", user_attributes.get_key());
        match user_attributes.get_key() {
            Some("start_time") => ev.start_time = user_attributes.get_uint_value(),
            Some("end_time") => ev.end_time = user_attributes.get_uint_value(),
            Some("time_specs") =>{
                let val = user_attributes.get_value().unwrap(); // Assuming val is always present
                let mut cursor = Cursor::new(&val);
                let type_specs_len = val.len() / 8;
                let mut type_specs = vec![0u64; type_specs_len];

                for i in 0..type_specs_len {
                    type_specs[i] = cursor.read_u64::<LittleEndian>().unwrap();
                }

                ev.type_specs = type_specs;
            }
            Some("runq_latency") => {
                let val = user_attributes.get_value().unwrap(); // Assuming val is always present
                let mut cursor = Cursor::new(&val);
                let runq_latency_len = val.len() / 8;
                let mut runq_latency = vec![0u64; runq_latency_len];

                for i in 0..runq_latency_len {
                    runq_latency[i] = cursor.read_u64::<LittleEndian>().unwrap();
                }

                ev.runq_latency = runq_latency;
            },
            Some("time_type") => {
                let val = user_attributes.get_value().unwrap(); // Assuming val is always present
                let mut cursor = Cursor::new(&val);
                let time_type_len = val.len();
                let mut time_type = Vec::with_capacity(time_type_len);
                for _ in 0..time_type_len {
                    let byte = cursor.read_u8().unwrap();
                    time_type.push(byte);
                }

                ev.time_type = time_type;
            },
            Some("on_info") => ev.on_info = String::from_utf8_lossy(user_attributes.get_value().unwrap_or_default()).to_string(),
            Some("off_info") => ev.off_info = String::from_utf8_lossy(user_attributes.get_value().unwrap_or_default()).to_string(),
            Some("log") => ev.log = String::from_utf8_lossy(user_attributes.get_value().unwrap_or_default()).to_string(),
            Some("stack") => ev.stack = String::from_utf8_lossy(user_attributes.get_value().unwrap_or_default()).to_string(),
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
    pub fn put_event_to_segments(&mut self, pid: u32, tid: u32, thread_name: String, event: CpuEvent) {
        let _lock = self.lock.lock().unwrap(); // Acquire the lock

        let tid_cpu_events = self.cpu_pid_events.entry(pid).or_insert_with(HashMap::new);
        let time_segments = tid_cpu_events.entry(tid).or_insert_with(|| {
            let base_time = event.start_timestamp() / NANO_TO_SECONDS;
            let mut segments = CircleQueue::new(MAX_SEGMENT_SIZE);

            for i in 0..MAX_SEGMENT_SIZE {
                let segment = Segment::new(
                    (base_time + (i as u64)) * NANO_TO_SECONDS,
                    (base_time + (i as u64) + 1) * NANO_TO_SECONDS,
                );
                segments.update_by_index(i, segment);

            }

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

        let clear_size = MAX_SEGMENT_SIZE / 2;
        let should_clear_segments = start_offset >= MAX_SEGMENT_SIZE as i32 || end_offset > MAX_SEGMENT_SIZE as i32;

        if should_clear_segments {
            if start_offset * 2 >= 3 * MAX_SEGMENT_SIZE as i32 {
                // Clear all elements

                time_segments.segments.clear();
                time_segments.base_time = event.start_timestamp() / NANO_TO_SECONDS;
                let end_offset = end_offset - start_offset;
                let start_offset = 0;

                for i in 0..MAX_SEGMENT_SIZE {
                    let segment = Segment::new(
                        (time_segments.base_time + (i as u64)) * NANO_TO_SECONDS,
                        (time_segments.base_time + ((i + 1) as u64)) * NANO_TO_SECONDS,
                    );
                    time_segments.segments.update_by_index(i, segment);
                }
            } else {
                // Clear half of the elements
                let clear_size = MAX_SEGMENT_SIZE / 2;

                time_segments.base_time += clear_size as u64;
                let mut start_offset = start_offset - clear_size as i32;
                let end_offset = end_offset - clear_size as i32;

                if start_offset < 0 {
                    start_offset = 0;
                }

                for i in 0..clear_size {
                    let moved_index = i + clear_size;
                    let val = time_segments.segments.get_by_index(moved_index);
                    if let Some(segment) = val {
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

        // Update the thread name immediately
        //time_segments.update_thread_name(thread_name);

        for i in start_offset..=end_offset.min(MAX_SEGMENT_SIZE as i32 - 1) {
            if let Some(segment) = time_segments.segments.get_by_index(i as usize) {
                let mut cloned_segment = segment.clone();
                cloned_segment.put_timed_event(event.clone());
                cloned_segment.is_send = 0;
                time_segments.segments.update_by_index(i as usize, cloned_segment);
            }
        }
    }

    pub fn print_cpu_pid_events(&mut self){
        println!("{:?}", self.cpu_pid_events);
    }

}
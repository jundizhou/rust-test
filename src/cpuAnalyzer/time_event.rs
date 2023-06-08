use std::any::Any;

pub trait TimedEvent {
    fn start_timestamp(&self) -> u64;
    fn end_timestamp(&self) -> u64;
    fn kind(&self) -> i32;
    fn as_any(&self) -> &dyn Any;
}

// use crate::cpuAnalyzer::model::{CpuEvent, JavaFutexEvent};
//
// pub(crate) enum TimedEvent {
//     Cpu(CpuEvent),
//     JavaFutex(JavaFutexEvent),
//
// }
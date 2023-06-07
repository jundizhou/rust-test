pub trait TimedEvent {
    fn start_timestamp(&self) -> u64;
    fn end_timestamp(&self) -> u64;
    fn kind(&self) -> i32;
}
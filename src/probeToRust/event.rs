use std::collections::HashMap;

#[derive(Debug)]
pub enum Source {
    SourceUnknown = 0,
    SyscallEnter = 1,
    SyscallExit = 2,
    Tracepoint = 3,
    Kprobe = 4,
    Kretprobe = 5,
    Uprobe = 6,
    Uretprobe = 7,
}

impl Default for Source {
    fn default() -> Self {
        Source::SourceUnknown
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Source::SourceUnknown => "SOURCE_UNKNOWN",
            Source::SyscallEnter => "SYSCALL_ENTER",
            Source::SyscallExit => "SYSCALL_EXIT",
            Source::Tracepoint => "TRACEPOINT",
            Source::Kprobe => "KRPOBE",
            Source::Kretprobe => "KRETPROBE",
            Source::Uprobe => "UPROBE",
            Source::Uretprobe => "URETPROBE",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub enum Category {
    CatNone = 0,
    CatOther = 1,
    CatFile = 2,
    CatNet = 3,
    CatIpc = 4,
    CatWait = 5,
    CatSignal = 6,
    CatSleep = 7,
    CatTime = 8,
    CatProcess = 9,
    CatScheduler = 10,
    CatMemory = 11,
    CatUser = 12,
    CatSystem = 13,
}

impl Default for Category {
    fn default() -> Self {
        Category::CatNone
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Category::CatNone => "CAT_NONE",
            Category::CatOther => "CAT_OTHER",
            Category::CatFile => "CAT_FILE",
            Category::CatNet => "CAT_NET",
            Category::CatIpc => "CAT_IPC",
            Category::CatWait => "CAT_WAIT",
            Category::CatSignal => "CAT_SIGNAL",
            Category::CatSleep => "CAT_SLEEP",
            Category::CatTime => "CAT_TIME",
            Category::CatProcess => "CAT_PROCESS",
            Category::CatScheduler => "CAT_SCHEDULER",
            Category::CatMemory => "CAT_MEMORY",
            Category::CatUser => "CAT_USER",
            Category::CatSystem => "CAT_SYSTEM",
        };
        write!(f, "{}", name)
    }
}




pub struct KeyValue {
    pub key: String,
    pub value_type: u32,
    pub value: Vec<u8>,
}


#[derive(Debug)]
pub struct Context {
    pub thread_info: Option<Thread>,
    pub fd_info: Option<Fd>,
}

#[derive(Debug)]
pub struct Thread {
    pub pid: u32,
    pub tid: u32,
    pub uid: u32,
    pub gid: u32,
    pub comm: String,
    pub container_id: String,
}
#[derive(Debug)]
pub struct Fd {
    pub num: i32,
    pub filename: String,
    pub directory: String,
    pub role: bool,
    pub sport: u32,
    pub dport: u32,
    pub source: u64,
    pub destination: u64,
}

#[derive(Debug)]
pub struct KindlingEvent {
    pub source: Source,
    pub timestamp: u64,
    pub name: String,
    pub category: Category,
    pub params_number: u16,
    pub latency: u64,
    pub ctx: Context,
}


#[derive(Debug)]
pub(crate) struct SubEvent {
    pub(crate) Category: String,
    pub(crate) Name: String,
    pub(crate) Params: HashMap<String, String>,
}

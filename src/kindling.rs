use libc::c_char;

#[repr(C)]
#[derive(Copy)]
pub struct KeyValue {
    key: *mut libc::c_char,
    value: *mut libc::c_char,
    len: u32,
    valueType: u32,
}
impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key: [0i8; 1].as_mut_ptr(),
            value: [0i8; 118192].as_mut_ptr(),
            len: 0,
            valueType: 0,
        }
    }
}

impl Clone for KeyValue {
    fn clone(&self) -> Self {
        // 这里根据字段类型的特性进行适当的克隆操作
        // 例如，如果字段是原始类型，可以直接进行拷贝
        // 如果字段是指针类型，需要根据实际情况进行内存拷贝或其他操作
        // 注意要确保正确地管理内存生命周期和所有权

        // 示例实现：
        Self {
            key: self.key,
            value: self.value,
            len: self.len,
            valueType: self.valueType,
        }
    }
}


#[repr(C)]
pub struct KindlingEventForGo {
    pub(crate) timestamp: u64,
    pub(crate) name: *mut libc::c_char,
    pub(crate) category: u32,
    pub(crate) paramsNumber: u16,
    pub(crate) latency: u64,
    pub(crate) userAttributes: [KeyValue; 16],
    pub(crate) context: EventContext,
}

impl Default for KindlingEventForGo {
    fn default() -> Self {
        KindlingEventForGo {
            timestamp: 0,
            name: [0i8; 256].as_mut_ptr(),
            category: 0,
            paramsNumber: 0,
            latency: 0,
            userAttributes: [KeyValue::default(); 16],
            context: EventContext::default(),
        }
    }
}

impl Clone for KindlingEventForGo {
    fn clone(&self) -> Self {
        KindlingEventForGo {
            timestamp: self.timestamp,
            name: self.name, // 注意：这里只是复制指针，并没有复制底层的数据
            category: self.category,
            paramsNumber: self.paramsNumber,
            latency: self.latency,
            userAttributes: self.userAttributes.clone(),
            context: self.context.clone(),
        }
    }
}


impl Default for EventContext {
    fn default() -> Self {
        EventContext {
            tinfo: ThreadInfo::default(),
            fdInfo: FdInfo::default(),
        }
    }
}

impl Clone for EventContext {
    fn clone(&self) -> Self {
        EventContext {
            tinfo: self.tinfo.clone(),
            fdInfo: self.fdInfo.clone(),
        }
    }
}

impl Clone for ThreadInfo {
    fn clone(&self) -> Self {
        ThreadInfo {
            pid: self.pid,
            tid: self.tid,
            uid: self.uid,
            gid: self.gid,
            comm: self.comm, // 注意：这里只是复制指针，并没有复制底层的数据
            containerId: self.containerId, // 注意：这里只是复制指针，并没有复制底层的数据
        }
    }
}

impl Clone for FdInfo {
    fn clone(&self) -> Self {
        FdInfo {
            num: self.num,
            fdType: self.fdType,
            filename: self.filename, // 注意：这里只是复制指针，并没有复制底层的数据
            directory: self.directory, // 注意：这里只是复制指针，并没有复制底层的数据
            protocol: self.protocol,
            role: self.role,
            sip: self.sip,
            dip: self.dip,
            sport: self.sport,
            dport: self.dport,
            source: self.source,
            destination: self.destination,
        }
    }
}

impl Default for ThreadInfo {
    fn default() -> Self {
        ThreadInfo {
            pid: 0,
            tid: 0,
            uid: 0,
            gid: 0,
            comm: [0i8; 256].as_mut_ptr(),
            containerId: [0i8; 256].as_mut_ptr(),
        }
    }
}

impl Default for FdInfo {
    fn default() -> Self {
        FdInfo {
            num: 0,
            fdType: 0,
            filename: [0i8; 256].as_mut_ptr(),
            directory: [0i8; 256].as_mut_ptr(),
            protocol: 0,
            role: 0,
            sip: [0; 4],
            dip: [0; 4],
            sport: 0,
            dport: 0,
            source: 0,
            destination: 0,
        }
    }
}

#[repr(C)]
pub struct EventContext {
    pub(crate) tinfo: ThreadInfo,
    pub(crate) fdInfo: FdInfo,
}

#[repr(C)]
pub struct ThreadInfo {
    pub(crate) pid: u32,
    pub(crate) tid: u32,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) comm: *mut libc::c_char,
    pub(crate) containerId: *mut libc::c_char,
}

#[repr(C)]
pub struct FdInfo {
    pub(crate) num: i32,
    pub(crate) fdType: u32,
    pub(crate) filename: *mut libc::c_char,
    pub(crate) directory: *mut libc::c_char,
    pub(crate) protocol: u32,
    pub(crate) role: u8,
    pub(crate) sip: [u32; 4],
    pub(crate) dip: [u32; 4],
    pub(crate) sport: u32,
    pub(crate) dport: u32,
    pub(crate) source: u64,
    pub(crate) destination: u64,
}

#[link(name = "kindling")]
extern "C" {
    pub fn runForGo() -> i32;
    pub fn getKindlingEvent(kindlingEvent: *mut *mut KindlingEventForGo) -> i32;
    pub fn suppressEventsCommForGo(comm: *mut libc::c_char);
    pub fn subEventForGo(eventName: *mut libc::c_char, category: *mut libc::c_char, params: *mut libc::c_void);
    pub fn initKindlingEventForGo(number: i32, kindlingEvent: *mut libc::c_void) -> i32;
    pub fn getEventsByInterval(interval: i32, kindlingEvent: *mut libc::c_void, count: *mut libc::c_void) -> i32;
    pub fn startProfile() -> i32;
    pub fn stopProfile() -> i32;
    pub fn startAttachAgent(pid: i32) -> *mut libc::c_char;
    pub fn stopAttachAgent(pid: i32) -> *mut libc::c_char;
    pub fn startProfileDebug(pid: i32, tid: i32);
    pub fn stopProfileDebug();
    pub fn getCaptureStatistics();
    pub fn catchSignalUp();
}

#[repr(C)]
pub struct event_params_for_subscribe {
    pub name: *mut c_char,
    pub value: *mut c_char,
}
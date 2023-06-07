use std::ffi::CStr;
use libc::c_char;


const CPU_EVENT: &str = "cpu_event";
const JAVA_FUTEX_INFO: &str = "java_futex_info";
const TRANSACTION_ID_EVENT: &str = "apm_trace_id_event";
const SPAN_EVENT: &str = "apm_span_event";
const OTHER_EVENT: &str = "other";

const ValueType_NONE: u32 = 0;
const ValueType_INT8: u32 = 1;
const ValueType_INT16: u32 = 2;
const ValueType_INT32: u32 = 3;
const ValueType_INT64: u32 = 4;
const ValueType_UINT8: u32 = 5;
const ValueType_UINT16: u32 = 6;
const ValueType_UINT32: u32 = 7;
const ValueType_UINT64: u32 = 8;
const ValueType_CHARBUF: u32 = 9;
const ValueType_BYTEBUF: u32 = 10;
const ValueType_FLOAT: u32 = 11;
const ValueType_DOUBLE: u32 = 12;
const ValueType_BOOL: u32 = 13;




#[repr(C)]
#[derive(Copy)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct KindlingEventForGo {
    pub(crate) timestamp: u64,
    pub(crate) name: *mut libc::c_char,
    pub(crate) category: u32,
    pub(crate) paramsNumber: u16,
    pub(crate) latency: u64,
    pub(crate) userAttributes: [KeyValue; 16],
    pub(crate) context: EventContext,
}

impl KindlingEventForGo {
    pub fn get_pid(&self) -> u32 {
        let ctx = self.get_ctx();
        if let Some(ctx) = ctx {
            let thread_info = ctx.get_thread_info();
            if let Some(thread_info) = thread_info {
                return thread_info.pid;
            }
        }
        0
    }


    pub fn get_tid(&self) -> u32 {
        let ctx = self.get_ctx();
        if let Some(ctx) = ctx {
            let thread_info = ctx.get_thread_info();
            if let Some(thread_info) = thread_info {
                return thread_info.tid;
            }
        }
        0
    }

    pub fn get_comm(&self) -> String {
        if let Some(ctx) = self.get_ctx() {
            if let Some(thread_info) = ctx.get_thread_info() {
                let c_str = unsafe { CStr::from_ptr(thread_info.comm) };
                if let Ok(str_slice) = c_str.to_str() {
                    return str_slice.to_string();
                }
            }
        }
        String::new()
    }

    fn get_ctx(&self) -> Option<&EventContext> {
        Some(&self.context)
    }

}

impl KeyValue {
    pub fn get_key(&self) -> Option<&str> {
        if !self.key.is_null() {
            unsafe {
                let cstr = CStr::from_ptr(self.key);
                Some(std::str::from_utf8_unchecked(cstr.to_bytes()))
            }
        } else {
            None
        }
    }
    pub fn get_value(&self) -> Option<&[u8]> {
        if !self.value.is_null() {
            unsafe {
                Some(std::slice::from_raw_parts(self.value as *const u8, self.len as usize))
            }
        } else {
            None
        }
    }


    pub fn get_uint_value(&self) -> u64 {
        let value_slice = unsafe { std::slice::from_raw_parts(self.value as *const u8, self.len as usize) };
        match self.valueType {
            ValueType_UINT8 => value_slice[0] as u64,
            ValueType_UINT16 => u16::from_le_bytes([value_slice[0], value_slice[1]]) as u64,
            ValueType_UINT32 => u32::from_le_bytes([value_slice[0], value_slice[1], value_slice[2], value_slice[3]]) as u64,
            ValueType_UINT64 => u64::from_le_bytes([value_slice[0], value_slice[1], value_slice[2], value_slice[3], value_slice[4], value_slice[5], value_slice[6], value_slice[7]]),
            _ => 0,
        }
    }
    pub fn get_int_value(&self) -> i64 {
        let value_slice = unsafe { std::slice::from_raw_parts(self.value as *const u8, self.len as usize) };
        match self.valueType {
            ValueType_INT8 => value_slice[0] as i8 as i64,
            ValueType_INT16 => i16::from_le_bytes([value_slice[0], value_slice[1]]) as i64,
            ValueType_INT32 => i32::from_le_bytes([value_slice[0], value_slice[1], value_slice[2], value_slice[3]]) as i64,
            ValueType_INT64 => i64::from_le_bytes([value_slice[0], value_slice[1], value_slice[2], value_slice[3], value_slice[4], value_slice[5], value_slice[6], value_slice[7]]),
            _ => 0,
        }
    }
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
#[derive(Debug)]
pub struct EventContext {
    pub(crate) tinfo: ThreadInfo,
    pub(crate) fdInfo: FdInfo,
}

impl EventContext {
    fn get_thread_info(&self) -> Option<&ThreadInfo> {
        Some(&self.tinfo)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ThreadInfo {
    pub(crate) pid: u32,
    pub(crate) tid: u32,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) comm: *mut libc::c_char,
    pub(crate) containerId: *mut libc::c_char,
}

#[repr(C)]
#[derive(Debug)]
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
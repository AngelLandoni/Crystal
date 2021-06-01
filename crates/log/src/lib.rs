extern crate crossbeam_queue;

mod console;
pub use console::Console;

use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

use crossbeam_queue::SegQueue;

/// Creates a global LOG holder.
static mut LOG: Option<Arc<Log>> = None;

pub enum LogSeverity {
    INFO,
    WARNING,
    ERROR
}

impl ToString for LogSeverity {
    fn to_string(&self) -> String {
        match self {
            LogSeverity::INFO => "INFO".to_string(),
            LogSeverity::WARNING => "WARNING".to_string(),
            LogSeverity::ERROR => "ERROR".to_string(),
        }
    }
}

pub struct LogEntry {
    date: DateTime<Utc>,
    buffer: String,
    severity: LogSeverity
}

impl LogEntry {
    fn info(str: &str) -> Self {
        Self::new_instance(str, LogSeverity::INFO)
    }

    fn warning(str: &str) -> Self {
        Self::new_instance(str, LogSeverity::WARNING)
    }

    fn error(str: &str) -> Self {
        Self::new_instance(str, LogSeverity::ERROR)
    }

    fn new_instance(str: &str, severity: LogSeverity) -> Self {
        Self {
            date: Utc::now(),
            buffer: String::from(str),
            severity: severity
        }
    }
}

/// Defines a log entry 
pub struct Log {
    /// Contains all the entries in the log.
    entries: SegQueue<LogEntry>,
    listeners: Mutex<Vec<Box<dyn Fn(&LogEntry)>>>
}

impl Log {
    /// Creates and returns a new Log.
    pub fn new() -> Self {
        Self {
             entries: SegQueue::new(),
             listeners: Mutex::new(Vec::new())
        }
    }

    /// Inits the log, this is mandatory in order to be able to use it.
    pub fn init() {
        unsafe {
            LOG = Some(Arc::new(Log::new()))
        }
    }
}

pub fn log_hook(hook: fn(&LogEntry)) {
    unsafe {
        if let Some(log) = &LOG {
            let mut listeners_lock = log.listeners.lock().unwrap();
            listeners_lock.push(Box::new(hook));
            return
        }    
    }
    panic!("Log is not initializated");
}

/// Logs an info log message.
pub fn info(str: &str) {
    unsafe {
        if let Some(log) = &LOG {
            let entry = LogEntry::info(str);
            let listeners_lock = log.listeners.lock().unwrap();
            for listener in listeners_lock.iter() {
                listener(&entry);
            }
            log.entries.push(entry);
            return   
        }    
    }
    panic!("Log is not initializated");
}


/// Logs an warning log message.
pub fn warning(str: &str) {
    unsafe {
        if let Some(log) = &LOG {
            let entry = LogEntry::warning(str);
            let listeners_lock = log.listeners.lock().unwrap();
            for listener in listeners_lock.iter() {
                listener(&entry);
            }
            log.entries.push(entry);
            return
        }    
    }
    panic!("Log is not initializated");
}

/// Logs an error log message.
pub fn error(str: &str) {
    unsafe {
        if let Some(log) = &LOG {
            let entry = LogEntry::error(str);
            let listeners_lock = log.listeners.lock().unwrap();
            for listener in listeners_lock.iter() {
                listener(&entry);
            }
            log.entries.push(entry);
            return 
        }    
    }
    panic!("Log is not initializated");
}

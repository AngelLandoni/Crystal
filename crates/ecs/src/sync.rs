use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
    time::Duration,
    thread
};

/// A type that allows know when a task finished, in a thread safe
/// way using atomics (no locks).
pub struct TaskSync {
    /// Contains a flag which determines if the task was finished or
    /// not.
    finish: AtomicBool,
}

impl Default for TaskSync {
    /// Creates and returns a new `TaskSync` instance with default,
    /// configuration.
    fn default() -> Self {
        Self {
            finish: AtomicBool::new(false)
        }
    }
}

impl TaskSync {
    /// Marks the task sync as finished.
    pub fn mark_as_finish(&self) {
        self.finish.swap(true, Ordering::Relaxed);
    }
}

pub trait TaskWaitable {
    fn wait(self);
}

impl TaskWaitable for (Arc<TaskSync>, ) {
    /// Locks the current thread until the TaskSyncs passed by 
    /// parameters are finished.
    fn wait(self) {
        // Infinite loop that locks the thread and check if all the 
        // flags are true.
        loop {
            let mut did_finish: bool = true;
            did_finish &= self.0.finish.load(Ordering::SeqCst);
            // If the flags are all true it means it finishes.
            if did_finish {
                return;
            }
            // Wait 1 millisecond to not flood the thread.
            thread::sleep(Duration::from_millis(1));
        }
    }
}

macro_rules! generate_task_waitable {
    ($([$type: ident, $index: tt]), +) => {
impl TaskWaitable for ($($type,)+ ) {
    fn wait(self) {
        // Infinite loop that locks the thread and check if all the 
        // flags are true.
        loop {
            let mut did_finish: bool = true;
            $(
                did_finish &= self.$index.finish.load(Ordering::SeqCst);
            )+            
            // If the flags are all true it means it finishes.
            if did_finish {
                return;
            }
            // Wait 1 millisecond to not flood the thread.
            thread::sleep(Duration::from_millis(1));
        }
    }
}       
    };
}

type RefTaskSync = Arc<TaskSync>;

generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4], [RefTaskSync, 5]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4], [RefTaskSync, 5], [RefTaskSync, 6]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4], [RefTaskSync, 5], [RefTaskSync, 6], [RefTaskSync, 7]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4], [RefTaskSync, 5], [RefTaskSync, 6], [RefTaskSync, 7], [RefTaskSync, 8]);
generate_task_waitable!([RefTaskSync, 0], [RefTaskSync, 1], [RefTaskSync, 2], [RefTaskSync, 3], [RefTaskSync, 4], [RefTaskSync, 5], [RefTaskSync, 6], [RefTaskSync, 7], [RefTaskSync, 8], [RefTaskSync, 9]);
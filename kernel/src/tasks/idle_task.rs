use pico_internals::task::yield_task;
use pico_internals::task_trampoline;

task_trampoline! {
    pub fn idle_task_entry(_argument: *mut ()) {
        loop {
            core::hint::spin_loop();
            yield_task();
        }
    }
}

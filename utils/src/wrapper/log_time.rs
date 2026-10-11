use std::time::Instant;
#[macro_export]
macro_rules! log_exec_time {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let ret = $expr;
        let dur = start.elapsed();
        println!("[exec time]: {:?}", dur);
        ret
    }};
}

pub fn log_exec_time<F: FnOnce() -> R, R>(func: F) -> impl FnOnce() -> R {
    move || {
        let start = Instant::now();
        let ret = func();
        let dur = start.elapsed();
        println!("[exec time]: {:?}", dur);
        ret
    }
}

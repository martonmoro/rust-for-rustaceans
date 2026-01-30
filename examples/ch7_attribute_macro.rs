use ch7_proc_macro_impl::timed;

#[timed]
fn slow_task() {
    std::thread::sleep(std::time::Duration::from_millis((150)));
}

#[timed("ns")]
fn fast_task() {
    let x = 1 + 1;
}

#[timed("ms")]
fn with_early_return(x: i32) -> i32 {
    if x < 0 {
        return 0; // Still gets timed!
    }
    x * 2
}

fn main() {
    slow_task();
    fast_task();
    with_early_return(-3);
}

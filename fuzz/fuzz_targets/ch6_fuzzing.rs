#![no_main]

use libfuzzer_sys::fuzz_target;

fn divide(a: u8, b: u8) -> u8 {
    a / b
}

fuzz_target!(|data: &[u8]| {
    if data.len() >= 2 {
        let _ = divide(data[0], data[1]);
    }
});

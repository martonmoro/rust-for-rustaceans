use std::mem::MaybeUninit;

fn main() {
    let mut buf: [MaybeUninit<u8>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
    // we could do this
    // let mut buf: [MaybeUninit<u8>; 1024] = [const { MaybeUninit::uninit() }; 1024];

    let bytes_read = 100;
    for i in 0..bytes_read {
        buf[i].write(i as u8);
    }

    let initialized_slice = unsafe { buf[..bytes_read].assume_init_ref() };

    println!("Read {} bytes", initialized_slice.len());
}

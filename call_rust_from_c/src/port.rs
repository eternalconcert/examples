#![crate_type = "staticlib"]
#![feature(asm)]

use std::{thread, time};

#[no_mangle]
pub extern fn rust_call() {

    println!("RUST!");

    unsafe fn test(port: u16, val: u8) {
        asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
    };


    let time = time::Duration::from_millis(100);

    unsafe {
        let mut i = 0;
        let now = time::Instant::now();
        while i != 255 {
            test(0x378, i);
            i += 1;
            thread::sleep(time);
            assert!(now.elapsed() >= time);
        };
   };
   
}

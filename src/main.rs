#![no_std]

use core::panic::PanicInfo;


fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



fn main() {
    println!("Hello, world!");
}

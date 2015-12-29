
#![feature(lang_items)]
#![feature(const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod io;
mod vga;

#[no_mangle]
pub extern "C" fn main() {
    // ATTENTION: we have a very small stack and no guard page
    
    startup();

    println!("Hello World{}", "!");
    println!("Testing, {},{},{}", 1, 2, 3);
    print!("Print a line");
    print!("Another print");
    loop{}
}

fn startup() {
    vga::clear_screen();
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {loop{}}
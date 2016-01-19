
#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;

// use spin::Mutex;

#[macro_use]
mod io;
mod vga;
mod memory;
// mod cpu;
// mod pic;

// use pic::ChainedPics;

use memory::FrameAllocator;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info_addr: usize) {
    vga::clear_screen();
    
    println!("jackOS");
    setup_memory(multiboot_info_addr);
    
    // Never return
    loop{}
}

fn setup_memory(multiboot_info_addr: usize) {
    let boot_info = unsafe { multiboot2::load(multiboot_info_addr) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    
    let multiboot_start = multiboot_info_addr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
    
    // Allocate all frames
    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }
}


#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
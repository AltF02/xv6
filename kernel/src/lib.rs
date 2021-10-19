#![no_std]

#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]


global_asm!(include_str!("asm/entry.S"));

mod start;
mod panic;
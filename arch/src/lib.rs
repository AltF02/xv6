#![no_std]
#![feature(llvm_asm)]
#![feature(global_asm)]

#[cfg(target_arch = "riscv64")]
#[path = "../riscv64"]
mod arch {
    mod boot;
    mod cpu;
}

#[cfg(target_arch = "x86")]
#[path = "../x86"]
mod arch {
    mod boot;
}

pub use arch::*;

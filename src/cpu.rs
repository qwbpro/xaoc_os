#[path = "_arch/x86_64/cpu.rs"]
mod x86_64_cpu;
mod boot;

pub use x86_64_cpu::wait_forever;
// use crate::exit_qemu;
// use crate::QemuExitCode;
// use core::panic::PanicInfo;
// use crate::println;
// use crate::serial_print;
// use crate::serial_println;


// #[cfg(not(test))]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }


// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);

//     loop {}
// }






/* Print text to the serial port on RISC-V Qemu-emulated virt machines 
 *
 * If you need to debug low-level, non-std RISC-V code, fire this up
 * in a Qemu virt machine, and print to the serial port, which will
 * appear in the terminal if you use the -nographic Qemu switch. For example:
 * 
 * $ qemu-system-riscv64 -bios none -nographic -machine virt -kernel <path to kernel>
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#![no_std]
use core::fmt;

const QEMU_VIRT_SERIAL_PORT: usize = 0x10000000;

#[macro_export]
macro_rules! println
{
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print
{
    ($($arg:tt)*) =>
    ({
        use core::fmt::Write;
        {
            unsafe { $crate::serial::QEMUUART.write_fmt(format_args!($($arg)*)).unwrap(); }
        }
    });
}

pub struct UartWriter;
pub static mut QEMUUART: UartWriter = UartWriter {};

impl fmt::Write for UartWriter
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        for c in s.bytes()
        {
            unsafe { *(QEMU_VIRT_SERIAL_PORT as *mut u8) = c };
        }
        Ok(())
    }
}

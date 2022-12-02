//! main.rs

#![no_main]
#![no_std]

// Include the boot header like this. Otherwise,
// it may be removed by the linker.
use imxrt1010evk_fcb as _;

// Same goes for the panic handler.
use panic_halt as _;

// The entry macro adorns your main function.
use imxrt_rt::entry;

const LED_OFFSET: u32 = 1 << 11;

// Register addresses come from the reference manual.
const IOMUXC_MUX_CTL_PAD_GPIO_11: *mut u32 = 0x401F_8090 as _;
const GPIO1_GDIR: *mut u32 = (0x401B_8000 + 0x04) as _;
const GPIO1_DR_SET: *mut u32 = (0x401B_8000 + 0x84) as _;

#[entry]
fn main() -> ! {
    unsafe {
        // Configure the pad named "GPIO_11" as a GPIO pin
        // (as opposed to a UART TX pin, for example).
        IOMUXC_MUX_CTL_PAD_GPIO_11.write_volatile(5);

        // Set the GPIO as an output with a RMW operation.
        let mut gpio1_gdir = GPIO1_GDIR.read_volatile();
        gpio1_gdir |= LED_OFFSET;
        GPIO1_GDIR.write_volatile(gpio1_gdir);

        // Turn on the LED.
        GPIO1_DR_SET.write_volatile(LED_OFFSET);
    }
    loop {}
}

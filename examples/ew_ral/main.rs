//! main.rs

#![no_main]
#![no_std]

use imxrt_ral as ral;

use imxrt1010evk_fcb as _;
use imxrt_rt::entry;
use panic_halt as _;

const LED_OFFSET: u32 = 1 << 11;

#[entry]
fn main() -> ! {
    // Safety: we're the only code that "owns" the IOMUXC and GPIO1 peripherals.
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let gpio1 = unsafe { ral::gpio::GPIO1::instance() };

    // Configure the pad named "GPIO_11" as a GPIO pin
    // (as opposed to a UART TX pin, for example).
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_11, 5);
    // Set the GPIO as an output with a RMW operation.
    ral::modify_reg!(ral::gpio, gpio1, GDIR, |gdir| gdir | LED_OFFSET);
    // Turn on the LED.
    ral::write_reg!(ral::gpio, gpio1, DR_SET, LED_OFFSET);

    loop {}
}

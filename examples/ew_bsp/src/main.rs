//! main.rs

#![no_main]
#![no_std]

use imxrt_hal as hal;
use imxrt_ral as ral;

use imxrt_rt::entry;
use panic_halt as _;

mod board {
    use core::sync::atomic::{AtomicBool, Ordering};

    /// Called by a board implementation to mark peripherals taken.
    fn take() -> Option<()> {
        static BOARD_FREE: AtomicBool = AtomicBool::new(true);
        BOARD_FREE.swap(false, Ordering::SeqCst).then_some(())
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "teensy4")] {
            mod teensy4;
            pub use teensy4::Resources;
        } else if #[cfg(feature = "imxrt1010evk")] {
            mod imxrt1010evk;
            pub use imxrt1010evk::Resources;
        } else if #[cfg(feature = "imxrt1170evk-cm7")] {
            mod imxrt1170evk_cm7;
            pub use imxrt1170evk_cm7::Resources;
        } else {
            compile_error!("No board selected!");
        }
    }
}

#[entry]
fn main() -> ! {
    let board::Resources { led, .. } = board::Resources::take().unwrap();
    led.set();

    loop {}
}

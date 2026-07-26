//! board/teensy4.rs

use crate::{hal, ral};
use teensy4_fcb as _;

pub type Led = hal::gpio::Output;

#[non_exhaustive]
pub struct Resources {
    pub led: Led,
}

impl Resources {
    pub fn take() -> Option<Self> {
        super::take()?;

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let gpio2 = unsafe { ral::gpio::GPIO2::instance() };

        let mut port = hal::gpio::Port::new(gpio2);
        let pads = hal::iomuxc::into_pads(iomuxc);
        let led = port.output(pads.gpio_b0.p03).unwrap();
        Some(Resources { led })
    }
}

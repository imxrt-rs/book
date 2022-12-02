//! board/teensy4.rs

use crate::{
    hal::{self, iomuxc::pads},
    ral,
};
use teensy4_fcb as _;

pub type Led = hal::gpio::Output<pads::gpio_b0::GPIO_B0_03>;

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
        let led = port.output(pads.gpio_b0.p03);
        Some(Resources { led })
    }
}

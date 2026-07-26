//! board/imxrt1010evk.rs

use crate::{hal, ral};
use imxrt1010evk_fcb as _;

pub type Led = hal::gpio::Output;

#[non_exhaustive]
pub struct Resources {
    pub led: Led,
}

impl Resources {
    pub fn take() -> Option<Self> {
        super::take()?;

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };

        let mut port = hal::gpio::Port::new(gpio1);
        let pads = hal::iomuxc::into_pads(iomuxc);
        let led = port.output(pads.gpio.p11).unwrap();
        Some(Resources { led })
    }
}

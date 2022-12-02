//! board/imxrt1170evk_cm7.rs

use crate::{
    hal::{self, iomuxc::pads},
    ral,
};
use imxrt1170evk_fcb as _;

pub type Led = hal::gpio::Output<pads::gpio_ad::GPIO_AD_04>;

#[non_exhaustive]
pub struct Resources {
    pub led: Led,
}

impl Resources {
    pub fn take() -> Option<Self> {
        super::take()?;

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let iomuxc_lpsr = unsafe { ral::iomuxc_lpsr::IOMUXC_LPSR::instance() };
        let gpio9 = unsafe { ral::gpio::GPIO9::instance() };

        let mut port = hal::gpio::Port::new(gpio9);
        let pads = hal::iomuxc::into_pads(iomuxc, iomuxc_lpsr);
        let led = port.output(pads.gpio_ad.p04);
        Some(Resources { led })
    }
}

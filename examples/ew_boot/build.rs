//! build.rs

use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // The iMXRT1010EVK has 16 MiB of flash.
    RuntimeBuilder::from_flexspi(Family::Imxrt1010, 16 * 1024 * 1024)
        .build()
        .unwrap();
}

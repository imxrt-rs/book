//! build.rs

use imxrt_rt::{Family, RuntimeBuilder};

struct Board {
    family: Family,
    flash_size: usize,
}

const BOARD: Board = if cfg!(feature = "teensy4") {
    Board {
        family: Family::Imxrt1060,
        flash_size: 1984 * 1024,
    }
} else if cfg!(feature = "imxrt1010evk") {
    Board {
        family: Family::Imxrt1010,
        flash_size: 16 * 1024 * 1024,
    }
} else if cfg!(feature = "imxrt1170evk-cm7") {
    Board {
        family: Family::Imxrt1170,
        flash_size: 16 * 1024 * 1024,
    }
} else {
    panic!("No board selected!")
};

fn main() {
    RuntimeBuilder::from_flexspi(BOARD.family, BOARD.flash_size)
        .build()
        .unwrap();
}

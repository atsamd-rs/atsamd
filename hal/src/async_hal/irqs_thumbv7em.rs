use crate::pac::Interrupt as InterruptEnum;
use embassy::interrupt::declare;

declare!(DMAC_0);
declare!(DMAC_1);
declare!(DMAC_2);
declare!(DMAC_3);
declare!(DMAC_OTHER);

declare!(TC2);
declare!(TC3);

#[cfg(feature = "min-samd51j")]
declare!(TC4);
#[cfg(feature = "min-samd51j")]
declare!(TC5);

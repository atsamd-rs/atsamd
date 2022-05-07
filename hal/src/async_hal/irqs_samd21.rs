use crate::pac::Interrupt as InterruptEnum;
use embassy::interrupt::declare;

declare!(DMAC);

declare!(TC3);
declare!(TC4);
declare!(TC5);

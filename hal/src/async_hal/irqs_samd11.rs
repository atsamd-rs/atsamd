use crate::pac::Interrupt as InterruptEnum;
use embassy::interrupt::declare;

declare!(DMAC);

declare!(TC1);

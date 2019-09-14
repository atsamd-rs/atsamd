use crate::{
    target_device::{QSPI, MCLK}, 
    gpio::{Pa8, Pa9, Pa10, Pa11, Pb10, Pb11, Input, Floating, PfH, Port}
};

pub struct Qspi {
    qspi: QSPI,
    sck: Pb10<PfH>,
    cs:  Pb11<PfH>,
    io0: Pa8<PfH>,
    io1: Pa9<PfH>,
    io2: Pa10<PfH>,
    io3: Pa11<PfH>,
}

impl Qspi {
   pub fn new(
        mclk: &mut MCLK,
        port: &mut Port,
        qspi: QSPI,
        sck: Pb10<Input<Floating>>,
        cs:  Pb11<Input<Floating>>,
        io0: Pa8<Input<Floating>>,
        io1: Pa9<Input<Floating>>,
        io2: Pa10<Input<Floating>>,
        io3: Pa11<Input<Floating>>,
    ) -> Qspi {
        let sck = sck.into_function_h(port);
        let cs = cs.into_function_h(port);
        let io0 = io0.into_function_h(port); 
        let io1 = io1.into_function_h(port); 
        let io2 = io2.into_function_h(port); 
        let io3 = io3.into_function_h(port); 

        mclk.apbcmask.modify(|_, w| w.qspi_().set_bit());
        mclk.ahbmask.modify(|_, w| w.qspi_().set_bit());
        mclk.ahbmask.modify(|_, w| w.qspi_2x_().clear_bit());

        qspi.ctrla.modify(|_, w| w.swrst().set_bit());
        qspi.baud.modify(|_, w| {
            unsafe { w.baud().bits(14) }; //120MHz / (14+1) = 8MHz
            
            // SPI MODE 0
            w.cpol().clear_bit();
            w.cpha().clear_bit()
        });

        qspi.ctrlb.modify(|_, w| {
            w.mode().memory();
            w.csmode().noreload();
            w.csmode().lastxfer();
            w.datalen()._8bits()
        });

        qspi.ctrla.modify(|_, w| w.enable().set_bit());

        Qspi { 
            qspi,
            sck,
            cs,
            io0,
            io1,
            io2,
            io3,
        }
    }
}

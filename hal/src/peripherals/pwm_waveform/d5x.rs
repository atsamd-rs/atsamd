#![allow(non_snake_case)]

use atsamd_hal_macros::hal_cfg;

#[cfg(feature = "async")]
use crate::dmac::ReadyFuture;
use crate::dmac::{AnyChannel, Beat, Buffer, Error as DmacError, TriggerAction, TriggerSource};
//  use crate::gpio::*;
use crate::gpio::PinId;
use crate::pac::Mclk;
use crate::time::Hertz;
use crate::timer_params::TimerParams;

use paste::paste;

#[derive(Clone)]
pub struct PwmWaveformGeneratorPtr<T: Beat>(pub(in super::super) *mut T);

unsafe impl<T: Beat> Buffer for PwmWaveformGeneratorPtr<T> {
    type Beat = T;

    #[inline]
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.0
    }

    #[inline]
    fn incrementing(&self) -> bool {
        false
    }

    #[inline]
    fn buffer_len(&self) -> usize {
        1
    }
}

pub use crate::pwm::PinoutCollapse;
pub trait PwmWgFutureTrait {
    type DmaChannel: AnyChannel<Status=ReadyFuture>;
    type TC;
    type Pinout: PinoutCollapse;

    fn decompose(self) -> (Self::DmaChannel, Self::TC, Self::Pinout);
    fn start_regular_pwm(&mut self, ccx_value: u8);
    async fn start_timer_prepare_dma_transfer<const N:usize, const INVERT:bool>(
        &mut self, 
        ccx_value:u8, 
        generation_pattern_iter: impl Iterator<Item=bool>) 
    -> Result<(), DmacError>;
}
pub trait PwmBaseTrait {
    type TC;
    type Pinout: PinoutCollapse;
    type ConvertibleToFuture<D>: PwmWgFutureTrait<TC = Self::TC, Pinout = Self::Pinout, DmaChannel = D>
    where
        D: AnyChannel<Status=ReadyFuture>;

    //  type Future: PwmWgFutureTrait<TC = Self::TC, Pinout = Self::Pinout, DmaChannel = Self::DmaChannel>;

    /// Create a new PWM Waveform Generator
    fn new_waveform_generator(
        clock_freq: Hertz,
        freq: Hertz,
        tc: Self::TC,
        pinout: Self::Pinout,
        mclk: Option<&mut Mclk>,
    ) -> Self;
    fn with_dma_channel<CH>(self, channel: CH) -> Self::ConvertibleToFuture<CH>
    where
        CH: AnyChannel<Status=ReadyFuture>;
}
// Timer/Counter (TCx)
//
macro_rules! pwm_wg {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident, $event:ident)),+) => {
        $(

use crate::pwm::$pinout;

pub struct $TYPE<I: PinId> {
    /// The frequency of the attached clock, not the period of the pwm.
    /// Used to calculate the period of the pwm.
    clock_freq: Hertz,
    requested_freq: Hertz,
    tc: crate::pac::$TC,
    #[allow(dead_code)]
    pinout: $pinout<I>,
    //  _channel: Option<DmaCh>,
}

paste!{
pub struct [<$TYPE Future>]<I: PinId, DmaCh: AnyChannel<Status=ReadyFuture>>{
    base_pwm: $TYPE<I>,
    _channel: DmaCh,
    _init_level: u8,
}

impl<I: PinId, DmaCh: AnyChannel<Status=ReadyFuture>> [<$TYPE Future>]<I, DmaCh> {
    fn get_init_level(&self) -> u8 {
        self._init_level
    }

}

impl<I: PinId, DmaCh: AnyChannel<Status=ReadyFuture>> PwmWgFutureTrait for [<$TYPE Future>]<I, DmaCh> {
    type DmaChannel = DmaCh;
    type TC = crate::pac::$TC;
    type Pinout = $pinout<I>;

    async fn start_timer_prepare_dma_transfer<const N: usize, const INVERT: bool>(&mut self, ccx_value:u8, generation_pattern_iter: impl Iterator<Item=bool>) 
        -> Result<(), DmacError> {

        let init_level = self.get_init_level();
        let mut generation_pattern_dma: [u8; N] = [init_level; N];
        for (idx, value) in generation_pattern_iter.enumerate() {
            //  TODO: move it to the right because for a reason it is not visisble on the wire
            //  plus resolve the initial driver state. Before the first TX it is low instead of high.
            let idx = idx + 2;
            if idx >= N {
                break;
            }
            //  Implement conditional inversion of the signal:
            let value = value != INVERT;
            //  TODO: Implement configurable idle bus state level
            let level = if value { 0xffu8 } else { 0x00u8 };
            generation_pattern_dma[idx] = level;
            //  hprintln!("trigger::source[{}]: {}", idx, level).ok();
        }

        let count = self.base_pwm.tc.count8();

        count.cc(0).write(|w| unsafe { w.bits(0x00u8) });
        while count.syncbusy().read().cc0().bit_is_set() {}
        count.cc(1).write(|w| unsafe { w.bits(ccx_value) });
        while count.syncbusy().read().cc1().bit_is_set() {}
        count.ccbuf(0).write(|w| unsafe { w.bits(0x00u8) });
        count.ccbuf(1).write(|w| unsafe { w.bits(ccx_value) });

        let pwm_dma_address = self.base_pwm.get_dma_ptr();
        let dma_future = self._channel.as_mut().transfer_future(
            &mut generation_pattern_dma,
            pwm_dma_address,
            TriggerSource::$event,
            TriggerAction::Burst,
        );
        //  Rest of the setup shall go into poll method: i.e. enabling interrupts and the counter
        //  of the timer.
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}

        //  First poll the future starts the DMA transfer. It sets enable bit of the DMA
        let value_to_return = dma_future.await;

        count.cc(1).write(|w| unsafe { w.bits(ccx_value) });
        count.ccbuf(1).write(|w| unsafe { w.bits(ccx_value) });

        value_to_return
    }
    fn start_regular_pwm(&mut self, ccx_value: u8) {
        self._init_level = ccx_value;
        let count = self.base_pwm.tc.count8();
        count.cc(0).write(|w| unsafe { w.bits(0x00u8) });
        while count.syncbusy().read().cc0().bit_is_set() {}
        count.cc(1).write(|w| unsafe { w.bits(ccx_value) });
        while count.syncbusy().read().cc1().bit_is_set() {}

        count.ccbuf(0).write(|w| unsafe { w.bits(0x00u8) });
        count.ccbuf(1).write(|w| unsafe { w.bits(ccx_value) });

        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }

    fn decompose(self) -> (Self::DmaChannel, Self::TC, Self::Pinout)
    {
        let $TYPE{clock_freq, requested_freq, tc, pinout} = self.base_pwm;
        (self._channel, tc, pinout)
    }
}

/*
///
/// PRESCALER = 6, in reference project written in C++
/// cc = 233
/// per = 233
///
/// As the timer is set to produce idle level signal, it can be started before
/// we start the DMA transfer. It will naturally pick and start from the
/// beginning of next cycle of the timer. Timer is configured to prodduce
/// constant period signal by setting PER register to a value that corresponds
/// to requested frequency of the manchester signal. Base of the working
/// principle is that the timer CCx register will be loaded with either 0x00
/// of 0xFF to produce either full cycle high or low signal.
*/
impl<I: PinId> PwmBaseTrait for $TYPE<I> {
    type TC = crate::pac::$TC;
    type Pinout = $pinout<I>;
    type ConvertibleToFuture<D> = [<$TYPE Future>]<I, D> where
        D: AnyChannel<Status=ReadyFuture>;

    fn new_waveform_generator(
        clock_freq: Hertz,
        freq: Hertz,
        tc: Self::TC,
        pinout: Self::Pinout,
        mclk: Option<&mut Mclk>,
    ) -> Self {
        const TIEMR_PERIOD: u8 = 233;  //  mclk / 256 / 233 = 1000 Hz
        let count = tc.count8();
        let tc_ccbuf_dma_data_register_address = tc.count8().ccbuf(1).as_ptr() as *const ();
        //  let PwmWaveformGeneratorPtr()(pub(in super::super) *mut T);

        //  write(|w| w.ccbuf().bits(duty as u8));
        let _params = TimerParams::new(freq.convert(), clock_freq);
        //  Works as a mask: you can only enable the clock not disable it. TODO: check if it is set in read only/steal method.
        match mclk {
            Some(mclk) => {
                mclk.$apmask().modify(|_, w| w.$apbits().set_bit());
            }
            None => {}
        }
        count.ctrla().write(|w| w.swrst().set_bit());
        while count.ctrla().read().bits() & 1 != 0 {}
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.ctrla().modify(|_, w| w.mode().count8());
        count.ctrla().modify(|_, w| {
            w.prescaler().div256()
            //  match params.divider {
            //      1 => w.prescaler().div1(),
            //      2 => w.prescaler().div2(),
            //      4 => w.prescaler().div4(),
            //      8 => w.prescaler().div8(),
            //      16 => w.prescaler().div16(),
            //      64 => w.prescaler().div64(),
            //      256 => w.prescaler().div256(),
            //      1024 => w.prescaler().div1024(),
            //      _ => unreachable!(),
            //  }
        });

        count.count().write(|w| unsafe { w.bits(233u8) });
        count.per().write(|w| unsafe { w.bits(233u8) });
        count.perbuf().write(|w| unsafe { w.bits(233u8) });
        //  while count.syncbusy().read().per().bit_is_set() {}

        count.wave().write(|w| w.wavegen().npwm());
        //  while count.syncbusy().read().wave().bit_is_set() {}

        //  TODO: do the test:
        //  prerequisites: forget DMA configuration
        //  1) Set CCx to 0x00 and measure the signal value
        //  2) Set CCx to 0x7F and measure the signal value and frequency
        //  3) Set CCx to 0xFF and measure the signal value
        count.cc(0).write(|w| unsafe { w.bits(0x00u8/*params.cycles as u8*/) });
        while count.syncbusy().read().cc0().bit_is_set() {}
        count.cc(1).write(|w| unsafe { w.bits(0xffu8) });
        while count.syncbusy().read().cc1().bit_is_set() {}

        Self {
            clock_freq: clock_freq,
            requested_freq: freq,
            tc,
            pinout,
        }
    }

    //  pub fn with_dma_channels<R, T>(self, rx: R, tx: T) -> Spi<C, D, R, T>
    fn with_dma_channel<CH>(self, channel: CH) -> Self::ConvertibleToFuture<CH>
    where
        CH: AnyChannel<Status=ReadyFuture>
    {
        [<$TYPE Future>] {
            base_pwm: self,
            _channel: channel,
            _init_level: 0x00u8,
        }
    }
}

impl<I: PinId> $TYPE<I> {

    pub fn start(&mut self) {
        //  Rest of the setup shall go into poll method: i.e. enabling interrupts and the counter
        //  of the timer.
        let count = self.tc.count8();
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }
    pub fn GetDmaPtr(tc: crate::pac::$TC) -> PwmWaveformGeneratorPtr<u8> {
        PwmWaveformGeneratorPtr(tc.count8().ccbuf(1).as_ptr() as *mut _)
    }
    pub fn get_dma_ptr(&self) -> PwmWaveformGeneratorPtr<u8> {
        PwmWaveformGeneratorPtr(self.tc.count8().ccbuf(1).as_ptr() as *mut _)
    }

    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count8();
        let divisor = count.ctrla().read().prescaler().bits();
        let top = count.cc(0).read().cc().bits();
        self.clock_freq / divisor as u32 / (top + 1) as u32
    }

    pub fn set_period(&mut self, period: Hertz)
    {
        let period = period.into();
        let params = TimerParams::new(period, self.clock_freq);
        let count = self.tc.count8();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.ctrla().modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.cc(0).write(|w| unsafe { w.cc().bits(params.cycles as u8) });
        while count.syncbusy().read().cc0().bit_is_set() {}
    }
}
}  //  paste!()

impl<I: PinId> $crate::ehal::pwm::ErrorType for$TYPE<I> {
    type Error = ::core::convert::Infallible;
}

impl<I: PinId> $crate::ehal::pwm::SetDutyCycle for $TYPE<I> {
    fn max_duty_cycle(&self) -> u16 {
        let count = self.tc.count8();
        let top = count.cc(0).read().cc().bits();
        top as u16
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let count = self.tc.count8();
        unsafe { count.ccbuf(1).write(|w| w.ccbuf().bits(duty as u8)); }
        Ok(())
    }
}

impl<I: PinId> $crate::ehal_02::PwmPin for $TYPE<I> {
    type Duty = u16;

    fn disable(&mut self) {
        let count = self.tc.count8();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }

    fn enable(&mut self) {
        let count = self.tc.count8();
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }


    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count8();
        let duty: u8 = count.ccbuf(1).read().ccbuf().bits();
        duty as Self::Duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        use $crate::ehal::pwm::SetDutyCycle;
        self.max_duty_cycle()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        use $crate::ehal::pwm::SetDutyCycle;
        let _ignore_infaillible = self.set_duty_cycle(duty);
    }
}

)+}}

#[hal_cfg("tc0")]
pwm_wg! { PwmWg0: (Tc0, TC0Pinout, Tc0Tc1Clock, apbamask, tc0_, PwmWg0Wrapper, Tc0Ovf) }
#[hal_cfg("tc1")]
pwm_wg! { PwmWg1: (Tc1, TC1Pinout, Tc0Tc1Clock, apbamask, tc1_, PwmWg1Wrapper, Tc1Ovf) }
#[hal_cfg("tc2")]
pwm_wg! { PwmWg2: (Tc2, TC2Pinout, Tc2Tc3Clock, apbbmask, tc2_, PwmWg2Wrapper, Tc2Ovf) }
#[hal_cfg("tc3")]
pwm_wg! { PwmWg3: (Tc3, TC3Pinout, Tc2Tc3Clock, apbbmask, tc3_, PwmWg3Wrapper, Tc3Ovf) }
#[hal_cfg("tc4")]
pwm_wg! { PwmWg4: (Tc4, TC4Pinout, Tc4Tc5Clock, apbcmask, tc4_, PwmWg4Wrapper, Tc4Ovf) }
#[hal_cfg("tc5")]
pwm_wg! { PwmWg5: (Tc5, TC5Pinout, Tc4Tc5Clock, apbcmask, tc5_, PwmWg5Wrapper, Tc5Ovf) }
#[hal_cfg("tc6")]
pwm_wg! { PwmWg6: (Tc6, TC6Pinout, Tc6Tc7Clock, apbdmask, tc6_, PwmWg6Wrapper, Tc6Ovf) }
#[hal_cfg("tc7")]
pwm_wg! { PwmWg7: (Tc7, TC7Pinout, Tc6Tc7Clock, apbdmask, tc7_, PwmWg7Wrapper, Tc7Ovf) }

    //  ($($TYPE:ident: ($TC:ident, $pinout:ident, $clock:ident, $apmask:ident, $apbits:ident, $wrapper:ident, $event:ident)),+) => {
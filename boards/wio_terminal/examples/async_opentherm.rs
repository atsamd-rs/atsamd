#![no_std]
#![no_main]

use bsp::hal::time::Hertz;
use core::time::Duration;
use defmt_rtt as _;
use hal::fugit::Hertz as FugitHertz;
use hal::fugit::MillisDuration;
use heapless::Vec;
use panic_probe as _;

use embassy_futures::join;
use embassy_sync::channel::{Channel, Receiver};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;

use crate::pac::Mclk;
use bsp::pac;
use crate::bsp::hal;
use hal::gpio::{Output, Input, OutputConfig, Pins, PinId, PushPull, PushPullOutput, Floating};
use hal::gpio::{E, PB08, PB09, PA16, PA17};
//  use wio_terminal::prelude::_embedded_hal_PwmPin;
use wio_terminal::aliases::UserLed;

use hal::{
    clock::{ClockGenId, ClockSource, GenericClockController, Tc4Tc5Clock, Tc2Tc3Clock},
    delay::Delay,
    dmac,
    dmac::{DmaController, PriorityLevel, Ch1, ReadyFuture},
    ehal::digital::{OutputPin, StatefulOutputPin},
    eic::{Eic, Sense},
    gpio::{Pin as GpioPin, PullUp, PullUpInterrupt},
    pwm::{TC4Pinout, TC2Pinout, PinoutNewTrait},
    pwm_wg::{PwmWg4, PwmWg2},
    timer_capture_waveform::{TimerCapture4, TimerCapture4Future, TimerCaptureFutureTrait, TimerCaptureBaseTrait, TimerCaptureResultAvailable},
};
use wio_terminal::prelude::_embedded_hal_blocking_delay_DelayMs;

//  use bsp::pins::UserLed;
use wio_terminal as bsp;

use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use rtic_monotonics::Monotonic;

use modular_bitfield::prelude::*;

//  use static_cell::StaticCell;

rtic_monotonics::systick_monotonic!(Mono, 10000);

#[cfg(feature = "use_opentherm")]
use boiler::opentherm_interface::{
    edge_trigger_capture_interface::{
        CaptureError, EdgeCaptureInterface, EdgeCaptureTransitiveToTriggerCapable, CapturedEdgePeriod, CaptureTypeEdges,
        EdgeTriggerInterface, EdgeTriggerTransitiveToCaptureCapable, InitLevel, TriggerError,
    },
    open_therm_message::{CHState, Temperature, OpenThermMessage},
    OpenThermEdgeTriggerBus,
    SendOpenThermMessage,
    ListenOpenThermMessage,
};
#[cfg(feature = "use_opentherm")]
use boiler::{BoilerControl, Instant, TimeBaseRef};
#[cfg(feature = "use_opentherm")]
use opentherm_boiler_controller_lib as boiler;

#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use cortex_m_semihosting::hprintln;

atsamd_hal::bind_interrupts!(struct Irqs {
    EIC_EXTINT_5 => atsamd_hal::eic::InterruptHandler;
    TC4 => atsamd_hal::timer_capture_waveform::TimerCaptureInterruptHandler<atsamd_hal::timer_capture_waveform::TimerCapture4InterruptData>;
});

atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

enum SignalTxSimulation{Ready_}
static CHANNEL: Channel<ThreadModeRawMutex, SignalTxSimulation, 64> = Channel::new();

trait OtMode {}
struct NoneT {}
struct OtTx {}
struct OtRx {}
impl OtMode for OtTx {}
impl OtMode for OtRx {}
impl OtMode for NoneT {}

#[cfg(feature = "use_opentherm")]
#[embassy_executor::task]
async fn boiler_task() {}

#[cfg(feature = "use_opentherm")]
mod boiler_implementation {
    use crate::dmac::ReadyFuture;
    use crate::timer_data_set::PinoutSpecificDataImplTc4;
    use atsamd_hal::gpio::G;
    use atsamd_hal::gpio::{Alternate, PullUpInput, pin::AnyPin};
    use atsamd_hal::pac::dsu::length;
    use atsamd_hal::pac::gclk::genctrl::OeR;
    use atsamd_hal::pac::tcc0::per;
    use atsamd_hal::pwm_wg::{PwmWgFutureTrait, PwmBaseTrait, PinoutCollapse};
    use atsamd_hal::timer_capture_waveform::{self, TimerCaptureData, TimerCaptureFutureTrait};
    use fugit::MicrosDuration;
    use core::any::Any;
    use core::marker::PhantomData;

    use super::*;

    trait AtsamdEdgeTriggerCaptureFactory{

    }

    pub(super) const VEC_SIZE_CAPTURE: usize = 128;

    pub(super) trait CreatePwmPinout {
        type PinTxId: PinId;
        type PinRxId: PinId;
        type PinTx: AnyPin;
        type PinRx: AnyPin;
        type DmaChannel: dmac::AnyChannel<Status = ReadyFuture>;
        type Timer;
        type PinoutTx: PinoutCollapse<PinId = Self::PinTxId> + PinoutNewTrait<Self::PinTxId>;
        type PinoutRx: PinoutCollapse<PinId = Self::PinRxId> + PinoutNewTrait<Self::PinRxId>;
        type PwmBase: PwmBaseTrait<TC = Self::Timer, Pinout = Self::PinoutTx, ConvertibleToFuture<Self::DmaChannel> = Self::PwmWg>;
        type PwmWg: PwmWgFutureTrait<DmaChannel = Self::DmaChannel, Pinout = Self::PinoutTx, TC = Self::Timer>;
        type TimerCaptureBase: TimerCaptureBaseTrait<TC = Self::Timer, Pinout = Self::PinoutRx, ConvertibleToFuture<Self::DmaChannel> = Self::TimerCaptureFuture>;
        type TimerCaptureFuture: TimerCaptureFutureTrait<DmaChannel = Self::DmaChannel, TC = Self::Timer, Pinout = Self::PinoutRx>;
        //  fn new_pwm_generator<'a>(pin: Self::PinTx, tc: Self::Timer, dma: Self::DmaChannel, mclk: &mut Mclk) -> Self::PwmWg;
        //  fn collapse(self) -> Self::PinTx;
    }

    pub(super) struct AtsamdEdgeTriggerCapture<
        PinoutSpecificData: CreatePwmPinout,
        M: OtMode = OtTx,
        const N: usize = VEC_SIZE_CAPTURE,
    > {
        tx_pin: Option<GpioPin<PinoutSpecificData::PinTxId, Output<PushPull>>>,
        rx_pin: Option<GpioPin<PinoutSpecificData::PinRxId, Input<PullUp>>>,

        //  Pin<PB08, Output<PushPull>>`, found `TC4Pinout<PB09>``
        tx_init_duty_value: u8,
        pwm: Option<PinoutSpecificData::PwmWg>, // one alternative when TX operation
        capture_device: Option<PinoutSpecificData::TimerCaptureFuture>, // one alternative when RX operation
        periph_clock_freq: Hertz,
        mode: PhantomData<M>,
        pinout: PhantomData<PinoutSpecificData>,
        }

    impl<PinoutSpecificData, const N: usize> AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
        OtTx: OtMode,
    {
        pub fn new_with_default(
            pin_tx: GpioPin<PinoutSpecificData::PinTxId, Output<PushPull>>,
            pin_rx: GpioPin<PinoutSpecificData::PinRxId, Input<PullUp>>,
            tc_timer: PinoutSpecificData::Timer /*pac::Tc4*/,
            mclk: &mut Mclk,
            input_clock_frequency: Hertz,
            pinout_factory: PinoutSpecificData,
            dma_channel: PinoutSpecificData::DmaChannel,
        ) -> AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N> {
            let pwm_tx_pin = pin_tx.into_alternate::<E>();

            //  Enable clocks for capture timer
            PinoutSpecificData::TimerCaptureBase::enable_mclk_clocks(mclk);

            let pwm_generator_future = PinoutSpecificData::PwmBase::new_waveform_generator(
                input_clock_frequency,
                Hertz::from_raw(32),
                tc_timer,
                PinoutSpecificData::PinoutTx::new_pin(pwm_tx_pin),
                Some(mclk),
            ).with_dma_channel(dma_channel);

            Self {
                tx_pin: None,
                rx_pin: Some(pin_rx),
                tx_init_duty_value: 0xff, // This determines idle bus state level. TODO: add configuration
                pwm: Some(pwm_generator_future),
                capture_device: None,
                periph_clock_freq: input_clock_frequency,
                mode: PhantomData,
                pinout: PhantomData,
            }
        }
    }

    impl<PinoutSpecificData, const N: usize> AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        //  Starting with TX as the boiler controller is more common and uses the TX command first
        pub fn new(
            pin_tx: GpioPin<PinoutSpecificData::PinTxId, Output<PushPull>>,
            pin_rx: GpioPin<PinoutSpecificData::PinRxId, Input<PullUp>>,
            tc_timer: PinoutSpecificData::Timer,
            periph_clock_freq: Hertz,
            dma_channel: PinoutSpecificData::DmaChannel,
        ) -> AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N> {
            let pwm_tx_pin = pin_tx.into_alternate::<E>();

            let pwm_generator_future = PinoutSpecificData::PwmBase::new_waveform_generator(
                periph_clock_freq,
                Hertz::from_raw(32),
                tc_timer,
                PinoutSpecificData::PinoutTx::new_pin(pwm_tx_pin),
                None,
            ).with_dma_channel(dma_channel);

            Self {
                tx_pin: None,
                rx_pin: Some(pin_rx),
                tx_init_duty_value: 0xff, // This determines idle bus state level. TODO: add configuration
                pwm: Some(pwm_generator_future),
                capture_device: None,
                periph_clock_freq: periph_clock_freq,
                mode: PhantomData,
                pinout: PhantomData,
            }
        }
    }

    impl<PinoutSpecificData, const N: usize> EdgeTriggerTransitiveToCaptureCapable
        for AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        type CaptureDevice = AtsamdEdgeTriggerCapture<PinoutSpecificData, OtRx, N>;
        fn transition_to_capture_capable_device(self) -> Self::CaptureDevice {
            let pwm = self.pwm.unwrap();
            let (dma, tc_timer, pinout) = pwm.decompose();
            let pin_tx = pinout.collapse();
            let pin_tx = pin_tx.into_push_pull_output();

            AtsamdEdgeTriggerCapture::<PinoutSpecificData, OtRx, N>::new(
                pin_tx,
                self.rx_pin.unwrap(),
                tc_timer,
                self.periph_clock_freq,
                dma,
            )
        }
    }

    impl<PinoutSpecificData, const N: usize> EdgeCaptureTransitiveToTriggerCapable
        for AtsamdEdgeTriggerCapture<PinoutSpecificData, OtRx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        type TriggerDevice = AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N>;
        fn transition_to_trigger_capable_device(self) -> AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N> {
            let (pin_tx, capture_timer) = (
                self.tx_pin.unwrap(),
                self.capture_device.unwrap(),
            );
            let (dma, tc_timer, pinout_rx) = capture_timer.decompose();
            //  let (dma, tc_timer, pinout) = pwm.decompose();
            let pin_rx = pinout_rx.collapse();
            let pin_rx = pin_rx.into_pull_up_input();

            AtsamdEdgeTriggerCapture::<PinoutSpecificData, OtTx, N>::new(
                pin_tx,
                pin_rx,
                tc_timer,
                self.periph_clock_freq,
                dma,
            )
        }
    }

    impl<PinoutSpecificData, const N: usize>
            AtsamdEdgeTriggerCapture<PinoutSpecificData, OtRx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        pub fn new(
            pin_tx: GpioPin<PinoutSpecificData::PinTxId, Output<PushPull>>,
            pin_rx: GpioPin<PinoutSpecificData::PinRxId, Input<PullUp>>,
            tc_timer: PinoutSpecificData::Timer,
            periph_clock_freq: Hertz,
            dma_channel: PinoutSpecificData::DmaChannel,
        ) -> Self {
            let pwm_rx_pin = pin_rx.into_alternate::<E>();

            let pinout = PinoutSpecificData::PinoutRx::new_pin(pwm_rx_pin);

            let timer_capture =
                    PinoutSpecificData::TimerCaptureBase::new_timer_capture(
                periph_clock_freq,
                Hertz::from_raw(32),
                tc_timer,
                pinout,
                None,
            )
            .with_dma_channel(dma_channel); // TODO: Channel shall be changed to channel0 later on. This is
                                            // just for prototyping

            Self {
                tx_pin: Some(pin_tx),
                rx_pin: None,
                tx_init_duty_value: 0xff, // This determines idle bus state level. TODO: add configuration
                pwm: None,
                capture_device: Some(timer_capture), //  TODO: Implement the capture device
                periph_clock_freq: periph_clock_freq,
                mode: PhantomData,
                pinout: PhantomData,
            }
        }
    }

    impl<PinoutSpecificData, const N: usize> EdgeTriggerInterface
    for
        AtsamdEdgeTriggerCapture<PinoutSpecificData, OtTx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        type OutputSelf = Self;
        async fn trigger(
            mut self,
            iterator: impl Iterator<Item = bool>,
            period: core::time::Duration,
        ) -> (Self, Result<(), TriggerError>) {

            //  Invert for hardware adapter compensation:
            const INVERT_SIGNAL: bool = true;
            //  TODO: Implement the period arg usage
            let response = match self.pwm.as_mut() {
                Some(pwm) => {
                    //  let mut source: [u8; N] = [self.tx_init_duty_value; N];
                    //  TODO: Actually use the period to set the PWM frequency
                    pwm.start_regular_pwm(self.tx_init_duty_value);
                    let dma_future = self
                        .pwm
                        .as_mut()
                        .unwrap() /* TODO: remove runtime panic */
                        .start_timer_prepare_dma_transfer::<N, INVERT_SIGNAL>(self.tx_init_duty_value, iterator);
                    dma_future.await.map_err(|_| TriggerError::GenericError)
                }
                None => Err(TriggerError::GenericError),
            };
            (self, response)
        }
    }

    impl<PinoutSpecificData, const N: usize> EdgeCaptureInterface for AtsamdEdgeTriggerCapture<PinoutSpecificData, OtRx, N>
    where
        PinoutSpecificData: CreatePwmPinout,
    {
        type OutputSelf = Self;
        async fn start_capture<OutputType: Extend<CapturedEdgePeriod>>(
            mut self,
            mut container: OutputType,  // fills the container with captured edges or drops it in case of error
            timeout_inactive_capture: Duration,
            timeout_till_active_capture: Duration,
        ) -> (Self, Result<(OutputType, CaptureTypeEdges), CaptureError>) {
            ///  TODO: <declare conditions on timer to finish the capture>
            ///  1) Timeout scenario: maybe realized with 32b timer overflow. Will require to set timer overflow event to happen at around 800ms
            ///  2) Correct frame finish detection: implemented with reading the timer counter register value in a loop
            ///  The C++ driver does it by checking number of edges detected to be captured so far by polling in the elements of the DMA buffer array.
            ///  This element could be improved by reading some internal register of DMA that returns this count, as the array itself is borrowed by DMA and Rust would not allow to read it.
            ///  In C++ idle bus time is based on the above mentioned edge count by using the independent system timestamp capure mechanism. Maybe that can be improved as well.
            //  let mut capture_memory: [u32; N] = [0; N];
            let mut data_container: Vec<core::time::Duration, N> = Vec::new();
            let init_level = self.capture_device.as_mut().unwrap().read_pin_level();
            let result = self.capture_device
                .as_mut()
                .unwrap()
                .start_timer_execute_dma_transfer::<_, N>(data_container).await;
            if let Ok(capture_result) = result {

                //  let mut timestamps = Vec::<core::time::Duration, N>::new();
                //  //  The start of the timer is assumed at counter value equal to zero so the lenght can be set to 0ms of relative capture time.
                //  let _ = timestamps.push(core::time::Duration::from_nanos(0u64));
                //  for (idx, value) in capture_memory.iter().enumerate() {
                //      //  TODO: Fix by using the dma transfer coun instead of using non-zero values condition
                //      //  hprintln!("memory captured[{}] = {}", idx, *value).ok();
                //      if *value > 0 {
                //          let ratio_adjusted = (*value as u64 * 4173) / 1000;  //  TODO: fix this by some ration compund type
                //          let _ = timestamps.push(core::time::Duration::from_nanos(ratio_adjusted));
                //      }
                //  }
                //  let differences: Vec<core::time::Duration, N> = timestamps
                //      .iter()
                //      .zip(timestamps.iter().skip(1))
                //      .map(|(a, b)| if b > a {*b - *a} else {core::time::Duration::from_micros(0)}).collect();
                //  let mut differences_reverse: Vec<Duration, N> = Vec::new();
                //  for value in differences.iter().rev() {
                //      let _ = differences_reverse.push(*value);
                //      hprintln!("timestamps difference: {}ns", value.as_nanos()).ok();
                //  }
                //  let differences_reverse = differences_reverse;
                //  match capture_result {
                //      TimerCaptureResultAvailable::DmaPollReady(timer_value_at_termination) => {
                //          let _ = timestamps.push(core::time::Duration::from_micros(timer_value_at_termination.get_raw_value() as u64));
                //          hprintln!("TimerCaptureResultAvailable::DmaPollReady: {}, lvl:{:?}, N={}", timer_value_at_termination.get_raw_value(), init_level, timestamps.len()).ok();
                //      }
                //      TimerCaptureResultAvailable::TimerTimeout(timer_value_at_termination) => {
                //          let _ = timestamps.push(core::time::Duration::from_micros(timer_value_at_termination.get_raw_value() as u64));
                //          hprintln!("TimerCaptureResultAvailable::TimerTimeout: {}, lvl:{:?}, N={}", timer_value_at_termination.get_raw_value(), init_level, timestamps.len()).ok();
                //      }
                //  }
                //  //  First period is skipped so, we start off with the second period and thus oposite level:
                //  let level = match init_level {
                //      true => InitLevel::Low,
                //      false => InitLevel::High,
                //  };
                //  container.extend(differences_reverse.iter().map(|v| CapturedEdgePeriod::FallingToFalling(*v)));
                //  (self, Ok((container, CaptureTypeEdges::RisingEdge)))

                match capture_result {
                    TimerCaptureResultAvailable::DmaPollReady(timer_capture_data) | TimerCaptureResultAvailable::TimerTimeout(timer_capture_data) => {
                        let timestamps = timer_capture_data.get_data();
                        if timestamps.len() < 3 {
                            hprintln!("TimerCaptureResultAvailable::[dma/timeout] timestamps: {:?}", timestamps).ok();
                            return (self, Err(CaptureError::GenericError));
                        }
                        container.extend(timestamps.iter().take(1).map(|v| CapturedEdgePeriod::StartToFirstRising(*v)));
                        //  The middle part without first and last, is RisignToRising:
                        container.extend(timestamps.iter().skip(1).take(timestamps.len()-2).map(|v| CapturedEdgePeriod::RisingToRising(*v)));
                        //  ... and the last one is RisingToStop:
                        container.extend(timestamps.iter().skip(timestamps.len()-1).map(|v| CapturedEdgePeriod::RisingToStop(*v)));
                        //  hprintln!("TimerCaptureResultAvailable::TimerTimeout: {:?}, N={}", timestamps, timestamps.len()).ok();
                        hprintln!("TimerCaptureResultAvailable::[dma/timeout] last: {:?}", timestamps.iter().last()).ok();
                    }
                }

                (self, Ok((container, CaptureTypeEdges::RisingEdge)))
            }
            else {
                return (self, Err(CaptureError::GenericError));
            }
        }
    }

    pub struct AtsamdGpioEdgeTriggerDev {
        pin_tx: GpioPin<PA17, PushPullOutput>,
    }

    impl AtsamdGpioEdgeTriggerDev
    {
        pub fn new(mut pin: GpioPin<PA17, PushPullOutput>) -> Self {
            pin.set_high().unwrap(); //  idle state
            Self{pin_tx: pin}
        }
    }

    impl EdgeTriggerInterface for AtsamdGpioEdgeTriggerDev {
        type OutputSelf = Self;
        async fn trigger( mut self, iterator: impl Iterator<Item = bool>,
                          period: core::time::Duration) -> (Self, Result<(), TriggerError>) {

            for (_idx, value) in iterator.enumerate() {
                if value
                {
                    self.pin_tx.set_high().unwrap(); //  idle state
                }
                else
                {
                    self.pin_tx.set_low().unwrap(); //  idle state
                }

                Mono::delay(MicrosDuration::<u32>::from_ticks(period.as_micros().try_into().unwrap()).convert()).await;
            }
            //  Always success:
            (self, Ok(()))
        }
    }

    pub(super) struct AtsamdTimeDriver {}

    impl AtsamdTimeDriver {
        pub(super) fn new() -> Self {
            Self {}
        }
    }

    impl TimeBaseRef for AtsamdTimeDriver {
        fn now(&self) -> Instant {
            //  Instant { ticks: 0 }
            todo!()
        }
    }

}

#[inline]
pub fn check_and_clear_interrupts(flags: InterruptFlags) -> InterruptFlags {
    let mut cleared = 0;
    let tc4 = unsafe { crate::pac::Peripherals::steal().tc4 };
    tc4.count8().intflag().modify(|r, w| {
        cleared = r.bits() & flags.into_bytes()[0];
        unsafe { w.bits(flags.into_bytes()[0]) }
    });
    InterruptFlags::from_bytes([cleared])
}

#[embassy_executor::task]
async fn toggle_pin_task(mut toggle_pin: GpioPin<PA17, Output<PushPull>>) {
    loop {
        toggle_pin.toggle().unwrap();
        Mono::delay(MillisDuration::<u32>::from_ticks(200).convert()).await;
    }
}

mod timer_data_set {
use crate::hal::pwm_wg::{PwmWg4Future, PwmWg2Future};
//  use super::bsp;
use crate::bsp::pac;
use crate::bsp::hal::{
    time::Hertz,
    timer_capture_waveform::{TimerCaptureBaseTrait, TimerCapture4Future, TimerCapture4, TimerCapture2Future, TimerCapture2},
    dmac, dmac::ReadyFuture,
    pwm_wg::{PwmWg4, PwmWg2, PwmBaseTrait},
    pwm::{TC4Pinout, TC2Pinout, PinoutNewTrait},
    gpio::{Pin, AnyPin, Output, Input, Alternate, OutputConfig, Pins, PushPull, PushPullOutput, Floating},
    gpio::{E, PB08, PB09, PA16, PA17},
};
use crate::pac::Mclk;

pub(super) struct PinoutSpecificDataImplTc4 {}

impl super::boiler_implementation::CreatePwmPinout for PinoutSpecificDataImplTc4 {
    type PinTxId = PB09;
    type PinRxId = PB08;
    type PinTx = Pin<Self::PinTxId, Output<PushPull>>;
    type PinRx = Pin<Self::PinRxId, Alternate<E>>;
    type PinoutTx = TC4Pinout<Self::PinTxId>;
    type PinoutRx = TC4Pinout<Self::PinRxId>;
    type DmaChannel = dmac::Channel<dmac::Ch0, ReadyFuture>;
    type PwmWg = PwmWg4Future<Self::PinTxId, Self::DmaChannel>;
    type TimerCaptureFuture = TimerCapture4Future<Self::PinRxId, Self::DmaChannel>;
    type TimerCaptureBase = TimerCapture4<Self::PinRxId>;
    type PwmBase = PwmWg4<Self::PinTxId>;
    type Timer = pac::Tc4;

 }
pub(super) struct PinoutSpecificDataImplTc2 {}

impl super::boiler_implementation::CreatePwmPinout for PinoutSpecificDataImplTc2 {
    type PinTxId = PA17;
    type PinRxId = PA16;
    type PinTx = Pin<Self::PinTxId, Output<PushPull>>;
    type PinRx = Pin<Self::PinRxId, Alternate<E>>;
    type PinoutTx = TC2Pinout<Self::PinTxId>;
    type PinoutRx = TC2Pinout<Self::PinRxId>;
    type DmaChannel = dmac::Channel<dmac::Ch1, ReadyFuture>;
    type PwmWg = PwmWg2Future<Self::PinTxId, Self::DmaChannel>;
    type TimerCaptureFuture = TimerCapture2Future<Self::PinRxId, Self::DmaChannel>;
    type TimerCaptureBase = TimerCapture2<Self::PinRxId>;
    type PwmBase = PwmWg2<Self::PinTxId>;
    type Timer = pac::Tc2;

 }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    use boiler_implementation::AtsamdEdgeTriggerCapture;

    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    //  let core = CorePeripherals::take().unwrap();

    let pins = Pins::new(peripherals.port);
    let dev_dependency_tx_simulation_pin: GpioPin<PA17, Output<PushPull>> = pins.pa17.into_push_pull_output();
    let dev_dependency_rx_simulation_pin: GpioPin<PA16, Input<PullUp>> = pins.pa16.into_pull_up_input();
    let receiver = CHANNEL.receiver();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    //  let mut delay = Delay::new(core.SYST, &mut clocks);
    let freq: FugitHertz<u32> = clocks.gclk0().into();
    Mono::start(core.SYST, freq.to_Hz());

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);
    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(DmacIrqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();
    // Initialize DMA Channels 0 and 1
    let mut channel0 = channels.0.init(PriorityLevel::Lvl0);
    let mut channel1 = channels.1.init(PriorityLevel::Lvl0);

    let tc2_timer = peripherals.tc2;

    //  #[cfg(feature = "use_opentherm")]
    //  spawner.spawn(full_boiler_opentherm_simulation(dev_dependency_tx_simulation_pin,
    //      dev_dependency_rx_simulation_pin,
    //      tc2_timer, channel1,
    //      &mut peripherals.mclk,
    //      &clocks.tc2_tc3(&gclk0).unwrap(),
    //  )).unwrap();
    //  #[cfg(feature = "use_opentherm")]
    //  spawner.spawn(simulate_opentherm_tx(dev_dependency_tx_simulation_pin, receiver)).unwrap();


    let mut pwm_tx_pin = pins.pb09.into_push_pull_output();
    //  Set initial level of OpenTherm bus to high:
    pwm_tx_pin.set_high().unwrap();

    let pwm_rx_pin = pins.pb08.into_pull_up_input();
    //  let _pwm_tx_pin = pins.pb09.into_alternate::<E>();
    let tc4_timer = peripherals.tc4;

    let async_lambda_delay = || async move {
        Mono::delay(MillisDuration::<u32>::from_ticks(50).convert()).await;
    };
    async_lambda_delay().await;

    //  let mut user_led: bsp::UserLed = pin_alias!(pins.user_led).into();
    let mut user_led: UserLed = pins.pa15.into();
    //  user_led.toggle().unwrap();

    //  let sets = bsp::Pins::new(peripherals.port).split();
    //  let mut user_led = sets.user_led.into_push_pull_output();
    user_led.set_low().unwrap();

    //  spawner.spawn(print_capture_timer_state_task()).unwrap();

    let pinout_specific_data = timer_data_set::PinoutSpecificDataImplTc4{};

    #[cfg(feature = "use_opentherm")]
    let mut edge_trigger_capture_dev =
        boiler_implementation::AtsamdEdgeTriggerCapture::<_, _, {boiler_implementation::VEC_SIZE_CAPTURE}>::new_with_default(
            pwm_tx_pin,
            pwm_rx_pin,
            tc4_timer,
            &mut peripherals.mclk,
            clocks.tc4_tc5(&gclk0).unwrap().freq(),
            pinout_specific_data,  // determines all the types
            channel0,
        );

    let pinout_specific_data_tc2 = timer_data_set::PinoutSpecificDataImplTc2{};

    #[cfg(feature = "use_opentherm")]
    let mut edge_trigger_capture_simulation_device =
        boiler_implementation::AtsamdEdgeTriggerCapture::<timer_data_set::PinoutSpecificDataImplTc2>::new_with_default(
            dev_dependency_tx_simulation_pin,
            dev_dependency_rx_simulation_pin,
            tc2_timer,
            &mut peripherals.mclk,
            clocks.tc2_tc3(&gclk0).unwrap().freq(),
            pinout_specific_data_tc2,
            channel1,
        );

    //  let _time_driver = boiler_implementation::AtsamdTimeDriver::new();
    //  let mut boiler_controller = BoilerControl::new(edge_trigger_capture_dev, time_driver);
    //  let _ = boiler_controller.set_point(Temperature::Celsius(16));

    //  Driver bringup temporary code:
    //  Idea is to configure additional PIN and connect it to the TC4 timer to capture the signal
    //  Pin toggle can be done in independent task
    //  spawner.spawn(toggle_pin_task(dev_dependency_tx_simulation_pin)).unwrap();

    hprintln!("main:: loop{} start:").ok();

    let (mut edge_trigger_capture_dev, _) = edge_trigger_capture_dev.send_open_therm_message(
                    OpenThermMessage::try_new_from_u32(0b0_000_0000_00000001_00100101_00000000_u32).unwrap()).await;

    let mut edge_trigger_capture_dev = edge_trigger_capture_dev.transition_to_capture_capable_device();
    let sender_trigger_tx_sequence = CHANNEL.sender();

    Mono::delay(MillisDuration::<u32>::from_ticks(5000).convert()).await;

    let mut count_all = 0;
    let mut count_success = 0;
    loop {
        //  Test one round of TX(simulation) -> RX(production)
        let tx_async_simulation =  async {
            sender_trigger_tx_sequence.send(SignalTxSimulation::Ready_).await;
            let (device, result) =
                edge_trigger_capture_simulation_device.send_open_therm_message(
                    OpenThermMessage::try_new_from_u32(0b0_000_0000_00000001_00100101_00000000_u32).unwrap()).await;
            device
        };

        let rx_async = async |&count_all| {
            let dur = Duration::from_millis(100);
            let start_time = Mono::now();
            let (tx_device, result) =
                edge_trigger_capture_dev.
                    listen_open_therm_message()/* -> (Self, Result<OpenThermMessage, Error>)*/.await;
            let duration = Mono::now() - start_time;
            match result {
                Ok(message) => {
                    count_success += 1;
                    hprintln!("Capture finished with opentherm message: {}, took: {}, rate: {}/{}", message, duration, count_success, count_all).ok();
                },
                Err(e) => {
                    hprintln!("Capture finished with error on OpenThermMessage: {}, took: {}, rate: {}/{}", e, duration, count_success, count_all).ok(); }
            }
            tx_device
        };

        count_all += 1;
        hprintln!("start: {}", count_all).ok();
        //  TODO: how to join the two futures and return devices as a result after execution?
        let (tx_result, rx_result) =
            embassy_futures::join::join(tx_async_simulation, rx_async(&count_all)).await;
        hprintln!("stop: {}", count_all).ok();

        //  TODO: Count successes and errors.

        edge_trigger_capture_dev = rx_result;
        edge_trigger_capture_simulation_device = tx_result;
    }

    //  let mut edge_trigger_capture_dev = rx_result;

    //  let time_d = core::time::Duration::from_millis(100);
    //  let time_d :u32 = time_d.as_millis().try_into().unwrap();
    //  hprintln!("Start Capture: {}", time_d).ok();

    //  let mut count_iterations: u32 = 0;
    //  loop {
    //      Mono::delay(MillisDuration::<u32>::from_ticks(200).convert()).await;
    //      let device = edge_trigger_capture_dev;
    //      //  hprintln!("Wait long before starting the capture").ok();
    //      //  Mono::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    //      hprintln!("Start Capture {}", count_iterations).ok();
    //      let dur = Duration::from_millis(100);
    //      // trigger gpio simulation of the OpenTherm TX message
    //      sender_trigger_tx_sequence.send(SignalTxSimulation::Ready_).await;
    //      let (rx_device, result) =
    //          device.start_capture(dur, dur).await;

    //      if let Ok((level, vector)) = result {
    //          hprintln!("Capture finished with: {}", vector.len()).ok();
    //          let differences: Vec<u128, 128> = vector
    //              .iter()
    //              .zip(vector.iter().skip(1))
    //              .map(|(a, b)| if b > a {b.as_micros() - a.as_micros()} else {0}).collect();

    //          //  for (i, v) in differences.into_iter().enumerate() {
    //          //      hprintln!("{}:{} us", i, v).ok();
    //          //  }
    //      }
    //      hprintln!("Finish Capture {}", count_iterations).ok();
    //      Mono::delay(MillisDuration::<u32>::from_ticks(50).convert()).await;

    //      hprintln!("Start FullOpentherm Capture {}", count_iterations).ok();
    //      let dur = Duration::from_millis(100);
    //      // trigger gpio simulation of the OpenTherm TX message
    //      sender_trigger_tx_sequence.send(SignalTxSimulation::Ready_).await;
    //      let (rx_device, result) =
    //          rx_device.listen_open_therm_message().await;
    //      if let Ok(message) = result {
    //          hprintln!("Capture finished with opentherm message: {}", message).ok();
    //      }
    //      else
    //      {
    //          //  TODO: extend the analysis of what is wrong with this capture here:
    //          hprintln!("Capture finished with error on OpenThermMessage").ok();
    //      }
    //      hprintln!("Finish Capture {}", count_iterations).ok();
    //      Mono::delay(MillisDuration::<u32>::from_ticks(50).convert()).await;

    //      hprintln!("Start Trigger {}", count_iterations).ok();
    //      let tx_device = rx_device.transition_to_trigger_capable_device();
    //      let (tx_device, result) =
    //          tx_device.send_open_therm_message(OpenThermMessage::try_new_from_u32(0b0_000_0000_00000001_00100101_00000000_u32).unwrap()).await;

    //      edge_trigger_capture_dev = tx_device.transition_to_capture_capable_device();
    //      //  let _ = boiler_controller.process().await.unwrap();

    //      user_led.toggle().unwrap();
    //      count_iterations += 1;
    //  }

    //  let _ot_rx: GpioPin<_, PullUpInterrupt> = pins.pb08.into(); // D0
    //                                                              //  let pb_09_ot_tx: GpioPin<_, PushPullOutput> = pins.pb09.into(); // D1
    //                                                              //  let capture_device = RpEdgeCapture::new(async_input);
    //                                                              //  let mut open_therm_bus = AtsamdEdgeTriggerCapture::new(pb_09_ot_tx);
    //  let example_vector = heapless::Vec::<bool, 13>::from_slice(&[
    //      true, true, false, true, false, true, false, false, true, false, false,
    //  ])
    //  .unwrap();
    //  //  let _ = open_therm_bus.trigger(example_vector.into_iter(), Duration::from_millis(100));

    //  #[cfg(feature = "use_opentherm")]
    //  let time_driver = AtsamdTimeDriver::new();
    //  //  let mut boiler_controller = BoilerControl::new(open_therm_bus, time_driver);
    //  //  let _ = boiler_controller.set_point(Temperature::Celsius(16));
    //  //  let _ = boiler_controller.enable_ch(CHState::Enable(true));
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
pub struct InterruptFlags {
    /// Overflow
    pub ovf: bool,
    /// Err
    pub err: bool,
    #[skip]
    _reserved: B6,
}

impl Default for InterruptFlags {
    fn default() -> Self {
        Self::new()
    }
}

// Dummy Waker implementation for no_std environment.
static RAW_WAKER_VTABLE: core::task::RawWakerVTable = core::task::RawWakerVTable::new(
    |ptr: *const ()| RawWaker::new(ptr, &RAW_WAKER_VTABLE),
    |_: *const ()| {}, // Do nothing, for simulation
    |_: *const ()| {}, // Do nothing, for simulation
    |_: *const ()| {},
);

unsafe fn raw_waker(waker_ptr: *const ()) -> Waker {
    Waker::from_raw(RawWaker::new(waker_ptr, &RAW_WAKER_VTABLE))
}

/// The idea here is to use simpler to implement TX driver using gpio timer to ease on
/// implementation of the OpenTherm RX driver based on timer + DMA
#[embassy_executor::task]
async fn simulate_opentherm_tx(mut tx_pin: GpioPin<PA17, Output<PushPull>>,
    receiver: Receiver<'static, ThreadModeRawMutex, SignalTxSimulation, 64>)
{
    let hw_dev = boiler_implementation::AtsamdGpioEdgeTriggerDev::new(tx_pin);
    const DURATION_MS: u32 = 500;

    // Give it some time before fire-up the
    Mono::delay(MillisDuration::<u32>::from_ticks(50).convert()).await;
    //  TODO: implement heapless queue receiving requests
    let mut pass_dev_in_loop = hw_dev;
    loop {
        //  Comment this out to have on demand instead of periodic transfer:
        let _received = receiver.receive().await;

        //  hprintln!("channel::RX").ok();
        //  tx_pin.toggle().unwrap();
        //  Wait for the receiver to be ready, give it some time to setup the capture
        Mono::delay(MillisDuration::<u32>::from_ticks(10).convert()).await;
        //  The device implements the trigger interface, it shall implement send as well:
        let (dev, result) =
            pass_dev_in_loop.send_open_therm_message(OpenThermMessage::try_new_from_u32(0b0_000_0000_00000001_00100101_00000000_u32).unwrap()).await;
        pass_dev_in_loop = dev;
        //  Mono::delay(MillisDuration::<u32>::from_ticks(DURATION_MS).convert()).await;
    }
}

#[embassy_executor::task]
async fn print_capture_timer_state_task(/*mut uart_tx: UartFutureTxDuplexDma<Config<bsp::UartPads>, Ch1>*/)
{
    let tc4_readonly = unsafe { crate::pac::Peripherals::steal().tc4 };
    let count32 = tc4_readonly .count32();
    let dmac_readonly = unsafe { crate::pac::Peripherals::steal().dmac };
    //  let mut value_cc1 = 0x00u8;
    loop {
        //  Read this value:
        //  let vcc1 = tc4_readonly.count8().cc(1).read().bits();
        //  if vcc1 != value_cc1 {
        //      hprintln!("tc4.cc1:0x{:08X}", vcc1).ok();
        //      value_cc1 = vcc1;
        //  }

        //  uart_tx.write(b"Hello, world!").await.unwrap();
        //  defmt::info!("Sent 10 bytes");

        //  let mut delay = Delay::new(core.SYST, &mut clocks);
        { //  Read counter one by one to see if it is running:
            let _ = count32.ctrlbset().write(|w| w.cmd().readsync());
            let cnt_value = count32.count().read().bits();
            let _ = count32.ctrlbset().write(|w| w.cmd().readsync());
            let cn2_value = count32.count().read().bits();
            //  hprintln!("cnt:0x{:08X}, 0x{:08X}", cnt_value, cn2_value).ok();
        }

        hprintln!("tc4int:0x{:08X}", count32.intflag().read().bits()).ok();
        //  hprintln!("tc4cc0:0x{:08X}", count32.cc(0).read().bits()).ok();
        //hprintln!("tc4ctrla:0x{:08X}", count32.ctrla().read().bits()).ok();
        //hprintln!("tc4evctrl:0x{:08X}", count32.evctrl().read().bits()).ok();
        //  hprintln!("tc4per:0x{:08X}", tc4_readonly.count8().per().read().bits()).ok();
        //  hprintln!("dmaact:0x{:08X}", dmac_readonly.active().read().bits()).ok();
        // let btcnt = dmac_readonly.active().read().btcnt() ).ok();
        //  hprintln!("dmaact:0x{:08X}", dmac_readonly.active().read().btcnt().bits() ).ok();
        //hprintln!("dmactrl:0x{:08X}", dmac_readonly.ctrl().read().bits()).ok();
        //hprintln!("dmabusy:0x{:08X}", dmac_readonly.busych().read().bits()).ok();
        //hprintln!("dmachint:0x{:08X}", dmac_readonly.intstatus().read().bits()).ok();
        //hprintln!("dmachintpend:0x{:08X}", dmac_readonly.intpend().read().bits()).ok();
        //hprintln!("ch[0]chctrla:0x{:08X}", dmac_readonly.channel(0).chctrla().read().bits()).ok();
        //hprintln!("ch[0]chint:0x{:08X}", dmac_readonly.channel(0).chintflag().read().bits()).ok();
        //hprintln!("ch[0]chstat:0x{:08X}", dmac_readonly.channel(0).chstatus().read().bits()).ok();

        //  let flags_to_check = InterruptFlags::new().with_ovf(true).with_err(true);
        //  if check_and_clear_interrupts(flags_to_check).ovf() {
        //      //  hprintln!("Overflow detected").ok();
        //  }

        //  delay.delay_ms(200u16);
        Mono::delay(MillisDuration::<u32>::from_ticks(2000).convert()).await;
    }
}
#[embassy_executor::task]
async fn print_timer_state_task(/*mut uart_tx: UartFutureTxDuplexDma<Config<bsp::UartPads>, Ch1>*/)
{
    let tc4_readonly = unsafe { crate::pac::Peripherals::steal().tc4 };
    let dmac_readonly = unsafe { crate::pac::Peripherals::steal().dmac };
    let mut value_cc1 = 0x00u8;
    loop {
        //  Read this value:
        let vcc1 = tc4_readonly.count8().cc(1).read().bits();
        if vcc1 != value_cc1 {
            hprintln!("tc4.cc1:0x{:08X}", vcc1).ok();
            value_cc1 = vcc1;
        }

        //  uart_tx.write(b"Hello, world!").await.unwrap();
        //  defmt::info!("Sent 10 bytes");

        //  let mut delay = Delay::new(core.SYST, &mut clocks);
        let _ = tc4_readonly
            .count8()
            .ctrlbset()
            .write(|w| w.cmd().readsync());
        let cnt_value = tc4_readonly.count8().count().read().bits();
        let _ = tc4_readonly
            .count8()
            .ctrlbset()
            .write(|w| w.cmd().readsync());
        let cn2_value = tc4_readonly.count8().count().read().bits();

        hprintln!("cnt:0x{:08X}, 0x{:08X}", cnt_value, cn2_value).ok();
        //  hprintln!(
        //      "tc4int:0x{:08X}, cc1:0x{:08X}",
        //      tc4_readonly.count8().intflag().read().bits(),
        //      tc4_readonly.count8().cc(1).read().bits()
        //  )
        //  .ok();
        //  hprintln!("tc4per:0x{:08X}", tc4_readonly.count8().per().read().bits()).ok();
        //  hprintln!("dma:0x{:08X}", dmac_readonly.active().read().bits()).ok();

        let flags_to_check = InterruptFlags::new().with_ovf(true).with_err(true);
        if check_and_clear_interrupts(flags_to_check).ovf() {
            //  hprintln!("Overflow detected").ok();
        }

        //  delay.delay_ms(200u16);
        Mono::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    }
}

#[embassy_executor::task]
async fn full_boiler_opentherm_simulation(tx_pin: GpioPin<PA17, Output<PushPull>>, mut rx_pin: GpioPin<PA16, Input<Floating>>,
    timer: pac::Tc2, dma_channel: hal::dmac::Channel<hal::dmac::Ch1, ReadyFuture>,
    mclk: &'static mut Mclk,
    timer_clocks: &'static Tc2Tc3Clock)
{
    // let mut edge_trigger_capture_dev =
    //     boiler_implementation::AtsamdEdgeTriggerCapture::new_with_default(
    //         tx_pin,
    //         rx_pin,
    //         timer,
    //         &mclk,
    //         &timer_clocks,
    //         dma_channel,
    //     );
    loop {
        Mono::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    }
}

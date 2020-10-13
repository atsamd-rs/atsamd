use crate::clock;
use crate::target_device::{evsys, EVSYS, PM};

pub struct EventSys {
    evsys: EVSYS,
}

#[derive(Clone, Copy)]
pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    #[cfg(feature = "samd21")]
    Channel6 = 6,
    #[cfg(feature = "samd21")]
    Channel7 = 7,
    #[cfg(feature = "samd21")]
    Channel8 = 8,
    #[cfg(feature = "samd21")]
    Channel9 = 9,
    #[cfg(feature = "samd21")]
    Channel10 = 10,
    #[cfg(feature = "samd21")]
    Channel11 = 11,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum User {
    DMAC_CH0 = 0x0,
    DMAC_CH1 = 0x1,
    DMAC_CH2 = 0x2,
    DMAC_CH3 = 0x3,

    TC3 = 0x12,
    TC4 = 0x13,
    TC5 = 0x14,
    TC6 = 0x15,
    TC7 = 0x16,

    ADC_START = 0x17,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Generator {
    NONE = 0x0,
    RTC_PER0 = 0x4,
    RTC_PER1 = 0x5,
    RTC_PER2 = 0x6,
    RTC_PER3 = 0x7,
    RTC_PER4 = 0x8,
    RTC_PER5 = 0x9,
    RTC_PER6 = 0xA,
    RTC_PER7 = 0xB,
    EXTINT2 = 0xE,
    DMAC_CH0 = 0x1E,
    DMAC_CH1 = 0x1F,
    DMAC_CH2 = 0x20,
    DMAC_CH3 = 0x21,
    TC3_OVF = 0x33,
    TC3_MC0 = 0x34,
    TC3_MC1 = 0x35,
    TC4_OVF = 0x36,
    TC4_MC0 = 0x37,
    TC4_MC1 = 0x38,
    TC5_OVF = 0x39,
    TC5_MC0 = 0x3A,
    TC5_MC1 = 0x3B,
}

pub enum EdgeSelector {
    Rising,
    Falling,
    Both,
}

impl Into<evsys::channel::EDGSEL_A> for EdgeSelector {
    fn into(self) -> evsys::channel::EDGSEL_A {
        match self {
            EdgeSelector::Rising => evsys::channel::EDGSEL_A::RISING_EDGE,
            EdgeSelector::Falling => evsys::channel::EDGSEL_A::FALLING_EDGE,
            EdgeSelector::Both => evsys::channel::EDGSEL_A::BOTH_EDGES,
        }
    }
}

pub trait Clock {}

macro_rules! ev_clock {
	($($Clock:ident,)+) => (
		$(
			impl Clock for clock::$Clock {}
		)+
	)
}

ev_clock! {
    Evsys0Clock,
    Evsys1Clock,
    Evsys2Clock,
    Evsys3Clock,
    Evsys4Clock,
    Evsys5Clock,
}

#[cfg(feature = "samd21")]
ev_clock! {
    Evsys6Clock,
    Evsys7Clock,
    Evsys8Clock,
    Evsys9Clock,
    Evsys10Clock,
    Evsys11Clock,
}

// TODO: enforce that the correct clock is passed for each event
pub enum Path<'a> {
    Asynchronous,
    Synchronous(&'a dyn Clock, EdgeSelector),
    Resynchronized(&'a dyn Clock, EdgeSelector),
}

impl EventSys {
    pub fn init(pm: &mut PM, evsys: EVSYS) -> Self {
        pm.apbcmask.modify(|_, w| w.evsys_().set_bit());

        Self { evsys }
    }

    pub fn connect(&mut self, channel: Channel, path: Path, generator: Generator, user: User) {
        self.subscribe(channel, user);
        self.publish(channel, path, generator);
    }

    pub fn subscribe(&mut self, channel: Channel, user: User) {
        self.evsys.user.write(|w| unsafe {
            w.channel().bits(channel as u8 + 1);
            w.user().bits(user as u8)
        });
    }

    pub fn publish(&mut self, channel: Channel, path: Path, generator: Generator) {
        self.evsys.channel.write(|w| unsafe {
            w.channel().bits(channel as u8);

            match path {
                Path::Asynchronous => {
                    w.path().asynchronous();
                }
                Path::Synchronous(_, edge_sel) => {
                    w.path().synchronous();
                    w.edgsel().variant(edge_sel.into());
                }
                Path::Resynchronized(_, edge_sel) => {
                    w.path().resynchronized();
                    w.edgsel().variant(edge_sel.into());
                }
            }

            w.evgen().bits(generator as u8)
        });
    }

    pub fn enable_event_detected_interrupt(&mut self, channel: Channel) {
        self.evsys.intenset.write(|w| match channel {
            Channel::Channel0 => w.evd0().set_bit(),
            Channel::Channel1 => w.evd1().set_bit(),
            Channel::Channel2 => w.evd2().set_bit(),
            Channel::Channel3 => w.evd3().set_bit(),
            Channel::Channel4 => w.evd4().set_bit(),
            Channel::Channel5 => w.evd5().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel6 => w.evd6().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel7 => w.evd7().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel8 => w.evd8().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel9 => w.evd9().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel10 => w.evd10().set_bit(),
            #[cfg(feature = "samd21")]
            Channel::Channel11 => w.evd11().set_bit(),
        });
    }

    pub fn trigger(&mut self, channel: Channel) {
        self.evsys.channel.write(|w| unsafe {
            w.channel().bits(channel as u8);
            w.swevt().set_bit()
        });
    }
}

#![allow(dead_code)]

use generic_array::{ArrayLength, GenericArray};

use crate::gpio::*;
use crate::target_device::{Interrupt, GMAC, MCLK, NVIC};

#[cfg(feature = "min-samd51n")]
pub struct MiiPadout<IOSet> {
    _gtxck: Pa14<PfL>,
    _gtxen: Pa17<PfL>,
    _gtx0: Pa18<PfL>,
    _gtx1: Pa19<PfL>,
    _gtx2: Pc16<PfL>,
    _gtx3: Pc17<PfL>,
    _gtxer: Pc19<PfL>,
    _grxck: Pc18<PfL>,
    _grxdv: Pc20<PfL>,
    _grx0: Pa13<PfL>,
    _grx1: Pa12<PfL>,
    _grx2: Pc15<PfL>,
    _grx3: Pc14<PfL>,
    _grxer: Pa15<PfL>,
    _gcrs: Pa16<PfL>,
    _gcol: Pc21<PfL>,
    _ioset: IOSet,
    /* _GMDC: GMDC,
     * _GMDIO: GMDIO, */
}

impl<IOSet> MiiPadout<IOSet> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: &mut Port,
        _gtxck: Pa14<Input<Floating>>,
        _gtxen: Pa17<Input<Floating>>,
        _gtx0: Pa18<Input<Floating>>,
        _gtx1: Pa19<Input<Floating>>,
        _gtx2: Pc16<Input<Floating>>,
        _gtx3: Pc17<Input<Floating>>,
        _gtxer: Pc19<Input<Floating>>,
        _grxck: Pc18<Input<Floating>>,
        _grxdv: Pc20<Input<Floating>>,
        _grx0: Pa13<Input<Floating>>,
        _grx1: Pa12<Input<Floating>>,
        _grx2: Pc15<Input<Floating>>,
        _grx3: Pc14<Input<Floating>>,
        _grxer: Pa15<Input<Floating>>,
        _gcrs: Pa16<Input<Floating>>,
        _gcol: Pc21<Input<Floating>>,
        _ioset: IOSet,
    ) -> Self {
        MiiPadout {
            _gtxck: _gtxck.into_function(port),
            _gtxen: _gtxen.into_function(port),
            _gtx0: _gtx0.into_function(port),
            _gtx1: _gtx1.into_function(port),
            _gtx2: _gtx2.into_function(port),
            _gtx3: _gtx3.into_function(port),
            _gtxer: _gtxer.into_function(port),
            _grxck: _grxck.into_function(port),
            _grxdv: _grxdv.into_function(port),
            _grx0: _grx0.into_function(port),
            _grx1: _grx1.into_function(port),
            _grx2: _grx2.into_function(port),
            _grx3: _grx3.into_function(port),
            _grxer: _grxer.into_function(port),
            _gcrs: _gcrs.into_function(port),
            _gcol: _gcol.into_function(port),
            _ioset,
        }
    }
}

pub struct RmiiPadout<IOSet> {
    _gtxck: Pa14<PfL>,
    _gtxen: Pa17<PfL>,
    _gtx0: Pa18<PfL>,
    _gtx1: Pa19<PfL>,
    #[cfg(feature = "same53j")]
    _grxdv: Pa16<PfL>,
    #[cfg(not(feature = "same53j"))]
    _grxdv: Pc20<PfL>,
    _grx0: Pa13<PfL>,
    _grx1: Pa12<PfL>,
    _grxer: Pa15<PfL>,
    _ioset: IOSet,
}

impl<IOSet> RmiiPadout<IOSet> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: &mut Port,
        _gtxck: Pa14<Input<Floating>>,
        _gtxen: Pa17<Input<Floating>>,
        _gtx0: Pa18<Input<Floating>>,
        _gtx1: Pa19<Input<Floating>>,
        #[cfg(feature = "same53j")] _grxdv: Pa16<Input<Floating>>,
        #[cfg(not(feature = "same53j"))] _grxdv: Pc20<Input<Floating>>,
        _grx0: Pa13<Input<Floating>>,
        _grx1: Pa12<Input<Floating>>,
        _grxer: Pa15<Input<Floating>>,
        _ioset: IOSet,
    ) -> Self {
        RmiiPadout {
            _gtxck: _gtxck.into_function(port),
            _gtxen: _gtxen.into_function(port),
            _gtx0: _gtx0.into_function(port),
            _gtx1: _gtx1.into_function(port),
            _grxdv: _grxdv.into_function(port),
            _grx0: _grx0.into_function(port),
            _grx1: _grx1.into_function(port),
            _grxer: _grxer.into_function(port),
            _ioset,
        }
    }
}

pub enum GmacMode {
    Rmii = 0,
    Mii = 1,
}

pub trait GmacPadout {
    const MODE: GmacMode;
}
impl<IOSet> GmacPadout for MiiPadout<IOSet> {
    const MODE: GmacMode = GmacMode::Mii;
}
impl<IOSet> GmacPadout for RmiiPadout<IOSet> {
    const MODE: GmacMode = GmacMode::Rmii;
}

// type RxCallback = fn(Gmac<P: GmacPadout, RxBuf, TxBuf, RxCallback,
// TxCallback>);
pub struct Gmac<P: GmacPadout, RxBuf, TxBuf, RxCallback, TxCallback> {
    _padout: P,
    gmac: GMAC,
    rx_buffers: RxBuf,
    tx_buffers: TxBuf,
    rx_buf_index: usize,
    tx_buf_index: usize,
    last_tx_buf_index: usize,
    rx_callback: Option<RxCallback>,
    tx_callback: Option<TxCallback>,
}

impl<P, RxBuf, TxBuf, RxCallback, TxCallback> Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>
where
    P: GmacPadout,
    RxBuf: BufferSet<RxBufferDescriptor>,
    TxBuf: BufferSet<TxBufferDescriptor>,
    RxCallback: FnMut(&mut Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>),
    TxCallback: FnMut(&mut Gmac<P, RxBuf, TxBuf, RxCallback, TxCallback>),
{
    pub fn new(
        padout: P,
        mclk: &MCLK,
        gmac: GMAC,
        rx_buffers: RxBuf,
        tx_buffers: TxBuf,
        /* rx_callback: RxCallback,
         * tx_callback: TxCallback, */
    ) -> Self {
        mclk.ahbmask.modify(|_, w| w.gmac_().set_bit());
        mclk.apbcmask.modify(|_, w| w.gmac_().set_bit());

        gmac.ncr.write_with_zero(|w| {
            w.lbl().clear_bit(); // disable loopback local
            w.mpe().set_bit(); // management port enable
            w.westat().clear_bit(); // Disable write for Static Register
            w.bp().clear_bit(); // Disable Back pressure
            w.enpbpr().clear_bit(); // Disable PFC Priority-based Pause Reception
            w.txpbpf().clear_bit() // Disable PFC Priority-based Pause Frame
        });
        unsafe {
            gmac.ncfgr.write_with_zero(|w| {
                w.spd().set_bit(); // 100Mbps Speed
                w.fd().set_bit(); // Enable Full Duplex
                w.dnvlan().clear_bit(); // Do not discard Non-VLAN Frames
                w.jframe().clear_bit(); // Disable Jumbo Frames
                w.caf().clear_bit(); // Disable Copy All Frames
                w.nbc().clear_bit(); // Disable No broadcast
                w.mtihen().clear_bit(); // Disable Multicast Hash Enable
                w.unihen().clear_bit(); // Disable Unicast Hash Enable
                w.maxfs().set_bit(); // 1536 Maximum Frame Size
                w.rty().clear_bit(); // Disable Retry Test
                w.pen().clear_bit(); // Disable Pause Enable
                w.rxbufo().bits(0); // Receive Buffer Offset <0-3>
                w.lferd().clear_bit(); // Disable Length Field Error Frame Discard
                w.rfcs().clear_bit(); // Disable Remove FCS
                w.clk().bits(4) // MDC Clock Division 4 => 64
            });
        }

        match P::MODE {
            GmacMode::Rmii => gmac.ur.write_with_zero(|w| w.mii().clear_bit()),
            GmacMode::Mii => gmac.ur.write_with_zero(|w| w.mii().set_bit()),
        }

        unsafe {
            gmac.dcfgr.write_with_zero(|w| {
                w.fbldo().bits(4); // Fixed Burst Length for DMA Data Operations, 4 => Always use INCR4 AHB bursts
                w.esma().clear_bit(); // Disable Endian Swap Mode Enable for Management Descriptor Accesses
                w.espa().clear_bit(); // Disable Endian Swap Mode Enable for Packet Data Accesses
                w.rxbms().bits(3); // Receiver Packet Buffer Memory Size Select, 3 => 4KB
                w.txpbms().set_bit(); // Transmitter Packet Buffer Memory Size Select, 1 => 4KB
                w.txcoen().clear_bit(); // Disable Transmitter Checksum Generation Offload Enable
                w.drbs().bits(2); // DMA Receive Buffer Size <1-255>, multiples of 64, 2 => 128B
                w.ddrp().clear_bit() // Disable DMA Discard Received Packets
            });
        }

        // Disable Wake on LAN (for now...)
        gmac.wol.write_with_zero(|w| w);

        gmac.ipgs.write_with_zero(|w| {
            let mul = 1; // IPG Stretch Multiple <0-15>
            let div = 1; // IPG Stretch Divide <1-16>
            unsafe { w.fl().bits(mul << 8 | div) }
        });

        unsafe {
            gmac.tbqb
                .write_with_zero(|w| w.addr().bits(tx_buffers.address()));
            gmac.rbqb
                .write_with_zero(|w| w.addr().bits(rx_buffers.address()));
        }

        NVIC::mask(Interrupt::GMAC);
        NVIC::unpend(Interrupt::GMAC);
        unsafe {
            //TODO Should this really be here?
            NVIC::unmask(Interrupt::GMAC);
        }

        Gmac {
            _padout: padout,
            gmac,
            rx_buffers,
            tx_buffers,
            rx_buf_index: 0,
            tx_buf_index: 0,
            last_tx_buf_index: 0,
            rx_callback: None,
            tx_callback: None,
        }
    }

    pub fn enable(&mut self) {
        self.gmac.ncr.modify(|_, w| {
            w.rxen().set_bit();
            w.txen().set_bit()
        });
    }
    pub fn disable(&mut self) {
        self.gmac.ncr.modify(|_, w| {
            w.rxen().clear_bit();
            w.txen().clear_bit()
        });
    }

    pub fn gmac_handler(&mut self) {
        let tsr = self.gmac.tsr.read().bits();
        let rsr = self.gmac.rsr.read().bits();
        // Must be Clear ISR (Clear on read)
        self.gmac.isr.read().bits();

        if (tsr & (1 << 5)) == 1 << 5 {
            // Frame transmited
            unsafe {
                self.gmac.tsr.write_with_zero(|w| w.bits(tsr));
            }
            if self.tx_buffers.get_descriptor(self.tx_buf_index).used() {
                // Call transmitted callback
                let mut cb = self.tx_callback.take().unwrap();
                cb(self);
                self.tx_callback = Some(cb);
            }
        }

        if (rsr & (1 << 1)) == 1 << 1 {
            // Frame received
            // Call received callback
            let mut cb = self.rx_callback.take().unwrap();
            cb(self);
            self.rx_callback = Some(cb);
        }
        unsafe {
            self.gmac.rsr.write_with_zero(|w| w.bits(rsr));
        }
    }
    #[rustfmt::skip]
    fn _comments() {
        // hri_gmac_write_NCR_reg(dev->hw,
        //                     (CONF_GMAC_NCR_LBL ? GMAC_NCR_LBL : 0) | (CONF_GMAC_NCR_MPE ? GMAC_NCR_MPE : 0)
        //                         | (CONF_GMAC_NCR_WESTAT ? GMAC_NCR_WESTAT : 0) | (CONF_GMAC_NCR_BP ? GMAC_NCR_BP : 0)
        //                         | (CONF_GMAC_NCR_ENPBPR ? GMAC_NCR_ENPBPR : 0)
        //                         | (CONF_GMAC_NCR_TXPBPF ? GMAC_NCR_TXPBPF : 0));
        // hri_gmac_write_NCFGR_reg(
        //     dev->hw,
        //     (CONF_GMAC_NCFGR_SPD ? GMAC_NCFGR_SPD : 0) | (CONF_GMAC_NCFGR_FD ? GMAC_NCFGR_FD : 0)
        //         | (CONF_GMAC_NCFGR_DNVLAN ? GMAC_NCFGR_DNVLAN : 0) | (CONF_GMAC_NCFGR_JFRAME ? GMAC_NCFGR_JFRAME : 0)
        //         | (CONF_GMAC_NCFGR_CAF ? GMAC_NCFGR_CAF : 0) | (CONF_GMAC_NCFGR_NBC ? GMAC_NCFGR_NBC : 0)
        //         | (CONF_GMAC_NCFGR_MTIHEN ? GMAC_NCFGR_MTIHEN : 0) | (CONF_GMAC_NCFGR_UNIHEN ? GMAC_NCFGR_UNIHEN : 0)
        //         | (CONF_GMAC_NCFGR_MAXFS ? GMAC_NCFGR_MAXFS : 0) | (CONF_GMAC_NCFGR_RTY ? GMAC_NCFGR_RTY : 0)
        //         | (CONF_GMAC_NCFGR_PEN ? GMAC_NCFGR_PEN : 0) | GMAC_NCFGR_RXBUFO(CONF_GMAC_NCFGR_RXBUFO)
        //         | (CONF_GMAC_NCFGR_LFERD ? GMAC_NCFGR_LFERD : 0) | (CONF_GMAC_NCFGR_RFCS ? GMAC_NCFGR_RFCS : 0)
        //         | GMAC_NCFGR_CLK(CONF_GMAC_NCFGR_CLK) | (CONF_GMAC_NCFGR_DCPF ? GMAC_NCFGR_DCPF : 0)
        //         | (CONF_GMAC_NCFGR_RXCOEN ? GMAC_NCFGR_RXCOEN : 0) | (CONF_GMAC_NCFGR_EFRHD ? GMAC_NCFGR_EFRHD : 0)
        //         | (CONF_GMAC_NCFGR_IRXFCS ? GMAC_NCFGR_IRXFCS : 0) | (CONF_GMAC_NCFGR_IPGSEN ? GMAC_NCFGR_IPGSEN : 0)
        //         | (CONF_GMAC_NCFGR_RXBP ? GMAC_NCFGR_RXBP : 0) | (CONF_GMAC_NCFGR_IRXER ? GMAC_NCFGR_IRXER : 0));
        // hri_gmac_write_UR_reg(dev->hw, (CONF_GMAC_UR_MII ? GMAC_UR_MII : 0));
        // hri_gmac_write_DCFGR_reg(
        //     dev->hw,
        //     GMAC_DCFGR_FBLDO(CONF_GMAC_DCFGR_FBLDO) | (CONF_GMAC_DCFGR_ESMA ? GMAC_DCFGR_ESMA : 0)
        //         | (CONF_GMAC_DCFGR_ESPA ? GMAC_DCFGR_ESPA : 0) | GMAC_DCFGR_RXBMS(CONF_GMAC_DCFGR_RXBMS)
        //         | (CONF_GMAC_DCFGR_TXPBMS ? GMAC_DCFGR_TXPBMS : 0) | (CONF_GMAC_DCFGR_TXCOEN ? GMAC_DCFGR_TXCOEN : 0)
        //         | GMAC_DCFGR_DRBS(CONF_GMAC_DCFGR_DRBS) | (CONF_GMAC_DCFGR_DDRP ? GMAC_DCFGR_DDRP : 0));
        // hri_gmac_write_WOL_reg(dev->hw, 0);
        // hri_gmac_write_IPGS_reg(dev->hw, GMAC_IPGS_FL((CONF_GMAC_IPGS_FL_MUL << 8) | CONF_GMAC_IPGS_FL_DIV));

        // _mac_init_bufdescr(dev);

        // _gmac_dev = dev;
        // NVIC_DisableIRQ(GMAC_IRQn);
        // NVIC_ClearPendingIRQ(GMAC_IRQn);
        // NVIC_EnableIRQ(GMAC_IRQn);

        // hri_gmac_set_NCR_reg(dev->hw, GMAC_NCR_RXEN | GMAC_NCR_TXEN);
        // ethernet_phy_init(&ETHERNET_PHY_0_desc, &COMMUNICATION_IO, CONF_ETHERNET_PHY_0_IEEE8023_MII_PHY_ADDRESS);
        // #if CONF_ETHERNET_PHY_0_IEEE8023_MII_CONTROL_REG0_SETTING == 1
        //     ethernet_phy_write_reg(&ETHERNET_PHY_0_desc, MDIO_REG0_BMCR, CONF_ETHERNET_PHY_0_IEEE8023_MII_CONTROL_REG0);
        // #endif /* CONF_ETHERNET_PHY_0_IEEE8023_MII_CONTROL_REG0_SETTING */
    }
}

pub struct Ioset<GMDC, GMDIO>(GMDC, GMDIO);
pub trait IOSet: private::Sealed {}
impl<GMDC, GMDIO> private::Sealed for Ioset<GMDC, GMDIO> {}
impl<GMDC, GMDIO> IOSet for Ioset<GMDC, GMDIO> {}

impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pb14<PfL>>,
    GMDIO: IntoFunction<Pb15<PfL>>,
{
    pub fn new1(gmdc: GMDC, gmdio: GMDIO) -> impl IOSet {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pc11<PfL>>,
    GMDIO: IntoFunction<Pc12<PfL>>,
{
    pub fn new2(gmdc: GMDC, gmdio: GMDIO) -> impl IOSet {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pc22<PfL>>,
    GMDIO: IntoFunction<Pc23<PfL>>,
{
    pub fn new3(gmdc: GMDC, gmdio: GMDIO) -> impl IOSet {
        Ioset(gmdc, gmdio)
    }
}
impl<GMDC, GMDIO> Ioset<GMDC, GMDIO>
where
    GMDC: IntoFunction<Pa20<PfL>>,
    GMDIO: IntoFunction<Pa21<PfL>>,
{
    pub fn new4(gmdc: GMDC, gmdio: GMDIO) -> impl IOSet {
        Ioset(gmdc, gmdio)
    }
}

// IOSET    1       2       3       4
// GMDC     PB14    PC11    PC22    PA20
// GMDIO    PB15    PC12    PC23    PA21

#[repr(C, align(8))]
pub struct TxBufferDescriptor {
    address: u32,
    status: u32,
}
#[allow(clippy::len_without_is_empty)]
impl TxBufferDescriptor {
    pub fn address(&self) -> u32 {
        self.address
    }
    pub(crate) fn set_address(&mut self, address: u32) {
        self.address = address;
    }
    pub(crate) fn reset_status(&mut self) {
        self.status = 1 << 31; // only used bit
    }
    pub(crate) fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address |= 1 << 30;
        } else {
            self.address &= !(1 << 30);
        }
    }
    pub fn len(&self) -> u16 {
        (self.status & ((1 << 14) - 1)) as u16
    }
    pub fn last_buf(&self) -> bool {
        (self.status >> 16) & 1 == 1
    }
    pub fn no_crc(&self) -> bool {
        (self.status >> 17) & 1 == 1
    }
    pub fn checksum_err(&self) -> u8 {
        (self.status >> 20 & 0x7) as u8
    }
    pub fn lco(&self) -> bool {
        (self.status >> 26) & 1 == 1
    }
    pub fn exhausted(&self) -> bool {
        (self.status >> 27) & 1 == 1
    }
    pub fn error(&self) -> bool {
        (self.status >> 29) & 1 == 1
    }
    pub fn wrap(&self) -> bool {
        (self.status >> 30) & 1 == 1
    }
    pub fn used(&self) -> bool {
        (self.status >> 31) & 1 == 1
    }
}

#[repr(C, align(8))]
pub struct RxBufferDescriptor {
    address: u32,
    status: u32,
}
#[allow(clippy::len_without_is_empty)]
impl RxBufferDescriptor {
    pub fn ownership(&self) -> bool {
        self.address & 1 == 1
    }
    pub fn wrap(&self) -> bool {
        (self.address >> 1) & 1 == 1
    }
    pub fn address(&self) -> u32 {
        self.address >> 2
    }
    pub(crate) fn set_address(&mut self, address: u32) {
        assert!(address < (1 << 30));
        self.address = address & ((1 << 30) - 1);
    }
    pub(crate) fn reset_status(&mut self) {
        self.status = 0;
    }
    pub(crate) fn set_wrap(&mut self, wrap: bool) {
        if wrap {
            self.address |= 1 << 1;
        } else {
            self.address &= !(1 << 1);
        }
    }

    pub fn len(&self) -> u16 {
        (self.status & ((1 << 13) - 1)) as u16
    }
    pub fn fcs(&self) -> bool {
        (self.status >> 13) & 1 == 1
    }
    pub fn sof(&self) -> bool {
        (self.status >> 14) & 1 == 1
    }
    pub fn eof(&self) -> bool {
        (self.status >> 15) & 1 == 1
    }
    pub fn cfi(&self) -> bool {
        (self.status >> 16) & 1 == 1
    }
    pub fn vlan_priority(&self) -> u8 {
        (self.status >> 17 & 0x7) as u8
    }
    pub fn priority_detected(&self) -> bool {
        (self.status >> 20) & 1 == 1
    }
    pub fn vlan_detected(&self) -> bool {
        (self.status >> 21) & 1 == 1
    }
    pub fn type_id_match(&self) -> u8 {
        (self.status >> 22 & 0x3) as u8
    }
    pub fn checksumoffload(&self) -> bool {
        (self.status >> 24) & 1 == 1
    }
    pub fn addrmatch(&self) -> u8 {
        ((self.status >> 25) & 0x3) as u8
    }
    pub fn ext_addr_match(&self) -> bool {
        (self.status >> 27) & 1 == 1
    }
    pub fn uni_hash_match(&self) -> bool {
        (self.status >> 29) & 1 == 1
    }
    pub fn multi_hash_match(&self) -> bool {
        (self.status >> 30) & 1 == 1
    }
    pub fn boardcast_detect(&self) -> bool {
        (self.status >> 31) & 1 == 1
    }
}

pub trait BufferDescriptor: private::Sealed {
    fn init(&mut self, buf: &[u8], is_last: bool);
}
impl BufferDescriptor for RxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
}
impl BufferDescriptor for TxBufferDescriptor {
    fn init(&mut self, buf: &[u8], is_last: bool) {
        self.set_address(buf.as_ptr() as u32);
        self.reset_status();
        self.set_wrap(is_last);
    }
}

pub struct GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GenericArray<u8, Size>>,
    Size: ArrayLength<u8>,
{
    descriptors: GenericArray<T, Count>,
    buffers: GenericArray<GenericArray<u8, Size>, Count>,
}

impl<T, Count, Size> GmacBufferSet<T, Count, Size>
where
    T: BufferDescriptor,
    Count: ArrayLength<T> + ArrayLength<GenericArray<u8, Size>>,
    Size: ArrayLength<u8>,
{
    pub fn new(
        mut descriptors: GenericArray<T, Count>,
        buffers: GenericArray<GenericArray<u8, Size>, Count>,
    ) -> Self {
        let count = Count::to_usize();
        for (idx, (descriptor, buffer)) in descriptors.iter_mut().zip(buffers.iter()).enumerate() {
            descriptor.init(&buffer[..], idx >= count - 1);
        }
        GmacBufferSet {
            descriptors,
            buffers,
        }
    }

    pub fn get_buffer(&self, index: usize) -> &[u8] {
        &self.buffers[index][..]
    }
    pub fn get_descriptor(&self, index: usize) -> &T {
        &self.descriptors[index]
    }
}

pub trait BufferSet<T> {
    fn address(&self) -> u32;
    fn get_buffer(&self, index: usize) -> &[u8];
    fn get_descriptor(&self, index: usize) -> &T;
}
impl<
        T: BufferDescriptor,
        Count: ArrayLength<T> + ArrayLength<GenericArray<u8, Size>>,
        Size: ArrayLength<u8>,
    > BufferSet<T> for GmacBufferSet<T, Count, Size>
{
    fn address(&self) -> u32 {
        self.descriptors.as_ptr() as u32
    }
    fn get_buffer(&self, index: usize) -> &[u8] {
        &self.get_buffer(index)[..]
    }
    fn get_descriptor(&self, index: usize) -> &T {
        self.get_descriptor(index)
    }
}

mod private {
    use generic_array::{ArrayLength, GenericArray};
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for super::RxBufferDescriptor {}
    impl Sealed for super::TxBufferDescriptor {}
    impl<
            T: super::BufferDescriptor,
            Count: ArrayLength<T> + ArrayLength<GenericArray<u8, Size>>,
            Size: ArrayLength<u8>,
        > Sealed for super::GmacBufferSet<T, Count, Size>
    {
    }
}

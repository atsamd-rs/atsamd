#![allow(dead_code)]

use crate::target_device::{GMAC, MCLK};
mod buffer;
use buffer::*;
mod pad;
use pad::*;

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

        // NVIC::mask(Interrupt::GMAC);
        // NVIC::unpend(Interrupt::GMAC);
        // unsafe {
        //     //TODO Should this really be here?
        //     NVIC::unmask(Interrupt::GMAC);
        // }

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

    pub fn deinit(self) -> (P, GMAC, RxBuf, TxBuf) {
        unsafe {
            /* Disable all GMAC Interrupt */
            self.gmac.idr.write_with_zero(|w| w.bits(0xFFFFFFFF));
            /* Disable transmit/receive */
            self.gmac.ncr.write_with_zero(|w| w.bits(0));
        }
        // Disable Interrupt
        //crate::target_device::NVIC::mask(crate::target_device::Interrupt::GMAC);

        (self._padout, self.gmac, self.rx_buffers, self.tx_buffers)
    }

    pub fn set_rx_callback(&mut self, callback: RxCallback) {
        self.rx_callback = Some(callback)
    }
    pub fn set_tx_callback(&mut self, callback: TxCallback) {
        self.tx_callback = Some(callback)
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

    pub fn write(&mut self, buf: &[u8]) -> nb::Result<(), ()> {
        let descriptor = self.tx_buffers.get_descriptor(self.last_tx_buf_index);
        if descriptor.used() && !descriptor.last_buf() {
            // Set used flag from first descriptor to last descriptor, as DMA
            // only set the first used flag
            for idx in 0..TxBuf::COUNT {
                let mut pos = self.last_tx_buf_index + idx;
                if pos >= TxBuf::COUNT {
                    pos -= TxBuf::COUNT;
                }
                let descriptor = self.tx_buffers.get_descriptor_mut(pos);
                descriptor.set_used();
                if descriptor.last_buf() {
                    break;
                }
            }
        }

        if !self.tx_buffers.get_descriptor(self.rx_buf_index).used() {
            return Err(nb::Error::WouldBlock);
        }

        /* Check if have enough buffers, the first buffer already checked */
        if buf.len() > TxBuf::SIZE {
            for idx in 0..TxBuf::COUNT {
                let mut pos = self.tx_buf_index + idx;
                if pos >= TxBuf::COUNT {
                    pos -= TxBuf::COUNT;
                }

                let descriptor = self.tx_buffers.get_descriptor(pos);
                if !descriptor.used() {
                    return Err(nb::Error::WouldBlock);
                }

                if (buf.len() - (TxBuf::SIZE * idx)) < TxBuf::SIZE {
                    break;
                }
            }
        }
        self.last_tx_buf_index = self.tx_buf_index;

        /* Write data to transmit buffer */
        let mut len = buf.len();
        let mut buf = buf;
        for _ in 0..TxBuf::COUNT {
            let blen = len.min(TxBuf::SIZE);
            self.tx_buffers
                .get_buffer_mut(self.tx_buf_index)
                .copy_from_slice(&buf[..blen]);
            len -= blen;
            buf = &buf[blen..];

            let descriptor = self.tx_buffers.get_descriptor_mut(self.tx_buf_index);
            if len > 0 {
                /* Here the Used flag be set to zero */
                descriptor.reset_with_len(blen, false);
            } else {
                /* Here the Used flag be set to zero */
                descriptor.reset_with_len(blen, true);
            }

            self.tx_buf_index += 1;
            if self.tx_buf_index == TxBuf::COUNT {
                self.tx_buf_index = 0;
                self.tx_buffers
                    .get_descriptor_mut(TxBuf::COUNT - 1)
                    .set_wrap(true);
            }
            if len == 0 {
                break;
            }
        }

        /* Data synchronization barrier */
        cortex_m::asm::dsb();

        /* Active Transmit */
        self.gmac.ncr.modify(|_, w| w.tstart().set_bit());

        Ok(())
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
                if let Some(mut cb) = self.tx_callback.take() {
                    // Call transmitted callback
                    cb(self);
                    self.tx_callback = Some(cb);
                }
            }
        }

        if (rsr & (1 << 1)) == 1 << 1 {
            // Frame received
            if let Some(mut cb) = self.rx_callback.take() {
                // Call received callback
                cb(self);
                self.rx_callback = Some(cb);
            }
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

mod private {
    use generic_array::ArrayLength;
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for super::RxBufferDescriptor {}
    impl Sealed for super::TxBufferDescriptor {}
    impl<
            T: super::BufferDescriptor,
            Count: ArrayLength<T> + ArrayLength<super::GmacBuffer<Size>>,
            Size: ArrayLength<u8>,
        > Sealed for super::GmacBufferSet<T, Count, Size>
    {
    }
}

use crate::pac::{MCLK, TRNG};

use rand_core::{CryptoRng, RngCore};

#[cfg(feature = "unproven")]
use embedded_hal::blocking::rng::Read;

pub struct Trng(TRNG);

impl Trng {
    pub fn new(mclk: &mut MCLK, trng: TRNG) -> Trng {
        mclk.apbcmask.modify(|_, w| w.trng_().set_bit());
        trng.ctrla.modify(|_, w| w.enable().set_bit());
        Self(trng)
    }

    pub fn random(&self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(4) {
            chunk.copy_from_slice(&self.random_u32().to_le_bytes()[..chunk.len()]);
        }
    }

    pub fn random_u8(&self) -> u8 {
        self.random_u32() as u8
    }

    pub fn random_u16(&self) -> u16 {
        self.random_u32() as u16
    }

    pub fn random_u32(&self) -> u32 {
        while self.0.intflag.read().datardy().bit_is_clear() {}
        self.0.data.read().bits()
    }

    pub fn random_u64(&self) -> u64 {
        while self.0.intflag.read().datardy().bit_is_clear() {}
        let lower_half = self.0.data.read().bits() as u64;
        while self.0.intflag.read().datardy().bit_is_clear() {}
        let upper_half = self.0.data.read().bits() as u64;
        (upper_half << 32) | lower_half
    }
}

impl RngCore for Trng {
    fn next_u32(&mut self) -> u32 {
        self.random_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.random_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.random(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for Trng {}

#[cfg(feature = "unproven")]
impl Read for Trng {
    type Error = ();
    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.random(buffer);
        Ok(())
    }
}

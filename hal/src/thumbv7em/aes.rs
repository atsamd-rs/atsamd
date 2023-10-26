//! # AES - Advanced Encryption Standard
//!
//! # Hardware Features
//!
//! * Compliant with FIPS Publication 197, Advanced Encryption Standard (AES)
//! * 128/192/256 bit cryptographic key supported
//! * Encryption time of 57/67/77 cycles with 128-bit/192-bit/256-bit
//!   cryptographic key
//! * Five confidentiality modes of operation as recommended in NIST Special
//!   Publication 800-38A
//! * Electronic Code Book (ECB)
//! * Cipher Block Chaining (CBC)
//! * Cipher Feedback (CFB)
//! * Output Feedback (OFB)
//! * Counter (CTR)
//! * Supports Counter with CBC-MAC (CCM/CCM*) mode for authenticated encryption
//! * 8,16, 32, 64, 128-bit data sizes possible in CFB mode
//! * Galois Counter mode (GCM) encryption and authentication
//!
//! ## Throughput
//!
//! The relationship between the module's clock frequency and throughput (in
//! bytes per second) is given by:
//!
//! Clock Frequency = (Throughput/2) * (Nr+1) for 2 byte parallel processing
//! Clock Frequency = (Throughput/4) * (Nr+1) for 4 byte parallel processing
//!
//! # Start modes
//!
//! * Manual
//!   * Manually configuring all registers and processing starts when
//!     `CTRLB.START` is set
//! * Automatic (DMA)
//!   * Similar to manual mode, but starts automatically when correct number of
//!     input data registers is written, used by DMA.
//! * Last Output Data Mode (LOD)
//!   * Used to generate Message Authentication Code (MAC) on data in CCM mode.
//!     CCM combines counter mode for encryption and CBC-MAC generation for
//!     authentication.
//!
//! # Basic operation
//!
//! ## Peripheral setup
//!
//! 1. Enable `CLK_AES_APB` (default disabled) to clock AES peripheral
//! 2. If required, setup interrupts via NVIC
//!
//! Note: Register Control A (CTRLA) is Enabled-protected,
//! thus in order to modify CTRLA register AES must be disabled first.
//!
//! # RustCrypto backend
//!
//! Implements RustCrypto BlockCiphers traits for AES
//!
//! > **WARNING**
//! >
//! >AES Hardware peripheral is directly accessed, for each
//! >call to `encrypt` and `decrypt` the peripheral is reset and reconfigured.
//! >
//! > User must ensure that these two interfaces are not simultaneously used
//!
//! If high performance is required this might not be the most
//! efficient way, then using the hardware directly might be better.
//!
//! This provides the ability to use other ciphers of the RustCrypto family,
//! such as
//! * [Counter (CTR)][ctr]
//! * [Cipher-based Message Authentication Code (CMAC)][cmac]
//! * [Cipher Feedback (CFB)][cfb]
//!
//! [ctr]: https://docs.rs/ctr/
//! [cmac]: https://docs.rs/cmac
//! [cfb]: https://docs.rs/cfb-mode
//!
//! The examples from these crates can directly be run
//! provided that the Aes128, Aes192 or Aes256 type
//! comes from this implementation.
//!
//! See example directly from RustCrypto AES ECB:
//!
//! ```no_run
//!     use atsamd_hal::aes::*;
//!
//!     // AES RustCrypto Example
//!
//!     let key = GenericArray::from_slice(&[0u8; 16]);
//!     let mut block = aes::Block::default();
//!
//!     // Initialize cipher
//!     let cipher = atsamd_hal::aes::Aes128::new(key);
//!
//!     let block_copy = block;
//!
//!     // Encrypt block in-place
//!     cipher.encrypt_block(&mut block);
//!
//!     // And decrypt it back
//!     cipher.decrypt_block(&mut block);
//!     assert_eq!(block, block_copy);
//! ```

// Re-exports
pub use crate::pac::aes::ctrla::{
    AESMODESELECT_A, CFBSSELECT_A, CIPHERSELECT_A, KEYSIZESELECT_A, LODSELECT_A, STARTMODESELECT_A,
    XORKEYSELECT_A,
};

// Re-export Aes128 with hardware backing
// TODO Should all these items here be reexported? (Aes*, Aes*Enc, Aes*Dec)
#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
mod rustcrypto;

#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
pub use rustcrypto::{Aes128, Aes192, Aes256};

#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
pub use cipher::{
    consts::{U1, U16, U24, U32, U8},
    generic_array::*,
    BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher,
};

use crate::pac::aes::*;

use bitfield::BitRange;

#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
use aes::Block;

type Dataptr = u8;
type Indata = u32;

type InitVec = [u32; 4];
type Hashkey = [u32; 4];
type Ghash = [u32; 4];
type Ciplen = u32;
type Seed = u32;

bitfield::bitfield! {
    /// Hardware Countermeasures against Differential Power Analysis Attacks
    ///
    /// The AES module features four types of hardware countermeasures that are
    /// useful for protecting data against differential power analysis attacks:
    ///
    /// * Type 1: Randomly add one cycle to data processing
    /// * Type 2: Randomly add one cycle to data processing (other version)
    /// * Type 3: Add a random number of clock cycles to data processing,
    ///   subject to a maximum of 11/13/15 clock
    ///   cycles for key sizes of 128/192/256 bits
    /// * Type 4: Add random spurious power consumption during data processing
    ///
    /// By default, all countermeasures are enabled, but require a write in DRNGSEED
    /// register to be effective.
    ///
    /// The countermeasures use random numbers generated by a deterministic
    /// random number generator embedded in AES module.
    ///
    /// The seed for the random number generator is written to the RANDSEED register.
    ///
    /// *Note*: also that a new seed must be written after a change in the keysize.
    ///
    /// *Note*: that enabling countermeasures reduces AES module’s throughput. In short,
    /// the throughput is highest with all the countermeasures disabled.
    pub struct Ctype(u8);
    impl Debug;
    u8;
    get_ctype1, set_ctype1:  0;
    get_ctype2, set_ctype2:  1;
    get_ctype3, set_ctype3:  2;
    get_ctype4, set_ctype4:  3;
}

bitfield::bitfield! {
    /// AES->CTRLA Register
    ///
    /// Enable-protected configuration register
    pub struct CtrlaConf(u32);
    impl Debug;
    impl Default;
    u8;
    get_swrst, set_swrst:  0;
    get_enable, set_enable: 1;
    get_aesmode, set_aesmode: 4, 2;
    get_cfbs, set_cfbs: 7, 5;
    get_keysize, set_keysize: 9, 8;
    get_cipher, set_cipher: 10;
    get_startmode, set_startmode: 11;
    get_lod, set_lod: 12;
    get_keygen, set_keygen: 13;
    get_xorkey, set_xorkey: 14;
    get_ctype, set_ctype: 19, 16;
}

/// Holding area for AES peripheral
///
/// > **WARNING**
/// > RustCrypto backend assumes full ownership of AES hardware peripheral
///
/// Apart from creating new RustCrypto
/// there is no AES peripheral functionality exposed
#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
pub struct AesRustCrypto {
    /// AES pac register providing hardware access
    aes: Aes,
}

#[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
impl AesRustCrypto {
    /// Use the AES peripheral as hardware backend for RustCrypto AES
    ///
    /// Don't forget to enable the `APB` bus for AES
    ///
    /// `AHB` bus is on by default at reset
    ///
    /// Clock::v1
    /// `mclk.apbcmask.modify(|_, w| w.aes_().set_bit());`
    ///
    /// Clock::v2
    /// `tokens.apbs.aes.enable();`
    #[inline]
    pub fn new(aes: Aes) -> Self {
        Self { aes }
    }

    /// Destroy the AES RustCrypto backend and return the underlying AES
    /// register
    #[inline]
    pub fn destroy(self) -> Aes {
        self.aes
    }

    #[inline]
    pub fn new_128bit(&self, key: &GenericArray<u8, U16>) -> rustcrypto::Aes128 {
        rustcrypto::Aes128::new(key)
    }
    #[inline]
    pub fn new_192bit(&self, key: &GenericArray<u8, U24>) -> rustcrypto::Aes192 {
        rustcrypto::Aes192::new(key)
    }
    #[inline]
    pub fn new_256bit(&self, key: &GenericArray<u8, U32>) -> rustcrypto::Aes256 {
        rustcrypto::Aes256::new(key)
    }
}

/// AES Peripheral
///
/// Encapsulates the PAC which acts as a token
/// and provides an interface to the AES hardware
pub struct Aes {
    /// AES pac register providing hardware access
    aes: crate::pac::AES,
}

impl Aes {
    /// Create the interface for the AES peripheral
    ///
    /// Don't forget to enable the `APB` bus for AES
    ///
    /// `AHB` bus is on by default at reset
    ///
    /// Clock::v1
    /// `mclk.apbcmask.modify(|_, w| w.aes_().set_bit());`
    ///
    /// Clock::v2
    /// `tokens.apbs.aes.enable();`
    #[inline]
    pub fn new(aes: crate::pac::AES) -> Self {
        Self { aes }
    }

    /// Use the AES peripheral as hardware backend for RustCrypto AES
    ///
    /// Requires the feature `enable_unsafe_aes_newblock_cipher` to be enabled
    ///
    /// > **WARNING**
    /// > RustCrypto backend assumes full ownership of AES hardware peripheral
    ///
    /// Don't forget to enable the `APB` bus for AES
    ///
    /// `AHB` bus is on by default at reset
    ///
    /// Clock::v1
    /// `mclk.apbcmask.modify(|_, w| w.aes_().set_bit());`
    ///
    /// Clock::v2
    /// `tokens.apbs.aes.enable();`
    #[cfg(feature = "enable_unsafe_aes_newblock_cipher")]
    #[inline]
    pub fn activate_rustcrypto_backend(self) -> AesRustCrypto {
        AesRustCrypto::new(self)
    }

    // Register Interface

    /// Integrity Check Module
    #[inline]
    fn aes(&self) -> &RegisterBlock {
        &self.aes
    }

    /// Control A
    ///
    /// Enable-protected register
    #[inline]
    fn ctrla(&self) -> &CTRLA {
        &(*self.aes()).ctrla
    }

    /// Control B
    #[inline]
    fn ctrlb(&self) -> &CTRLB {
        &self.aes().ctrlb
    }

    /// Interrupt Enable Clear
    #[inline]
    fn intenclr(&self) -> &INTENCLR {
        &self.aes().intenclr
    }

    /// Interrupt Enable Set
    #[inline]
    fn intenset(&self) -> &INTENSET {
        &self.aes().intenset
    }

    /// Interrupt Flag Status and Clear
    #[inline]
    fn intflag(&self) -> &INTFLAG {
        &self.aes().intflag
    }

    /// Data Buffer Pointer
    #[inline]
    fn databufptr(&self) -> &DATABUFPTR {
        &self.aes().databufptr
    }

    /// Debug
    #[inline]
    fn dbgctrl(&self) -> &DBGCTRL {
        &self.aes().dbgctrl
    }

    /// INDATA
    ///
    /// A write to or read from this register corresponds to a write to or read
    /// from one of the four data registers. The four 32-bit Data registers
    /// set the 128-bit data block used for encryption/decryption. The data
    /// register that is written to or read from is given by the
    /// DATABUFPTR.INDATPTR field.
    ///
    /// Note:  Both input and output shares the same data buffer.
    /// Reading INDATA register will return 0’s when AES is performing
    /// encryption or decryption operation
    #[inline]
    fn indata(&self) -> &INDATA {
        &self.aes().indata
    }

    /// Galois Hash x (GCM mode only)
    ///
    /// Cipher length
    #[inline]
    fn ciplen(&self) -> &CIPLEN {
        &self.aes().ciplen
    }

    /// Random Seed
    #[inline]
    fn randseed(&self) -> &RANDSEED {
        &self.aes().randseed
    }

    // User interface for AES

    /// Enable the AES peripheral
    #[inline]
    pub fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
    }

    /// Disable the AES peripheral
    #[inline]
    pub fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
    }

    /// Reset the AES controller
    #[inline]
    pub fn swrst(&self) {
        self.ctrla().modify(|_, w| w.swrst().set_bit());
    }

    /// Destroy the AES peripheral and return the underlying AES register
    #[inline]
    pub fn destroy(self) -> crate::pac::AES {
        self.aes
    }

    // Control A

    /// Set AES Mode
    #[inline]
    pub fn set_aesmode(self, mode: AESMODESELECT_A) {
        self.ctrla().modify(|_, w| w.aesmode().variant(mode));
    }

    /// Set Cipher Feedback Block Size (CFBS)
    #[inline]
    pub fn set_cfbs(self, blocksize: CFBSSELECT_A) {
        self.ctrla().modify(|_, w| w.cfbs().variant(blocksize));
    }

    /// Set Encryption Key Size
    #[inline]
    pub fn set_keysize(self, keysize: KEYSIZESELECT_A) {
        self.ctrla().modify(|_, w| w.keysize().variant(keysize));
    }

    /// Set Cipher Mode
    #[inline]
    pub fn set_cipher(self, mode: CIPHERSELECT_A) {
        self.ctrla().modify(|_, w| w.cipher().variant(mode));
    }

    /// Set Start Mode
    #[inline]
    pub fn set_startmode(self, mode: STARTMODESELECT_A) {
        self.ctrla().modify(|_, w| w.startmode().variant(mode));
    }

    /// Set Last Output Data (LOD) Mode
    #[inline]
    pub fn set_lod(self, mode: LODSELECT_A) {
        self.ctrla().modify(|_, w| w.lod().variant(mode));
    }

    /// Start Last Key Generation
    ///
    /// Compute last NK words of expanded key
    #[inline]
    pub fn set_keygen(self, keygen_start: bool) {
        self.ctrla().modify(|_, w| w.keygen().bit(keygen_start));
    }

    /// XOR Key Generation
    ///
    /// The user keyword gets XORed with the previous keyword register content
    #[inline]
    pub fn set_xorkey(self, mode: XORKEYSELECT_A) {
        self.ctrla().modify(|_, w| w.xorkey().variant(mode));
    }

    /// Counter Measure Type
    #[inline]
    pub fn set_ctype(self, countermeasures: Ctype) {
        self.ctrla()
            .modify(|_, w| unsafe { w.ctype().bits(countermeasures.bit_range(3, 0)) });
    }

    // Control B

    /// Start Encryption/Decryption
    #[inline]
    pub fn start(&self) {
        self.ctrlb().modify(|_, w| w.start().set_bit());
    }

    /// New Message
    /// Used in cipher block chaining (CBC), cipher feedback (CFB) and output
    /// feedback (OFB), counter (CTR) modes to indicate that the hardware
    /// should use Initialization Vector for encrypting the first block of a
    /// message.
    #[inline]
    pub fn newmsg(&self) {
        self.ctrlb().modify(|_, w| w.newmsg().set_bit());
    }

    /// End of Message (GCM mode only)
    ///
    /// This triggers generation of final `GHASH` value for the message
    #[inline]
    pub fn eom(&self) {
        self.ctrlb().modify(|_, w| w.eom().set_bit());
    }

    /// GF Multiplication (GCM mode only)
    ///
    /// This triggers GF multiplication of data buffer content and hashkey
    /// register content.
    #[inline]
    pub fn gfmul(&self) {
        self.ctrlb().modify(|_, w| w.gfmul().set_bit());
    }

    // Interrupt Enable Clear

    /// Disable Encryption Complete Interrupt
    #[inline]
    pub fn disable_enccmp(&self) {
        self.intenclr().modify(|_, w| w.enccmp().set_bit());
    }

    /// Disable GF Multiplication Complete Interrupt
    #[inline]
    pub fn disable_gfmcmp(&self) {
        self.intenclr().modify(|_, w| w.gfmcmp().set_bit());
    }

    // Interrupt Enable Set

    /// Enable Encryption Complete Interrupt
    #[inline]
    pub fn enable_enccmp(&self) {
        self.intenset().modify(|_, w| w.enccmp().set_bit());
    }

    /// Enable GF Multiplication Complete Interrupt
    #[inline]
    pub fn enable_gfmcmp(&self) {
        self.intenset().modify(|_, w| w.gfmcmp().set_bit());
    }

    // Interrupt Flag Status and Clear

    /// Clear Encryption Complete Interrupt
    ///
    /// Also automatically cleared if
    ///
    /// 1. Manual encryption/decryption occurs (via CTRLB.START)
    /// 2. Reading from GHASHx register
    #[inline]
    pub fn clear_enccmp(&self) {
        self.intflag().modify(|_, w| w.enccmp().set_bit());
    }

    /// Clear GF Multiplication Complete Interrupt
    ///
    /// Also automatically cleared if
    ///
    /// 1. Manual encryption/decryption occurs (via CTRLB.START)
    /// 2. Reading from data register `INDATAx` when `LOD = 0`
    /// 2. Writing into the data register `INDATAx` when `LOD = 1`
    /// 2. Reading from Hash Key register (`HASHKEYx`)
    #[inline]
    pub fn clear_gfmcmp(&self) {
        self.intflag().modify(|_, w| w.gfmcmp().set_bit());
    }

    /// Read Encryption Complete Interrupt
    #[inline]
    pub fn read_enccmp(&self) -> bool {
        self.intflag().read().enccmp().bit_is_set()
    }

    /// Read GF Multiplication Complete Interrupt
    #[inline]
    pub fn read_gfmcmp(&self) -> bool {
        self.intflag().read().gfmcmp().bit_is_set()
    }

    // Data Buffer Pointer

    /// Set the Data Buffer Pointer
    #[inline]
    pub fn set_databufptr(&self, dataptr: Dataptr) {
        self.databufptr()
            .modify(|_, w| unsafe { w.indataptr().bits(dataptr) })
    }

    // Debug run control

    /// Control if AES is active during debugging
    ///
    /// Enable protected, only possible to change when AES is disabled
    #[inline]
    pub fn set_debug(&self, run_during_debug: bool) {
        self.dbgctrl()
            .modify(|_, w| w.dbgrun().bit(run_during_debug))
    }

    // Set Keyword

    /// Set keyword / cryptographic key
    ///
    /// Consists of four/six/eight Key Word registers
    /// for setting the cryptographic key
    #[inline]
    pub fn set_keyword<const N: usize>(&self, keyword: &[u8; N]) {
        for (index, _) in keyword.iter().step_by(4).enumerate() {
            // Combine four u8 into one u32
            let data = u32::from_ne_bytes([
                keyword[index],
                keyword[index + 1],
                keyword[index + 2],
                keyword[index + 3],
            ]);
            self.aes().keyword[index].write(|w| unsafe { w.bits(data) });
        }
    }

    // Data

    /// Set indata
    #[inline]
    pub fn set_data(&self, data: Indata) {
        self.indata().write(|w| unsafe { w.bits(data) });
    }

    /// Read indata
    pub fn get_data(&self) -> Indata {
        self.indata().read().bits()
    }

    // Initialization Vector Register

    /// Set initialization vector
    ///
    /// The four 32-bit Initialization Vector registers INTVECTVn set the
    /// 128-bit Initialization Vector data block that is used by
    /// some modes of operation as an additional initial input.
    /// INTVECTV0.INTVECTV corresponds to the first word of the
    /// Initialization Vector, INTVECTV3.INTVECTV to the last one.
    ///
    /// These registers are write-only to prevent the Initialization Vector
    /// from being read by another application. For CBC, OFB, and CFB modes,
    /// the Initialization Vector corresponds to the initialization vector.
    /// For CTR mode, it corresponds to the counter value.
    #[inline]
    pub fn set_initialization_vector(&self, iv: InitVec) {
        for (index, data) in iv.iter().enumerate() {
            self.aes().intvectv[index].write(|w| unsafe { w.bits(*data) });
        }
    }

    // Hash key (GCM mode only)

    /// Set GCM hash key value (GCM mode only)
    #[inline]
    pub fn set_hashkey(&self, hashkey: Hashkey) {
        for (index, data) in hashkey.iter().enumerate() {
            self.aes().hashkey[index].write(|w| unsafe { w.bits(*data) });
        }
    }

    pub fn get_hashkey(&self) -> Hashkey {
        let mut output = [0; 4];
        for (index, data) in self.aes().hashkey.iter().enumerate() {
            output[index] = data.read().bits();
        }
        output
    }

    // Galois Hash (GCM mode only)

    /// Set Galois Hash value (GCM mode only)
    ///
    /// Writing a new key to keyword clears `GHASH`
    #[inline]
    pub fn set_ghash(&self, ghash: Ghash) {
        for (index, data) in ghash.iter().enumerate() {
            self.aes().ghash[index].write(|w| unsafe { w.bits(*data) });
        }
    }

    /// Read Galois Hash value (GCM mode only)
    #[inline]
    pub fn get_ghash(&self) -> Ghash {
        let mut output = [0; 4];
        for (index, data) in self.aes().ghash.iter().enumerate() {
            output[index] = data.read().bits();
        }
        output
    }

    // Galois Hash x (GCM mode only)

    /// Set cipher length (GCM mode only)
    #[inline]
    pub fn set_ciplen(&self, ciplen: Ciplen) {
        self.ciplen().write(|w| unsafe { w.bits(ciplen) });
    }

    /// Read Cipher Length (GCM mode only)
    #[inline]
    pub fn get_ciplen(&self) -> Ciplen {
        self.ciplen().read().bits()
    }

    // Random Seed

    /// Set a new seed for the AES random number generator
    #[inline]
    pub fn set_randseed(&self, seed: Seed) {
        self.randseed().write(|w| unsafe { w.bits(seed) });
    }
}

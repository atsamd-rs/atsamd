#![warn(missing_docs)]
//! # Public Key Cryptography Controller (PUKCC)
//!
//! This module provides both low and high level abstractions for dealing with
//! a PUKCC peripheral.
//!
//! PUKCC consists of a set of functions (called services) hidden within a
//! reserved region of memory. These functions usually make use of a separate
//! piece of RAM to operate called CryptoRAM.
//!
//! [`c_abi`] module contains raw structs and callable C-like function
//! definitions. [`Pukcc`] wraps this low-level access API and exposes it in a
//! safe manner.
//!
//! ## WARNING!
//! This module has not been evaluated for correctness nor suitability for any
//! use-case. Subtle implementation details may have catastrophic implications
//! for the security of your cryptosystem, and users are advised to engage a
//! cryptographer before making use of this module.
#![allow(clippy::just_underscores_and_digits)]
pub mod c_abi;
pub mod curves;

use core::iter::{once, repeat};

use crate::pac::MCLK;
use c_abi::{u2, u4, CryptoRamSlice, Service};
use curves::Curve;

use rand_core::{CryptoRng, RngCore};

/// This macro linearly copies provided iterable slices/arrays to CryptoRAM and
/// assigns slices to provided declared local variables from outer scope
macro_rules! copy_to_cryptoram {
        (
            $crypto_ram:expr,
            $(
                ($name:ident, $data:expr)
            ),+
        ) =>
        {
            {
            (&[])
                .iter()
                .cloned()
            $(
                .chain($data)
            )+
            .zip($crypto_ram.iter_mut())
            .for_each(|(data_iter, cr_iter)| *cr_iter = data_iter);

            let mut _offset = 0;
            $(
                let len = $data.size_hint().1.unwrap_or_else(|| panic!("provided iterator has no size hint"));
                $name = &$crypto_ram[_offset.._offset + len];
                _offset += len;
            )+
            }
        }
}

/// Struct representing a PUKCC peripheral.
///
/// It provides an access to cryptographic algorithms in a safe, high-level
/// manner
pub struct Pukcc {
    __: (),
}

impl Pukcc {
    /// Constructor.
    ///
    /// Waits for a CryptoRAM readiness, enables a synchronous PUKCC clock and
    /// performs a self test. In case a self test fails it returns an error
    pub fn enable(mclk: &mut MCLK) -> Result<Self, SelfTestFailure> {
        unsafe {
            c_abi::wait_for_crypto_ram_clear_process();
        }
        mclk.ahbmask.modify(|_, w| w.pukcc_().set_bit());
        let pukcc = Self { __: () };
        pukcc.self_test().map(|_| pukcc)
    }

    /// Self test service.
    ///
    /// Clears up a CryptoRAM and does the checksum. If a checksum and a version
    /// matches one defined in a HAL, it means that a self test passed
    /// successfully.
    ///
    /// While using a high-level API, user should not need to use this service
    /// explicitly.
    pub fn self_test(&self) -> Result<(), SelfTestFailure> {
        const PUKCL_VERSION: u4 = 0x04070100;
        const CHECKNUM_1: u4 = 0x6E70DDD2;
        const CHECKNUM_2: u4 = 0x25C8D64F;
        let mut pukcl_params = c_abi::PukclParams::default();
        unsafe {
            c_abi::SelfTest::call(&mut pukcl_params);
        }
        let header = pukcl_params.header;
        let service_params = unsafe { pukcl_params.params.SelfTest };
        match header.u2Status.into() {
            PukclReturnCode::Ok => {}
            _ => return Err(SelfTestFailure(service_params)),
        };
        if service_params.u4Version != PUKCL_VERSION {
            return Err(SelfTestFailure(service_params));
        }
        if service_params.u4CheckNum1 != CHECKNUM_1 {
            return Err(SelfTestFailure(service_params));
        }
        if service_params.u4CheckNum2 != CHECKNUM_2 {
            return Err(SelfTestFailure(service_params));
        }

        Ok(())
    }

    /// Service generating an ECDSA signature.
    ///
    /// GF(p) service. GF(2^n) variant is not implemented -- use low-level API.
    ///
    /// Input parameters:
    /// - `hash`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Hash of a message that is supposed to be signed.
    /// - `private_key`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Private key used for signing. Poorly generated `private_key` might
    ///       have negative security implications.
    /// - `k_buffer`: `&mut [u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Mutable buffer that is being populated by an entropy source and
    ///       then used for signing.
    /// - `k_entropy_source`: `&mut (impl RngCore + CryptoRng)`
    ///     - Generic source of cryptographically secure randomness.
    ///
    /// Output parameters:
    /// - `signature`: `&mut [u8]` of length `2 * `[`Curve::MOD_LENGTH`]
    ///     - Mutable slice that signature will be copied to from CryptoRAM
    ///       after generation is finished. First [`Curve::MOD_LENGTH`] bytes
    ///       contain `R` part of a signature. Last [`Curve::MOD_LENGTH`] bytes
    ///       contain `S` part of a signature.
    ///
    /// Return value:
    /// - `Result::Ok`
    ///     - Signature was generated successfully
    /// - `Result::Err`
    ///     - Possible failure scenarios are encapsulated in a
    ///       [`EcdsaSignFailure`] enum type
    ///
    /// Note: Provided [`Curve`] needs to be sound. Otherwise, point
    /// multiplication can become reversible (lack of _trapdoor function_
    /// property) and an attacker might be able to reverse engineer a
    /// `private_key` from a `signature`.
    pub fn zp_ecdsa_sign_with_entropy<C: Curve>(
        &self,
        signature: &mut [u8],
        hash: &[u8],
        private_key: &[u8],
        k_buffer: &mut [u8],
        k_entropy_source: &mut (impl RngCore + CryptoRng),
    ) -> Result<(), EcdsaSignFailure> {
        k_entropy_source.fill_bytes(k_buffer);
        self.zp_ecdsa_sign::<C>(signature, hash, private_key, k_buffer)
    }

    /// Service generating an ECDSA signature.
    ///
    /// GF(p) service. GF(2^n) variant is not implemented -- use low-level API.
    ///
    /// Input parameters:
    /// - `hash`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Hash of a message that is supposed to be signed.
    /// - `private_key`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Private key used for signing. Poorly generated `private_key` might
    ///       have negative security implications.
    /// - `k`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - A random number used for signature generation. It is heavily
    ///       encouraged to use cryptographically-secure random number
    ///       generators. One should never use the same `k` more than once.
    ///       Private key can be extracted from signatures generated with a
    ///       poorly randomized / the same `k` value.
    ///
    /// Exact same set of input parameters (hash, private_key and k) produces
    /// exactly the same signature.
    ///
    /// Output parameters:
    /// - `signature`: `&mut [u8]` of length `2 * `[`Curve::MOD_LENGTH`]
    ///     - Mutable slice that signature will be copied to from CryptoRAM
    ///       after generation is finished. First [`Curve::MOD_LENGTH`] bytes
    ///       contain `R` part of a signature. Last [`Curve::MOD_LENGTH`] bytes
    ///       contain `S` part of a signature.
    ///
    /// Return value:
    /// - `Result::Ok`
    ///     - Signature was generated successfully
    /// - `Result::Err`
    ///     - Possible failure scenarios are encapsulated in a
    ///       [`EcdsaSignFailure`] enum type
    ///
    /// Note: Provided [`Curve`] needs to be sound. Otherwise, point
    /// multiplication can become reversible (lack of _trapdoor function_
    /// property) and an attacker might be able to reverse engineer a
    /// `private_key` from a `signature`.
    ///
    /// # Safety
    ///
    /// `k` value must be cryptographically secure.
    pub unsafe fn zp_ecdsa_sign_with_raw_k<C: Curve>(
        &self,
        signature: &mut [u8],
        hash: &[u8],
        private_key: &[u8],
        k: &[u8],
    ) -> Result<(), EcdsaSignFailure> {
        self.zp_ecdsa_sign::<C>(signature, hash, private_key, k)
    }

    fn zp_ecdsa_sign<C: Curve>(
        &self,
        signature: &mut [u8],
        hash: &[u8],
        private_key: &[u8],
        k: &[u8],
    ) -> Result<(), EcdsaSignFailure> {
        C::verify_curve().map_err(EcdsaSignFailure::InvalidCurve)?;

        if signature.len() != (2 * C::MOD_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameterLength {
                faulty_slice: "signature",
                expected_length: (2 * C::MOD_LENGTH).into(),
                actual_length: signature.len(),
            });
        }
        if hash.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameterLength {
                faulty_slice: "hash",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: hash.len(),
            });
        }
        if private_key.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameterLength {
                faulty_slice: "private_key",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: private_key.len(),
            });
        }
        if k.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameterLength {
                faulty_slice: "k",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: k.len(),
            });
        }
        let (
            modulo_p,
            a_curve,
            base_point_a_x,
            base_point_a_y,
            base_point_a_z,
            order_point,
            cns,
            hash_cr,
            private_key_cr,
            k_cr,
            workspace,
            mut __,
        );
        let mut crypto_ram = unsafe { c_abi::CryptoRam::new() };
        // 32-byte padding with zeroes on a MSB side of every parameter is required by
        // PUKCC algorithms. Little endianness requires padding *after* a parameter
        // as MSB is placed on high addresses.

        // 32-byte zero padding for curve parameters should be included in original
        // slices.
        copy_to_cryptoram! {
            crypto_ram,
            (modulo_p, C::MODULO_P.iter().cloned().rev()),
            (a_curve, C::A_CURVE.iter().cloned().rev()),
            (base_point_a_x, C::BASE_POINT_A_X.iter().cloned().rev()),
            (base_point_a_y, C::BASE_POINT_A_Y.iter().cloned().rev()),
            (base_point_a_z, C::BASE_POINT_A_Z.iter().cloned().rev()),
            (order_point, C::ORDER_POINT.iter().cloned().rev()),
            (cns, C::CNS.iter().cloned().rev()),
            (hash_cr, hash.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            (private_key_cr, private_key.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            (k_cr, k.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            // Workspace is just marked with a zero length iterator just to get its address.
            // As it is placed at the end, idea is that algorithm will use whatever amount
            // of memory it needs
            (workspace, 0..0)
        };
        let mut pukcl_params = c_abi::PukclParams::default();
        unsafe {
            let mut service_params = &mut pukcl_params.params.ZpEcDsaGenerateFast;
            service_params.nu1ModBase = modulo_p.pukcc_base();
            service_params.nu1CnsBase = cns.pukcc_base();
            service_params.u2ModLength = C::MOD_LENGTH as u16;
            service_params.nu1ScalarNumber = k_cr.pukcc_base();
            service_params.nu1OrderPointBase = order_point.pukcc_base();
            service_params.nu1PrivateKey = private_key_cr.pukcc_base();
            service_params.nu1HashBase = hash_cr.pukcc_base();
            service_params.u2ScalarLength = C::SCALAR_LENGTH;
            service_params.nu1PointABase = base_point_a_x.pukcc_base();
            service_params.nu1ABase = a_curve.pukcc_base();
            service_params.nu1Workspace = workspace.pukcc_base();
        }

        unsafe { c_abi::ZpEcDsaGenerateFast::call(&mut pukcl_params) };

        match pukcl_params.header.u2Status.into() {
            PukclReturnCode::Ok => {}
            error_code => return Err(EcdsaSignFailure::ServiceFailure(error_code)),
        };

        // Generated signature R part is written to base point X coordinate memory.
        // Generated signature S part is written to base point Y coordinate memory.
        // Base point Z coordinate should be zero.

        if !base_point_a_z.iter().all(|&el| el == 0) {
            return Err(EcdsaSignFailure::BasePointZCoordinateIsNotZero);
        }

        // Copying signature back from the CryptoRAM while ignoring irrelevant padding.
        signature
            .iter_mut()
            .zip(
                base_point_a_x
                    .iter()
                    .rev()
                    .skip(4)
                    .chain(base_point_a_y.iter().rev().skip(4)),
            )
            .for_each(|(target_iter, source_iter)| *target_iter = *source_iter);

        Ok(())
    }

    /// Service verifying an ECDSA signature.
    ///
    /// GF(p) service. GF(2^n) variant is not implemented -- use low-level API.
    ///
    /// Input parameters:
    /// - `signature`: `&[u8]` of length `2 * `[`Curve::SCALAR_LENGTH`]
    ///     - Signature that is being verified
    /// - `hash`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Hash of a message that is signed.
    /// - `public_key`: `&[u8]` of length [`Curve::SCALAR_LENGTH`]
    ///     - Public key used for a signature verification.
    ///
    /// Return value:
    /// - `Result::Ok`
    ///     - Signature is valid against chosen `hash` and `public_key`
    /// - `Result::Err`
    ///     - Possible failure scenarios are encapsulated in a
    ///       [`EcdsaSignatureVerificationFailure`] enum type
    ///
    /// In case of an invalid signature the returned error type will be
    /// [`EcdsaSignatureVerificationFailure::ServiceFailure`]`(`
    /// [`Warning`][`PukclReturnCode::Warning`]`(`
    /// [`WrongSignature`][`PukclReturnCodeWarning::WrongSignature`]`))`
    pub fn zp_ecdsa_verify_signature<C: Curve>(
        &self,
        signature: &[u8],
        hash: &[u8],
        public_key: &[u8],
    ) -> Result<(), EcdsaSignatureVerificationFailure> {
        C::verify_curve().map_err(EcdsaSignatureVerificationFailure::InvalidCurve)?;

        let (
            modulo_p,
            a_curve,
            base_point_a_x,
            order_point,
            cns,
            signature_cr,
            hash_cr,
            public_key_cr,
            workspace,
            mut __,
        );
        if signature.len() != (2 * C::SCALAR_LENGTH).into() {
            return Err(
                EcdsaSignatureVerificationFailure::WrongInputParameterLength {
                    faulty_slice: "signature",
                    expected_length: (2 * C::SCALAR_LENGTH).into(),
                    actual_length: signature.len(),
                },
            );
        }
        if hash.len() != (C::SCALAR_LENGTH).into() {
            return Err(
                EcdsaSignatureVerificationFailure::WrongInputParameterLength {
                    faulty_slice: "hash",
                    expected_length: (C::SCALAR_LENGTH).into(),
                    actual_length: hash.len(),
                },
            );
        }
        if public_key.len() != (2 * C::MOD_LENGTH).into() {
            return Err(
                EcdsaSignatureVerificationFailure::WrongInputParameterLength {
                    faulty_slice: "public_key",
                    expected_length: (2 * C::MOD_LENGTH).into(),
                    actual_length: public_key.len(),
                },
            );
        }
        let mut crypto_ram = unsafe { c_abi::CryptoRam::new() };
        // 32-byte padding with zeroes on a MSB side of every parameter is required by
        // PUKCC algorithms. Little endianness requires padding *after* a parameter
        // as MSB is placed on high addresses.

        // 32-byte zero padding for curve parameters should be included in original
        // slices.
        copy_to_cryptoram! {
            crypto_ram,
            (modulo_p, C::MODULO_P.iter().cloned().rev()),
            (a_curve, C::A_CURVE.iter().cloned().rev()),
            (base_point_a_x, C::BASE_POINT_A_X.iter().cloned().rev()),
            (__, C::BASE_POINT_A_Y.iter().cloned().rev()),
            (__, C::BASE_POINT_A_Z.iter().cloned().rev()),
            (order_point, C::ORDER_POINT.iter().cloned().rev()),
            (cns, C::CNS.iter().cloned().rev()),
            // Signature has to be split into two parts + padding must be added
            // Signature layout:
            //   [ R: (little endian) ][ 0_u32 ]..
            (signature_cr, signature.iter().cloned().take(C::SCALAR_LENGTH.into()).rev()),
            (__, repeat(0).take(4)),
            // ..[ S: (little endian) ][ 0_u32 ]
            (__, signature.iter().cloned().skip(C::SCALAR_LENGTH.into()).take(C::SCALAR_LENGTH.into()).rev()),
            (__, repeat(0).take(4)),
            (hash_cr, hash.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            // Public key has to be represented as a point + padding must be added
            // Public key layout:
            //   [ X coordinate: (little endian) ][ 0_u32 ]..
            (public_key_cr, public_key.iter().cloned().take(C::MOD_LENGTH.into()).rev()),
            (__, repeat(0).take(4)),
            // ..[ Y coordinate: (little endian) ][ 0_u32 ]
            (__, public_key.iter().cloned().skip(C::MOD_LENGTH.into()).take(C::MOD_LENGTH.into()).rev()),
            (__, repeat(0).take(4)),
            // ..[ Z coordinate: (little endian) ][ 0_u32 ] == 1
            (__, once(1).chain(repeat(0).take((C::MOD_LENGTH - 1).into()))),
            (__, repeat(0).take(4)),
            // Workspace is just marked with a zero length iterator just to get its address.
            // As it is placed at the end, idea is that algorithm will use whatever amount
            // of memory it needs
            (workspace, 0..0)
        };
        let mut pukcl_params = c_abi::PukclParams::default();
        unsafe {
            let mut service_params = &mut pukcl_params.params.ZpEcDsaVerifyFast;
            service_params.nu1ModBase = modulo_p.pukcc_base();
            service_params.nu1CnsBase = cns.pukcc_base();
            service_params.u2ModLength = C::MOD_LENGTH;
            service_params.nu1OrderPointBase = order_point.pukcc_base();
            service_params.nu1PointSignature = signature_cr.pukcc_base();
            service_params.nu1HashBase = hash_cr.pukcc_base();
            service_params.u2ScalarLength = C::SCALAR_LENGTH;
            service_params.nu1PointABase = base_point_a_x.pukcc_base();
            service_params.nu1PointPublicKeyGen = public_key_cr.pukcc_base();
            service_params.nu1ABase = a_curve.pukcc_base();
            service_params.nu1Workspace = workspace.pukcc_base();
        }

        unsafe { c_abi::ZpEcDsaVerifyFast::call(&mut pukcl_params) };

        match pukcl_params.header.u2Status.into() {
            PukclReturnCode::Ok => Ok(()),
            error_code => Err(EcdsaSignatureVerificationFailure::ServiceFailure(
                error_code,
            )),
        }
    }

    /// Service performing a modular exponentiation.
    ///
    /// ```text
    /// result = pow(input, exponent) % modulus
    /// ```
    ///
    /// Input parameters:
    /// - `input`: `&[u8]`
    ///     - Requirements:
    ///         - `len(input) <= len(modulus)`
    ///     - Message, hash, any slice of data that will undergo modular
    ///       exponentiation
    /// - `exponent`: `&[u8]`
    ///     - Requirements:
    ///         - `len(exponent) <= len(modulus)`
    /// - `modulus`: `&[u8]`
    ///     - Requirements:
    ///         - `len(modulus) % 4`
    ///         - `12 <= len(modulus) < ?`
    ///     - Note: Maximum size depends on few factors like CryptoRAM and
    ///       workspace window size. Consult the table with data layout down
    ///       below.
    /// - `mode`: [`ExpModMode`]
    ///     - Mode of operation: use regular or fast variant of the underlying
    ///       algorithm
    ///     - This parameter does not influence the end result of a computation
    /// - `window_size`: [`ExpModWindowSize`]
    ///     - Enum describing 4 predefined workspace sizes (from smallest to
    ///       biggest) in CryptoRAM.
    ///     - Bigger the workspace size - faster the algorithm can operate -
    ///       greater limitations on input parameters are put (as they occupy
    ///       CryptoRAM address space as well). Consult the table with data
    ///       layout down below.
    ///     - This parameter does not influence the end result of a computation
    /// - `buffer`: `&'a mut [u8]`
    ///     - Requirements:
    ///         - `len(buffer) >= len(modulus) + 5`
    ///     - Buffer used internally for CNS calculation. Piece of it is used
    ///       also for a return value.
    ///
    /// Return value:
    /// - `Result::Ok(&'a [u8])`
    ///     - Length: `len(modulus)`
    ///     - A result of modular exponentiation
    /// - `Result::Err`
    ///     - Possible failure scenarios are encapsulated in a [`ExpModFailure`]
    ///       enum type
    ///
    /// Failing to meet the requirements for any input parameter will end up
    /// with an error being returned.
    ///
    /// CryptoRAM is `4KiB` (`0x1000` bytes) in size. Data layout in CryptoRAM
    /// looks as follows and its size cannot go over the threshold of `4KiB`.
    ///
    /// ```text
    /// - modulus: len(modulus) + 4
    /// - cns (reduction constant): len(modulus) + 8
    /// - output/input (after/before calculation): len(modulus) + 16
    /// - exponent: len(exponent) + 4 (+ padding to be % 4)
    /// - workspace: (depending on `window_size`)
    ///     - ExpModWindowSize::One => 3 * (modulus.len() + 4) + 8
    ///     - ExpModWindowSize::Two => 4 * (modulus.len() + 4) + 8
    ///     - ExpModWindowSize::Three => 6 * (modulus.len() + 4) + 8
    ///     - ExpModWindowSize::Four => 10 * (modulus.len() + 4) + 8
    /// ```
    ///
    /// # RSA
    ///
    /// This function can be used to perform RSA related computation like
    /// encryption, decryption, signature generation and validation.
    /// - To **encrypt** `input` value, split _public key_ into _public_
    ///   `exponent` and `modulus` and pass them into the function. Return value
    ///   is going to be a cipher of an `input`.
    /// - To **decrypt** `input` value, split _private key_ into _private_
    ///   `exponent` and `modulus` and pass them into the function. Return value
    ///   is going to be a decrypted `input`.
    /// - To **generate a signature**, pass the _hash_ of a message being signed
    ///   as an `input` into the function and use a _private key_. Return value
    ///   is going to be a signature (encrypted _hash_).
    /// - To **validate a signature**, pass it as an `input` into the function
    ///   and use the _public key_. Decrypted signature is an expected hash of a
    ///   message. Calculate the hash of your message and compare it with an
    ///   expected value. If they are the same, validation can be considered
    ///   successful.
    ///
    /// All RSA variants up to **RSA4096** (included) will fit into CryptoRAM
    /// and therefore are supported.
    pub fn modular_exponentiation<'a>(
        &self,
        input: &[u8],
        exponent: &[u8],
        modulus: &[u8],
        mode: ExpModMode,
        window_size: ExpModWindowSize,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], ExpModFailure> {
        const PUKCL_EXPMOD_EXPINPUKCCRAM: u16 = 0x02;

        // Modulus validation
        if modulus.len() % 4 != 0 {
            return Err(ExpModFailure::WrongInputParameterAlignment {
                faulty_slice: "modulus",
            });
        }
        // Modulus size must be at least 12 bytes (43.3.5.2.3 of DS60001507F datasheet)
        const MINIMUM_MODULUS_LEN: usize = 12;
        if modulus.len() < MINIMUM_MODULUS_LEN {
            return Err(ExpModFailure::WrongInputParameterLength {
                faulty_slice: "modulus",
                actual_length: modulus.len(),
                expected_length: ExpectedLengthError::AtLeast(MINIMUM_MODULUS_LEN),
            });
        }
        // Input validation
        // Note: Only length is checked but in theory value itself being larger than
        // modulus is probably wrong as well
        if input.len() > modulus.len() {
            return Err(ExpModFailure::WrongInputParameterLength {
                faulty_slice: "input",
                actual_length: input.len(),
                expected_length: ExpectedLengthError::AtMost(modulus.len()),
            });
        }
        // Exponent validation
        // Note: Only length is checked but in theory value itself being larger than
        // modulus is probably wrong as well
        if exponent.len() > modulus.len() {
            return Err(ExpModFailure::WrongInputParameterLength {
                faulty_slice: "exponent",
                actual_length: exponent.len(),
                expected_length: ExpectedLengthError::AtMost(modulus.len()),
            });
        }

        let (modulus_cr, cns_cr, output, workspace, exponent_cr, mut __);

        let cns = self.zp_calculate_cns(buffer, modulus)?;
        let padding_for_cns = padding_for_len(cns.len());
        // Sanity check in case someone changes `zp_calculate_cns` implementation
        assert!(cns.len() + padding_for_cns == modulus.len() + 8);
        let padding_for_exponent = padding_for_len(exponent.len());

        let mut crypto_ram = unsafe { c_abi::CryptoRam::new() };
        // 32-byte padding with zeroes on a MSB side of every parameter is required by
        // PUKCC algorithms (unless said otherwise). Little endianness requires padding
        // *after* a parameter as MSB is placed on high addresses.
        copy_to_cryptoram! {
            crypto_ram,
            (modulus_cr, modulus.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            (cns_cr, cns.iter().cloned().rev()),
            (__, repeat(0).take(padding_for_cns)),
            // 1. `input` is replaced with an outcome of the computation
            // 2. `input` is padded to match `len(modulus)`
            (output, input.iter().cloned().rev().chain(repeat(0).take(modulus.len() - input.len()))),
            // `input` area is used for computation and requires additional 4 0_u32 on MSB side
            (__, repeat(0).take(16)),
            // Exponent is required to have 0_u32 on LSB side
            // `exponent_cr` is only used to get a pointer during `pukcl_params` initialization
            (exponent_cr, repeat(0).take(4)),
            (__, exponent.iter().cloned().rev()),
            (__, repeat(0).take(padding_for_exponent)),
            // Workspace is just marked with a zero length slice just to get its address. As
            // it is placed at the end, idea is that algorithm will use whatever amount of
            // memory it needs
            (workspace, 0..0)
        };
        // Table 43-52; 43.3.5.2.5 in DS60001507F
        let workspace_len: usize = match window_size {
            ExpModWindowSize::One => 3 * (modulus.len() + 4) + 8,
            ExpModWindowSize::Two => 4 * (modulus.len() + 4) + 8,
            ExpModWindowSize::Three => 6 * (modulus.len() + 4) + 8,
            ExpModWindowSize::Four => 10 * (modulus.len() + 4) + 8,
        };
        let workspace_end_ptr = workspace.as_ptr().wrapping_add(workspace_len);
        let crypto_ram_end_ptr = crypto_ram.as_ptr_range().end;
        if workspace_end_ptr > crypto_ram_end_ptr {
            return Err(ExpModFailure::RunOutOfCryptoRam {
                workspace_end_ptr,
                crypto_ram_end_ptr,
            });
        }
        let mut pukcl_params = c_abi::PukclParams::default();
        unsafe {
            // Note: `exponent` outside of Crypto RAM is not supported
            pukcl_params.header.u2Option = PUKCL_EXPMOD_EXPINPUKCCRAM
                | window_size.get_windows_size_mask()
                | mode.get_mode_mask();
            let mut service_params = &mut pukcl_params.params.ExpMod;
            service_params.nu1XBase = output.pukcc_base();
            service_params.nu1ModBase = modulus_cr.pukcc_base();
            service_params.nu1CnsBase = cns_cr.pukcc_base();
            service_params.nu1PrecompBase = workspace.pukcc_base();
            service_params.pfu1ExpBase = exponent_cr.as_ptr() as _;
            service_params.u2ModLength = modulus.len() as _;
            service_params.u2ExpLength = (exponent.len() + padding_for_exponent) as _;
            service_params.u1Blinding = 0;
        }

        unsafe { c_abi::ExpMod::call(&mut pukcl_params) };
        match pukcl_params.header.u2Status.into() {
            PukclReturnCode::Ok => {}
            error_code => return Err(ExpModFailure::ServiceFailure(error_code)),
        }

        buffer
            .iter_mut()
            .zip(output.iter().rev())
            .for_each(|(target_iter, source_iter)| *target_iter = *source_iter);

        Ok(&buffer[..modulus.len()])
    }

    /// Service producing a reduction constant value
    fn zp_calculate_cns<'a>(
        &self,
        buffer: &'a mut [u8],
        modulus: &[u8],
    ) -> Result<&'a [u8], CalculateCnsFailure> {
        const PUKCL_REDMOD_SETUP: u16 = 0x0100;
        if modulus.len() % 4 != 0 {
            return Err(CalculateCnsFailure::WrongInputParameterAlignment {
                faulty_slice: "modulus",
            });
        }

        // Even though documentation says that CNS occupies len(modulus) + 12 of space,
        // it is only needed for computation, 7 MSB bytes are zeroes. This distinction
        // between lengths allows to skip these 7 MSB zero bytes.
        let cns_length = modulus.len() + 12;
        let actual_cns_length = modulus.len() + 5;

        if buffer.len() < actual_cns_length {
            return Err(CalculateCnsFailure::WrongInputParameterLength {
                faulty_slice: "buffer",
                actual_length: buffer.len(),
                expected_length: ExpectedLengthError::AtLeast(actual_cns_length),
            });
        }
        let (modulus_cr, cns_cr, workspace_r, workspace_x, mut __);
        let mut crypto_ram = unsafe { c_abi::CryptoRam::new() };
        copy_to_cryptoram! {
            crypto_ram,
            (modulus_cr, modulus.iter().cloned().rev()),
            (__, repeat(0).take(4)),
            (cns_cr, repeat(0).take(actual_cns_length)),
            (__, repeat(0).take(cns_length - actual_cns_length)),
             // GF(p) -> 64 bytes
            (workspace_r, repeat(0).take(64)),
            (workspace_x, 0..0)
        };
        let mut pukcl_params = c_abi::PukclParams::default();
        unsafe {
            // Flag that switches behaviour of `RedMod` service into CNS generator
            pukcl_params.header.u2Option = PUKCL_REDMOD_SETUP;
            let mut service_params = &mut pukcl_params.params.RedMod;
            service_params.nu1ModBase = modulus_cr.pukcc_base();
            service_params.nu1CnsBase = cns_cr.pukcc_base();
            service_params.u2ModLength = modulus.len() as _;
            service_params.nu1RBase = workspace_r.pukcc_base();
            service_params.nu1XBase = workspace_x.pukcc_base();
        }
        unsafe { c_abi::RedMod::call(&mut pukcl_params) };
        match pukcl_params.header.u2Status.into() {
            PukclReturnCode::Ok => {}
            error_code => return Err(CalculateCnsFailure::ServiceFailure(error_code)),
        }

        buffer
            .iter_mut()
            .zip(cns_cr.iter().rev())
            .for_each(|(target_iter, source_iter)| *target_iter = *source_iter);
        Ok(&buffer[..actual_cns_length])
    }
}

/// An error type representing failure modes a [`Pukcc::self_test`] service
#[derive(Debug)]
pub struct SelfTestFailure(c_abi::SelfTest);

/// An error type representing failure modes for a
/// [`Pukcc::zp_ecdsa_sign_with_entropy`] and
/// [`Pukcc::zp_ecdsa_sign_with_raw_k`] service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EcdsaSignFailure {
    WrongInputParameterLength {
        faulty_slice: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
    InvalidCurve(curves::CurveVerificationFailure),
    BasePointZCoordinateIsNotZero,
    ServiceFailure(PukclReturnCode),
}

/// An error type representing failure modes for a
/// [`Pukcc::zp_ecdsa_verify_signature`] service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EcdsaSignatureVerificationFailure {
    WrongInputParameterLength {
        faulty_slice: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
    InvalidCurve(curves::CurveVerificationFailure),
    ServiceFailure(PukclReturnCode),
}

/// An error type specifying an expected length of a slice in question
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExpectedLengthError {
    AtMost(usize),
    AtLeast(usize),
    Exactly(usize),
}

/// An enum describing available modes of operation of
/// `Pukcc::modular_exponentiation` algoritm
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExpModMode {
    Regular,
    Fast,
}

impl ExpModMode {
    /// Function mapping the enum variant with a low level mask value needed in
    /// [`c_abi::PukclHeader::u2Option`] for [`c_abi::ExpMod`] service
    pub fn get_mode_mask(&self) -> u2 {
        use ExpModMode::*;
        match self {
            Regular => 0x01,
            Fast => 0x04,
        }
    }
}

/// An enum describing allowed, predefined window sizes for a calculation
/// workspace in CryptoRAM for [`Pukcc::modular_exponentiation`] algorithm
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExpModWindowSize {
    /// 3 * (len(modulus) + 4) + 8 bytes allowed to be used as a workspace
    One,
    /// 4 * (len(modulus) + 4) + 8 bytes allowed to be used as a workspace
    Two,
    /// 6 * (len(modulus) + 4) + 8 bytes allowed to be used as a workspace
    Three,
    /// 10 * (len(modulus) + 4) + 8 bytes allowed to be used as a workspace
    Four,
}

impl ExpModWindowSize {
    /// Function mapping the enum variant with a low level mask value needed in
    /// [`c_abi::PukclHeader::u2Option`] for [`c_abi::ExpMod`] service
    pub fn get_windows_size_mask(&self) -> u2 {
        use ExpModWindowSize::*;
        match self {
            One => 0x00,
            Two => 0x08,
            Three => 0x10,
            Four => 0x18,
        }
    }
}

/// An error type representing failure modes for a
/// [`Pukcc::modular_exponentiation`] service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ExpModFailure {
    WrongInputParameterLength {
        faulty_slice: &'static str,
        expected_length: ExpectedLengthError,
        actual_length: usize,
    },
    /// Should be 4-aligned
    WrongInputParameterAlignment {
        faulty_slice: &'static str,
    },
    RunOutOfCryptoRam {
        workspace_end_ptr: *const u8,
        crypto_ram_end_ptr: *const u8,
    },
    CalculateCnsFailure(CalculateCnsFailure),
    ServiceFailure(PukclReturnCode),
}

/// An error type representing failure modes for a
/// `Pukcc::zp_calculate_cns` service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum CalculateCnsFailure {
    WrongInputParameterLength {
        faulty_slice: &'static str,
        expected_length: ExpectedLengthError,
        actual_length: usize,
    },
    /// Should be 4-aligned
    WrongInputParameterAlignment {
        faulty_slice: &'static str,
    },
    ServiceFailure(PukclReturnCode),
}

impl From<CalculateCnsFailure> for ExpModFailure {
    fn from(f: CalculateCnsFailure) -> Self {
        ExpModFailure::CalculateCnsFailure(f)
    }
}

// PukclReturnCode <-> c_abi::PukclReturnCode
impl core::convert::From<c_abi::PukclReturnCode> for PukclReturnCode {
    fn from(v: c_abi::PukclReturnCode) -> Self {
        use PukclReturnCode::*;
        match v.0 {
            0x0000 => Ok,
            0xC001 => Severe(PukclReturnCodeSevere::ComputationNotStarted),
            0xC002 => Severe(PukclReturnCodeSevere::UnknownService),
            0xC003 => Severe(PukclReturnCodeSevere::UnexploitableOptions),
            0xC004 => Severe(PukclReturnCodeSevere::HardwareIssue),
            0xC005 => Severe(PukclReturnCodeSevere::WrongHardware),
            0xC006 => Severe(PukclReturnCodeSevere::LibraryMalformed),
            0xC007 => Severe(PukclReturnCodeSevere::Error),
            0xC008 => Severe(PukclReturnCodeSevere::UnknownSubservice),
            0xC010 => Severe(PukclReturnCodeSevere::OverlapNotAllowed),
            0xC011 => Severe(PukclReturnCodeSevere::ParamNotInPukccram),
            0xC012 => Severe(PukclReturnCodeSevere::ParamNotInRam),
            0xC013 => Severe(PukclReturnCodeSevere::ParamNotInCpuram),
            0xC014 => Severe(PukclReturnCodeSevere::ParamWrongLength),
            0xC015 => Severe(PukclReturnCodeSevere::ParamBadAlignment),
            0xC016 => Severe(PukclReturnCodeSevere::ParamXBiggerThanY),
            0xC017 => Severe(PukclReturnCodeSevere::ParamLengthTooSmall),
            0xC101 => Severe(PukclReturnCodeSevere::DivisionByZero),
            0xC102 => Severe(PukclReturnCodeSevere::MalformedModulus),
            0xC103 => Severe(PukclReturnCodeSevere::FaultDetected),
            0xC104 => Severe(PukclReturnCodeSevere::MalformedKey),
            0xC105 => Severe(PukclReturnCodeSevere::AprioriOk),
            0xC106 => Severe(PukclReturnCodeSevere::WrongService),
            0x8001 => Warning(PukclReturnCodeWarning::PointAtInfinity),
            0x8002 => Warning(PukclReturnCodeWarning::WrongSignature),
            0x8003 => Warning(PukclReturnCodeWarning::WrongSelectnumber),
            0x8004 => Warning(PukclReturnCodeWarning::PointIsNotOnCurve),
            0x4001 => Info(PukclReturnCodeInfo::NumberIsNotPrime),
            0x4002 => Info(PukclReturnCodeInfo::NumberIsPrime),
            code => Unknown { code },
        }
    }
}

impl core::convert::From<PukclReturnCode> for c_abi::PukclReturnCode {
    fn from(v: PukclReturnCode) -> Self {
        use PukclReturnCode::*;
        Self(match v {
            Ok => 0x0000,
            Severe(PukclReturnCodeSevere::ComputationNotStarted) => 0xC001,
            Severe(PukclReturnCodeSevere::UnknownService) => 0xC002,
            Severe(PukclReturnCodeSevere::UnexploitableOptions) => 0xC003,
            Severe(PukclReturnCodeSevere::HardwareIssue) => 0xC004,
            Severe(PukclReturnCodeSevere::WrongHardware) => 0xC005,
            Severe(PukclReturnCodeSevere::LibraryMalformed) => 0xC006,
            Severe(PukclReturnCodeSevere::Error) => 0xC007,
            Severe(PukclReturnCodeSevere::UnknownSubservice) => 0xC008,
            Severe(PukclReturnCodeSevere::OverlapNotAllowed) => 0xC010,
            Severe(PukclReturnCodeSevere::ParamNotInPukccram) => 0xC011,
            Severe(PukclReturnCodeSevere::ParamNotInRam) => 0xC012,
            Severe(PukclReturnCodeSevere::ParamNotInCpuram) => 0xC013,
            Severe(PukclReturnCodeSevere::ParamWrongLength) => 0xC014,
            Severe(PukclReturnCodeSevere::ParamBadAlignment) => 0xC015,
            Severe(PukclReturnCodeSevere::ParamXBiggerThanY) => 0xC016,
            Severe(PukclReturnCodeSevere::ParamLengthTooSmall) => 0xC017,
            Severe(PukclReturnCodeSevere::DivisionByZero) => 0xC101,
            Severe(PukclReturnCodeSevere::MalformedModulus) => 0xC102,
            Severe(PukclReturnCodeSevere::FaultDetected) => 0xC103,
            Severe(PukclReturnCodeSevere::MalformedKey) => 0xC104,
            Severe(PukclReturnCodeSevere::AprioriOk) => 0xC105,
            Severe(PukclReturnCodeSevere::WrongService) => 0xC106,
            Warning(PukclReturnCodeWarning::PointAtInfinity) => 0x8001,
            Warning(PukclReturnCodeWarning::WrongSignature) => 0x8002,
            Warning(PukclReturnCodeWarning::WrongSelectnumber) => 0x8003,
            Warning(PukclReturnCodeWarning::PointIsNotOnCurve) => 0x8004,
            Info(PukclReturnCodeInfo::NumberIsNotPrime) => 0x4001,
            Info(PukclReturnCodeInfo::NumberIsPrime) => 0x4002,
            Unknown { code } => code,
        })
    }
}

/// An enum type that is a human readable representation of a low-level
/// [`c_abi::PukclReturnCode`] type. Useful when used together with a [`Debug`]
/// traits and formatters.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PukclReturnCode {
    Ok,
    Info(PukclReturnCodeInfo),
    Warning(PukclReturnCodeWarning),
    Severe(PukclReturnCodeSevere),
    Unknown { code: u16 },
}

/// [`PukclReturnCode`] nested enum subtype
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PukclReturnCodeInfo {
    NumberIsNotPrime,
    NumberIsPrime,
}

/// [`PukclReturnCode`] nested enum subtype
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PukclReturnCodeWarning {
    PointAtInfinity,
    WrongSignature,
    WrongSelectnumber,
    PointIsNotOnCurve,
}

/// [`PukclReturnCode`] nested enum subtype
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PukclReturnCodeSevere {
    ComputationNotStarted,
    UnknownService,
    UnexploitableOptions,
    HardwareIssue,
    WrongHardware,
    LibraryMalformed,
    Error,
    UnknownSubservice,
    OverlapNotAllowed,
    ParamNotInPukccram,
    ParamNotInRam,
    ParamNotInCpuram,
    ParamWrongLength,
    ParamBadAlignment,
    ParamXBiggerThanY,
    ParamLengthTooSmall,
    DivisionByZero,
    MalformedModulus,
    FaultDetected,
    MalformedKey,
    AprioriOk,
    WrongService,
}

fn padding_for_len(len: usize) -> usize {
    const ALIGNMENT: usize = 4;
    if len % ALIGNMENT != 0 {
        ALIGNMENT - (len % ALIGNMENT)
    } else {
        0
    }
}

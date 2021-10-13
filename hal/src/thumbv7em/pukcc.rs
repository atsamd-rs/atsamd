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

pub mod c_abi;
pub mod curves;

use crate::pac::MCLK;
use c_abi::{u4, CryptoRamSlice, Service};
use curves::Curve;

use rand_core::{CryptoRng, RngCore};

/// This macro linearly copies provided iterable slices/arrays to CryptoRAM and
/// assigns slices to provided declared local variables from outer scope
///
/// All parameters are stored in big endian format. PUKCC algorithms require
/// little endian format. This is solved by copying data in reverse within a
/// macro.
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
            $(
                .chain($data.iter().rev()) // > Little endian format! <
            )+
            .zip($crypto_ram.iter_mut())
            .for_each(|(data_iter, cr_iter)| *cr_iter = *data_iter);

            let mut _offset = 0;
            $(
                $name = &$crypto_ram[_offset.._offset + $data.len()];
                _offset += $data.len();
            )+
            }
        }
}

/// Struct representing a PUKCC peripheral.
///
/// It provides an access to cryptograhic algorithms in a safe, high-level
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
    /// Unsafe: `k` value must be cryptograhically secure.
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
        match C::verify_curve() {
            Err(e) => return Err(EcdsaSignFailure::InvalidCurve(e)),
            _ => {}
        };

        if signature.len() != (2 * C::MOD_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameter {
                faulty_slice: "signature",
                expected_length: (2 * C::MOD_LENGTH).into(),
                actual_length: signature.len(),
            });
        }
        if hash.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameter {
                faulty_slice: "hash",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: hash.len(),
            });
        }
        if private_key.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameter {
                faulty_slice: "private_key",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: private_key.len(),
            });
        }
        if k.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignFailure::WrongInputParameter {
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
        // PUKCC algorithms. Endianess of parameters themselves is resolved
        // within macro. Little endianess requires padding *after* a parameter
        // as MSB is placed on high addresses.

        // 32-byte zero padding for curve parameters should be included in original
        // slices.
        copy_to_cryptoram! {
            crypto_ram,
            (modulo_p, C::MODULO_P),
            (a_curve, C::A_CURVE),
            (base_point_a_x, C::BASE_POINT_A_X),
            (base_point_a_y, C::BASE_POINT_A_Y),
            (base_point_a_z, C::BASE_POINT_A_Z),
            (order_point, C::ORDER_POINT),
            (cns, C::CNS),
            (hash_cr, hash),
            (__, &[0_u8; 4]),
            (private_key_cr, private_key),
            (__, &[0_u8; 4]),
            (k_cr, k),
            (__, &[0_u8; 4]),
            // Workspace is just marked with a zero length slice just to get its address. As
            // it is placed at the end, idea is that algorithm will use whatever amount of
            // memory it needs
            (workspace, &[0_u8; 0])
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
        match C::verify_curve() {
            Err(e) => return Err(EcdsaSignatureVerificationFailure::InvalidCurve(e)),
            _ => {}
        };

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
            return Err(EcdsaSignatureVerificationFailure::WrongInputParameter {
                faulty_slice: "signature",
                expected_length: (2 * C::SCALAR_LENGTH).into(),
                actual_length: signature.len(),
            });
        }
        if hash.len() != (C::SCALAR_LENGTH).into() {
            return Err(EcdsaSignatureVerificationFailure::WrongInputParameter {
                faulty_slice: "hash",
                expected_length: (C::SCALAR_LENGTH).into(),
                actual_length: hash.len(),
            });
        }
        if public_key.len() != (2 * C::MOD_LENGTH).into() {
            return Err(EcdsaSignatureVerificationFailure::WrongInputParameter {
                faulty_slice: "public_key",
                expected_length: (2 * C::MOD_LENGTH).into(),
                actual_length: public_key.len(),
            });
        }
        let mut crypto_ram = unsafe { c_abi::CryptoRam::new() };
        // 32-byte padding with zeroes on a MSB side of every parameter is required by
        // PUKCC algorithms. Endianess of parameters themselves is resolved
        // within macro. Little endianess requires padding *after* a parameter
        // as MSB is placed on high addresses.

        // 32-byte zero padding for curve parameters should be included in original
        // slices.
        copy_to_cryptoram! {
            crypto_ram,
            (modulo_p, C::MODULO_P),
            (a_curve, C::A_CURVE),
            (base_point_a_x, C::BASE_POINT_A_X),
            (__, C::BASE_POINT_A_Y),
            (__, C::BASE_POINT_A_Z),
            (order_point, C::ORDER_POINT),
            (cns, C::CNS),
            // Signature has to be split into two parts + padding must be added
            // Signature layout:
            //   [ R: (little endian) ][ 0_u32 ]..
            // ..[ S: (little endian) ][ 0_u32 ]
            (signature_cr, signature[..C::SCALAR_LENGTH.into()]),
            (__, &[0_u8; 4]),
            (__, signature[C::SCALAR_LENGTH.into()..(2*C::SCALAR_LENGTH).into()]),
            (__, &[0_u8; 4]),
            (hash_cr, hash),
            (__, &[0_u8; 4]),
            // Public key has to be represented as a point + padding must be added
            // Public key layout:
            //   [ X coordinate: (little endian) ][ 0_u32 ]..
            // ..[ Y coordinate: (little endian) ][ 0_u32 ]
            // ..[ Z coordinate: (little endian) ][ 0_u32 ]
            (public_key_cr, public_key[..C::MOD_LENGTH.into()]),
            (__, &[0_u8; 4]),
            (__, public_key[C::MOD_LENGTH.into()..(2*C::MOD_LENGTH).into()]),
            (__, &[0_u8; 4]),
            // Public Key Z coordinate == 1
            // Workaround: Reusing base point Z coordinate as it also == 1
            // and has the same length
            (__, C::BASE_POINT_A_Z),
            (__, &[0_u8; 4]),
            // Workspace is just marked with a zero length slice just to get its address. As
            // it is placed at the end, idea is that algorithm will use whatever amount of
            // memory it needs
            (workspace, &[0_u8; 0])
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
}

/// An error type representing failure modes a [`Pukcc::self_test`] service
#[derive(Debug)]
pub struct SelfTestFailure(c_abi::SelfTest);

/// An error type representing failure modes for a [`Pukcc::zp_ecdsa_sign`]
/// service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EcdsaSignFailure {
    WrongInputParameter {
        faulty_slice: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
    InvalidCurve(curves::CurveVerficationFailure),
    BasePointZCoordinateIsNotZero,
    ServiceFailure(PukclReturnCode),
}

/// An error type representing failure modes for a
/// [`Pukcc::zp_ecdsa_verify_signature`] service
#[allow(missing_docs)]
#[derive(Debug)]
pub enum EcdsaSignatureVerificationFailure {
    WrongInputParameter {
        faulty_slice: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
    InvalidCurve(curves::CurveVerficationFailure),
    ServiceFailure(PukclReturnCode),
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
            0xC015 => Severe(PukclReturnCodeSevere::ParamBadAlignement),
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
            Severe(PukclReturnCodeSevere::ParamBadAlignement) => 0xC015,
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
    ParamBadAlignement,
    ParamXBiggerThanY,
    ParamLengthTooSmall,
    DivisionByZero,
    MalformedModulus,
    FaultDetected,
    MalformedKey,
    AprioriOk,
    WrongService,
}

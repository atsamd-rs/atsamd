//! Module that defines low-level constructs required for interaction with
//! PUKCC
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bitfield::bitfield;

pub type nu1 = u16;
pub type u2 = u16;
pub type u4 = u32;
pub type pu1 = u32;
pub type u1 = u8;
pub type pfu1 = u32;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Smult {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1XBase: nu1,
    pub u2RLength: u2,
    pub nu1ZBase: nu1,
    pub nu1RBase: nu1,
    pub u2XLength: u2,
    pub u4MulValue: u4,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEccAddSubFast {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointABase: nu1,
    pub nu1PointBBase: nu1,
    pub nu1Workspace: nu1,
    pub __Padding1: nu1,
    pub u2Operator: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEccMulFast {
    pub nu1PointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1KBase: nu1,
    pub nu1ABase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2KLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fill {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub __Padding5: u2,
    pub nu1RBase: nu1,
    pub u2RLength: u2,
    pub u4FillValue: u4,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct RedMod {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1RBase: nu1,
    pub __Padding0: u2,
    pub __Padding1: u2,
    pub nu1XBase: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Swap {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,
    pub nu1XBase: nu1,
    pub nu1YBase: nu1,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub __Padding5: u2,
    pub u2XLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcDsaGenerateFast {
    pub nu1PointABase: nu1,
    pub nu1OrderPointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PrivateKey: nu1,
    pub nu1ScalarNumber: nu1,
    pub nu1ABase: nu1,
    pub nu1HashBase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2ScalarLength: u2,
    pub __Padding0: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcDsaVerifyFast {
    pub nu1PointABase: nu1,
    pub nu1OrderPointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PointPublicKeyGen: nu1,
    pub nu1PointSignature: nu1,
    pub nu1ABase: nu1,
    pub nu1HashBase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2ScalarLength: u2,
    pub __Padding0: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcDsaQuickVerify {
    pub pu1ModCnsBase: pu1,
    pub pu1PointABase: pu1,
    pub pu1PointPublicKeyGen: pu1,
    pub pu1PointSignature: pu1,
    pub pu1OrderPointBase: pu1,
    pub pu1AWorkBase: pu1,
    pub pu1HashBase: pu1,
    pub u2ModLength: u2,
    pub u2ScalarLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SelfTest {
    pub u4Version: u4,
    pub u4PUKCCVersion: u4,
    pub u4CheckNum1: u4,
    pub u4CheckNum2: u4,
    pub u1Step: u1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ClearFlags {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub __Padding5: u2,
    pub __Padding6: u2,
    pub __Padding7: u2,
    pub __Padding8: u2,
    pub __Padding9: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEccMulFast {
    pub nu1PointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1KBase: nu1,
    pub nu1ABase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2KLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Square {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1XBase: nu1,
    pub __Padding0: u2,
    pub nu1ZBase: nu1,
    pub nu1RBase: nu1,
    pub __Padding1: u2,
    pub u2XLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Fmult {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1XBase: nu1,
    pub nu1YBase: nu1,
    pub nu1ZBase: nu1,
    pub nu1RBase: nu1,
    pub u2XLength: u2,
    pub u2YLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct PrimeGen {
    pub nu1RndBase: nu1,
    pub nu1NBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PrecompBase: nu1,
    pub nu1RBase: nu1,
    pub nu1ExpBase: nu1,
    pub u2NLength: u2,
    pub __Padding0: nu1,
    pub u1MillerRabinIterations: u1,
    pub __Padding1: u1,
    pub u2MaxIncrement: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct CondCopy {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,
    pub nu1XBase: nu1,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub nu1RBase: nu1,
    pub u2RLength: u2,
    pub u2XLength: u2,
    pub __Padding5: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Div {
    pub nu1ModBase: nu1,
    pub nu1WorkSpace: nu1,
    pub u2ModLength: u2,
    pub nu1RBase: nu1,
    pub __Padding0: u2,
    pub nu1QuoBase: nu1,
    pub nu1NumBase: nu1,
    pub __Padding1: u2,
    pub __Padding2: u2,
    pub u2NumLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEccAddFast {
    pub nu1PointABase: nu1,
    pub nu1PointBBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1Workspace: nu1,
    pub nu1ABBase: nu1,
    pub __Padding1: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Comp {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,
    pub nu1XBase: nu1,
    pub nu1YBase: nu1,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub u2YLength: u2,
    pub u2XLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEccAddFast {
    pub nu1PointABase: nu1,
    pub nu1PointBBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1Workspace: nu1,
    pub __Padding0: nu1,
    pub __Padding1: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcDsaGenerateFast {
    pub nu1PointABase: nu1,
    pub nu1OrderPointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PrivateKey: nu1,
    pub nu1ScalarNumber: nu1,
    pub nu1ABase: nu1,
    pub nu1HashBase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2ScalarLength: u2,
    pub __Padding0: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcDsaVerifyFast {
    pub nu1PointABase: nu1,
    pub nu1OrderPointBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PointPublicKeyGen: nu1,
    pub nu1PointSignature: nu1,
    pub nu1ABase: nu1,
    pub nu1HashBase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub u2ScalarLength: u2,
    pub __Padding0: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct CRT {
    pub nu1XBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1PrecompBase: nu1,
    pub __Padding0: nu1,
    pub pfu1ExpBase: pfu1,
    pub u2ModLength: u2,
    pub u2ExpLength: u2,
    pub u1Blinding: u1,
    pub __Padding1: u1,
    pub __Padding2: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Rng {
    pub nu1XKeyBase: nu1,
    pub nu1WorkSpace: nu1,
    pub u2XKeyLength: u2,
    pub nu1XSeedBase: nu1,
    pub nu1WorkSpace2: nu1,
    pub nu1QBase: nu1,
    pub nu1RBase: nu1,
    pub u2RLength: u2,
    pub u2X9_31Rounds: u2,
    pub __Padding1: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GCD {
    pub __Padding0: u2,
    pub nu1WorkSpace: nu1,
    pub __Padding1: u2,
    pub nu1XBase: nu1,
    pub nu1YBase: nu1,
    pub nu1ABase: nu1,
    pub nu1ZBase: nu1,
    pub u2Length: u2,
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PukclStatus(u32);
    impl Debug;
    u32;
    carry_in, _: 0;
    carry_out, _: 1;
    zero, _: 2;
    gf2n, _: 3;
    violation, _: 4;
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct PukclHeader {
    pub u1Service: u1,
    pub u1SubService: u1,
    pub u2Option: u2,
    pub Specific: PukclStatus,
    pub u2Status: PukclReturnCode,
    pub __Padding0: u2,
    pub __Padding1: u4,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PukclReturnCode(pub u2);

impl core::fmt::Debug for PukclReturnCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        super::PukclReturnCode::from(*self).fmt(f)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PukclParamsUnion {
    pub ClearFlags: ClearFlags,
    pub Comp: Comp,
    pub CondCopy: CondCopy,
    pub CRT: CRT,
    pub Div: Div,
    pub ExpMod: ExpMod,
    pub FastCopy: FastCopy,
    pub Fill: Fill,
    pub Fmult: Fmult,
    pub GCD: GCD,
    pub PrimeGen: PrimeGen,
    pub RedMod: RedMod,
    pub Rng: Rng,
    pub SelfTest: SelfTest,
    pub Smult: Smult,
    pub Square: Square,
    pub Swap: Swap,

    /* ECC on Prime Field */
    pub ZpEccAddFast: ZpEccAddFast,
    pub ZpEccDblFast: ZpEccDblFast,
    pub ZpEccAddSubFast: ZpEccAddSubFast,
    pub ZpEccMulFast: ZpEccMulFast,
    pub ZpEcDsaGenerateFast: ZpEcDsaGenerateFast,
    pub ZpEcDsaVerifyFast: ZpEcDsaVerifyFast,
    pub ZpEcDsaQuickVerify: ZpEcDsaQuickVerify,
    pub ZpEccQuickDualMulFast: ZpEccQuickDualMulFast,
    pub ZpEcConvProjToAffine: ZpEcConvProjToAffine,
    pub ZpEcConvAffineToProjective: ZpEcConvAffineToProjective,
    pub ZpEcRandomiseCoordinate: ZpEcRandomiseCoordinate,
    pub ZpEcPointIsOnCurve: ZpEcPointIsOnCurve,

    /* ECC on Binary Field */
    pub GF2NEccAddFast: GF2NEccAddFast,
    pub GF2NEccDblFast: GF2NEccDblFast,
    pub GF2NEccMulFast: GF2NEccMulFast,
    pub GF2NEcDsaGenerateFast: GF2NEcDsaGenerateFast,
    pub GF2NEcDsaVerifyFast: GF2NEcDsaVerifyFast,
    pub GF2NEcConvProjToAffine: GF2NEcConvProjToAffine,
    pub GF2NEcConvAffineToProjective: GF2NEcConvAffineToProjective,
    pub GF2NEcRandomiseCoordinate: GF2NEcRandomiseCoordinate,
    pub GF2NEcPointIsOnCurve: GF2NEcPointIsOnCurve,
}

impl core::default::Default for PukclParamsUnion {
    fn default() -> Self {
        Self {
            ClearFlags: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PukclParams {
    pub header: PukclHeader,
    pub params: PukclParamsUnion,
}

impl core::fmt::Debug for PukclParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PukclParams")
            .field("header", &self.header)
            .field(
                "params",
                match debug_union_matcher(self) {
                    Ok(field) => field,
                    Err(_) => &"invalid service",
                },
            )
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEccQuickDualMulFast {
    pub pu1ModCnsBase: pu1,
    pub pu1PointABase: pu1,
    pub pu1PointBBase: pu1,
    pub pu1KABBase: pu1,
    pub pu1AWorkBase: pu1,
    pub u2ModLength: u2,
    pub u2KLength: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEccDblFast {
    pub nu1PointABase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1ABBase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub __Padding0: nu1,
    pub __Padding1: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEccDblFast {
    pub nu1PointABase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1ABase: nu1,
    pub nu1Workspace: nu1,
    pub u2ModLength: u2,
    pub __Padding0: nu1,
    pub __Padding1: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ExpMod {
    pub nu1XBase: nu1,
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub nu1PrecompBase: nu1,
    pub pfu1ExpBase: pfu1,
    pub u2ModLength: u2,
    pub u2ExpLength: u2,
    pub u1Blinding: u1,
    pub __Padding0: u1,
    pub __Padding1: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcConvProjToAffine {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointABase: nu1,
    pub __Padding0: nu1,
    pub nu1Workspace: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcConvAffineToProjective {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointABase: nu1,
    pub __Padding0: nu1,
    pub nu1Workspace: nu1,
    pub __Padding1: nu1,
    pub __Padding2: nu1,
    pub __Padding3: nu1,
    pub __Padding4: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcPointIsOnCurve {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1AParam: nu1,
    pub nu1BParam: nu1,
    pub nu1PointBase: nu1,
    pub nu1Workspace: nu1,
    pub __Padding0: u2,
    pub __Padding1: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct GF2NEcRandomiseCoordinate {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointBase: nu1,
    pub nu1RandomBase: nu1,
    pub nu1Workspace: nu1,
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: nu1,
    pub __Padding3: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct FastCopy {
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: u2,

    pub nu1XBase: nu1,
    pub __Padding3: u2,
    pub __Padding4: u2,
    pub nu1RBase: nu1,
    pub u2RLength: u2,
    pub u2XLength: u2,
    pub __Padding5: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcConvProjToAffine {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointABase: nu1,
    pub __Padding0: nu1,
    pub nu1Workspace: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcConvAffineToProjective {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointABase: nu1,
    pub __Padding0: nu1,
    pub nu1Workspace: nu1,
    pub __Padding1: nu1,
    pub __Padding2: nu1,
    pub __Padding3: nu1,
    pub __Padding4: nu1,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcPointIsOnCurve {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1AParam: nu1,
    pub nu1BParam: nu1,
    pub nu1PointBase: nu1,
    pub nu1Workspace: nu1,
    pub __Padding0: u2,
    pub __Padding1: u2,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ZpEcRandomiseCoordinate {
    pub nu1ModBase: nu1,
    pub nu1CnsBase: nu1,
    pub u2ModLength: u2,
    pub nu1PointBase: nu1,
    pub nu1RandomBase: nu1,
    pub nu1Workspace: nu1,
    pub __Padding0: nu1,
    pub __Padding1: nu1,
    pub __Padding2: nu1,
    pub __Padding3: nu1,
}

pub trait Service: crate::typelevel::Sealed {
    const SERVICE_NUM: u8;
    const FUNCTION_ADDRESS: usize;
    /// Call to PUKCC functions
    ///
    /// # Safety
    ///
    /// User must ensure that [`PukclParams`] is correctly initialised
    /// according to the service being called
    unsafe fn call(pukcl_params: &mut PukclParams) {
        pukcl_params.header.u1Service = Self::SERVICE_NUM;
        pukcl_params.header.u2Status =
            super::PukclReturnCode::Severe(super::PukclReturnCodeSevere::ComputationNotStarted)
                .into();
        core::mem::transmute::<_, extern "C" fn(*mut PukclParams)>(Self::FUNCTION_ADDRESS)(
            pukcl_params,
        )
    }
}

const JUMP_TABLE_START: usize = 0x02000001;

/// Macro implementing a concept of jump table for all the services provided by
/// PUKCC
macro_rules! services {
        (
            $(
                ($service_num:literal, $jump_table_offset:literal, $service:ident)
            ),+
        ) =>
        {
            // TODO: Maybe there's a better way of doing this?
            /// A function that provides matching for a service identifier with an union variant
            /// Used in a custom [`Debug`] trait implementation for a [`PukclParams`]
            fn debug_union_matcher(pukcl_params: &PukclParams) -> Result<&dyn core::fmt::Debug, u8> {
                match pukcl_params.header.u1Service {
                    $(
                        $service_num => Ok(unsafe { &pukcl_params.params.$service }),
                    )+
                    unknown_value => Err(unknown_value)
                }
            }
            $(
                impl Service for $service {
                    const SERVICE_NUM: u8 = $service_num;
                    const FUNCTION_ADDRESS: usize = JUMP_TABLE_START + $jump_table_offset;
                }
                impl crate::typelevel::Sealed for $service {}
            )+
        }
    }

services!(
    (0x50, 0x08, RedMod),
    (0x51, 0x0c, CondCopy),
    (0x52, 0x50, Div),
    (0x53, 0x28, ZpEcDsaGenerateFast),
    (0x54, 0x04, GF2NEcRandomiseCoordinate),
    (0x55, 0x2c, ZpEcDsaVerifyFast),
    (0x56, 0x84, ZpEcConvProjToAffine),
    (0x57, 0x6c, GF2NEcPointIsOnCurve),
    (0x58, 0x5c, CRT),
    (0x59, 0x30, GF2NEccAddFast),
    (0x5b, 0x54, SelfTest),
    (0x5c, 0x60, FastCopy),
    (0x5d, 0x1c, GCD),
    (0x5e, 0x78, ZpEcRandomiseCoordinate),
    (0x5f, 0x10, ClearFlags),
    (0x60, 0x34, ZpEccDblFast),
    (0x61, 0x68, ZpEcConvAffineToProjective),
    (0x62, 0x70, Rng),
    (0x63, 0x74, Swap),
    (0x64, 0x20, GF2NEccMulFast),
    (0x65, 0x40, ZpEccMulFast),
    (0x66, 0x38, ZpEccAddFast),
    (0x67, 0x48, Smult),
    (0x68, 0x8c, ZpEcPointIsOnCurve),
    (0x69, 0x14, GF2NEccDblFast),
    (0x6b, 0x24, Comp),
    (0x6c, 0x80, ExpMod),
    (0x6d, 0x4c, Square),
    (0x6e, 0x58, PrimeGen),
    (0x6f, 0x3c, Fill),
    (0x70, 0x64, GF2NEcDsaGenerateFast),
    (0x71, 0x18, Fmult),
    (0x72, 0x88, GF2NEcConvProjToAffine),
    (0x73, 0x7c, GF2NEcConvAffineToProjective),
    (0x74, 0x44, GF2NEcDsaVerifyFast),
    (0x75, 0x94, ZpEccAddSubFast),
    (0x76, 0x98, ZpEccQuickDualMulFast),
    (0x77, 0x9c, ZpEcDsaQuickVerify)
);

/// Function that has to be called before using PUKCC peripheral
///
/// # Safety
///
/// Defensively put as unsafe
///
/// Only useful for low-level ABI calls
pub unsafe fn wait_for_crypto_ram_clear_process() {
    const PUKCCSR: *mut u32 = 0x4200302C as _;
    const BIT_PUKCCSR_CLRRAM_BUSY: u32 = 0x1;

    while core::ptr::read_volatile(PUKCCSR) & BIT_PUKCCSR_CLRRAM_BUSY != 0 {}
}

/// Slice wrapper that provides Rust-like access to CryptoRAM memory area
pub struct CryptoRam(&'static mut [u8]);

impl CryptoRam {
    const CRYPTORAM_BASE: *mut u8 = 0x02011000 as _;
    const CRYPTORAM_LENGTH: usize = 0x1000;
    /// Create CryptoRam
    ///
    /// # Safety
    ///
    /// Might break aliasing rules if more than one instance is used.
    ///
    /// Only to be used exclusively for low-level access,
    /// never together with the high level API.
    pub unsafe fn new() -> Self {
        Self(core::slice::from_raw_parts_mut(
            Self::CRYPTORAM_BASE,
            Self::CRYPTORAM_LENGTH,
        ))
    }
}

/// Trait implemented for all `&[u8]` slices in order to provide a normalized
/// way of downcasting pointers in a form accepted by PUKCC ABI
pub trait CryptoRamSlice: crate::typelevel::Sealed {
    /// Take a slice base-pointer and mask out a halfword according to
    /// the expected format of CryptoRAM pointers in [`PukclParams`]-related
    /// structs.
    ///
    /// # Safety
    ///
    /// Might break aliasing rules if more than one instance is used.
    ///
    /// Only to be used exclusively for low-level access,
    /// never together with the high level API.
    unsafe fn pukcc_base(&self) -> nu1;
}

impl CryptoRamSlice for &[u8] {
    unsafe fn pukcc_base(&self) -> nu1 {
        (self.as_ptr() as usize & 0x0000FFFF).try_into().unwrap()
    }
}

impl crate::typelevel::Sealed for &[u8] {}

impl core::ops::Deref for CryptoRam {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl core::ops::DerefMut for CryptoRam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

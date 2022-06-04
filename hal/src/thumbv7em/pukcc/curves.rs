//! Module that defines curves parametrizations

/// A type representing a standard curve defined by National Institute of
/// Standards and Technology (variant 256p)
pub enum Nist256p {}

impl Curve for Nist256p {
    const MOD_LENGTH: super::c_abi::u2 = 32;
    const SCALAR_LENGTH: super::c_abi::u2 = 32;
    const MODULO_P: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    const A_CURVE: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC,
    ];

    const B_CURVE: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd,
        0x55, 0x76, 0x98, 0x86, 0xbc, 0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53, 0xb0, 0xf6, 0x3b, 0xce,
        0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
    ];

    const BASE_POINT_A_X: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6,
        0xe5, 0x63, 0xa4, 0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1,
        0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96,
    ];

    const BASE_POINT_A_Y: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb,
        0x4a, 0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6,
        0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
    ];

    const BASE_POINT_A_Z: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];

    const ORDER_POINT: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9,
        0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
    ];

    const CNS: &'static [u8] = &[
        0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFD, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
}

/// A trait that generalizes over a curve concept.
///
/// General equation of a curve is:
/// `y^2 = x^3 + a*x + b`
///
/// Provides all the parametrizations through associated constants.
///
/// Associated constant slices must incorporate zero padding required by PUKCC.
///
/// Const generics are limited. It is impossible to have const arrays with a
/// length as a separate const parameter. Therefore slices are used instead and
/// length verification is moved to runtime ([`Curve::verify_curve`])
pub trait Curve {
    /// Length of P modulus (bytes)
    const MOD_LENGTH: super::c_abi::u2;
    /// Length of the scalar (bytes)
    const SCALAR_LENGTH: super::c_abi::u2;
    /// P modulus parameter
    /// Length: MOD_LENGTH + 4
    const MODULO_P: &'static [u8];
    /// A parameter of a curve
    /// Length: MOD_LENGTH + 4
    const A_CURVE: &'static [u8];
    /// B parameter of a curve
    /// Length: MOD_LENGTH + 4
    const B_CURVE: &'static [u8];
    /// X coordinate of a base point (point of origin on a curve)
    /// Length: MOD_LENGTH + 4
    const BASE_POINT_A_X: &'static [u8];
    /// Y coordinate of a base point (point of origin on a curve)
    /// Length: MOD_LENGTH + 4
    const BASE_POINT_A_Y: &'static [u8];
    /// Z coordinate of a base point (point of origin on a curve)
    /// It is equal to 1
    /// Length: MOD_LENGTH + 4
    const BASE_POINT_A_Z: &'static [u8];
    /// Order point of the curve
    /// Length: SCALAR_LENGTH + 4
    const ORDER_POINT: &'static [u8];
    /// Modulo reduction constant precalculated with RedMod service in a
    /// SetupConstant mode
    ///
    /// Note:
    /// That CNS value is for services over prime field: GF(p)
    /// For polynomials GF(2^n) it has to be generated separately
    /// Length: SCALAR_LENGTH + 12
    const CNS: &'static [u8];
    /// Function that can be used during runtime to verify if a curve is
    /// correctly defined.
    ///
    /// That is:
    /// - lengths of slices are following the requirements
    /// - slices are 4 aligned
    fn verify_curve() -> Result<(), CurveVerificationFailure> {
        if Self::MOD_LENGTH % 4 != 0 || Self::SCALAR_LENGTH % 4 != 0 {
            return Err(CurveVerificationFailure::LengthsAreNotAlignedTo4);
        }
        if Self::MODULO_P.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "MODULO_P",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::MODULO_P.len(),
            });
        }
        if Self::A_CURVE.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "A_CURVE",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::A_CURVE.len(),
            });
        }
        if Self::B_CURVE.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "B_CURVE",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::B_CURVE.len(),
            });
        }
        if Self::BASE_POINT_A_X.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "BASE_POINT_A_X",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::BASE_POINT_A_X.len(),
            });
        }
        if Self::BASE_POINT_A_Y.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "BASE_POINT_A_Y",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::BASE_POINT_A_Y.len(),
            });
        }
        if Self::BASE_POINT_A_Z.len() != (Self::MOD_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "BASE_POINT_A_Z",
                expected_length: (Self::MOD_LENGTH + 4).into(),
                actual_length: Self::BASE_POINT_A_Z.len(),
            });
        }
        if Self::ORDER_POINT.len() != (Self::SCALAR_LENGTH + 4).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "ORDER_POINT",
                expected_length: (Self::SCALAR_LENGTH + 4).into(),
                actual_length: Self::ORDER_POINT.len(),
            });
        }
        if Self::CNS.len() != (Self::SCALAR_LENGTH + 12).into() {
            return Err(CurveVerificationFailure::IncorrectSliceLength {
                faulty_slice: "CNS",
                expected_length: (Self::SCALAR_LENGTH + 12).into(),
                actual_length: Self::CNS.len(),
            });
        }
        Ok(())
    }
}

/// An error type representing failure modes for a
/// [`Curve::verify_curve`] function
#[allow(missing_docs)]
#[derive(Debug)]
pub enum CurveVerificationFailure {
    IncorrectSliceLength {
        faulty_slice: &'static str,
        expected_length: usize,
        actual_length: usize,
    },
    LengthsAreNotAlignedTo4,
}

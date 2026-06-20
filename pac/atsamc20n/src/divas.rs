#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Dividend"]
    pub dividend: crate::Reg<dividend::DIVIDEND_SPEC>,
    #[doc = "0x0c - Divisor"]
    pub divisor: crate::Reg<divisor::DIVISOR_SPEC>,
    #[doc = "0x10 - Result"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x14 - Remainder"]
    pub rem: crate::Reg<rem::REM_SPEC>,
    #[doc = "0x18 - Square Root Input"]
    pub sqrnum: crate::Reg<sqrnum::SQRNUM_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "DIVIDEND register accessor: an alias for `Reg<DIVIDEND_SPEC>`"]
pub type DIVIDEND = crate::Reg<dividend::DIVIDEND_SPEC>;
#[doc = "Dividend"]
pub mod dividend;
#[doc = "DIVISOR register accessor: an alias for `Reg<DIVISOR_SPEC>`"]
pub type DIVISOR = crate::Reg<divisor::DIVISOR_SPEC>;
#[doc = "Divisor"]
pub mod divisor;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "REM register accessor: an alias for `Reg<REM_SPEC>`"]
pub type REM = crate::Reg<rem::REM_SPEC>;
#[doc = "Remainder"]
pub mod rem;
#[doc = "SQRNUM register accessor: an alias for `Reg<SQRNUM_SPEC>`"]
pub type SQRNUM = crate::Reg<sqrnum::SQRNUM_SPEC>;
#[doc = "Square Root Input"]
pub mod sqrnum;

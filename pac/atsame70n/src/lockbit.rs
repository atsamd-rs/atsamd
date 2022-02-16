#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: crate::Reg<lockbit_word0::LOCKBIT_WORD0_SPEC>,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub lockbit_word1: crate::Reg<lockbit_word1::LOCKBIT_WORD1_SPEC>,
    #[doc = "0x08 - Lock Bits Word 2"]
    pub lockbit_word2: crate::Reg<lockbit_word2::LOCKBIT_WORD2_SPEC>,
    #[doc = "0x0c - Lock Bits Word 3"]
    pub lockbit_word3: crate::Reg<lockbit_word3::LOCKBIT_WORD3_SPEC>,
}
#[doc = "LOCKBIT_WORD0 register accessor: an alias for `Reg<LOCKBIT_WORD0_SPEC>`"]
pub type LOCKBIT_WORD0 = crate::Reg<lockbit_word0::LOCKBIT_WORD0_SPEC>;
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
#[doc = "LOCKBIT_WORD1 register accessor: an alias for `Reg<LOCKBIT_WORD1_SPEC>`"]
pub type LOCKBIT_WORD1 = crate::Reg<lockbit_word1::LOCKBIT_WORD1_SPEC>;
#[doc = "Lock Bits Word 1"]
pub mod lockbit_word1;
#[doc = "LOCKBIT_WORD2 register accessor: an alias for `Reg<LOCKBIT_WORD2_SPEC>`"]
pub type LOCKBIT_WORD2 = crate::Reg<lockbit_word2::LOCKBIT_WORD2_SPEC>;
#[doc = "Lock Bits Word 2"]
pub mod lockbit_word2;
#[doc = "LOCKBIT_WORD3 register accessor: an alias for `Reg<LOCKBIT_WORD3_SPEC>`"]
pub type LOCKBIT_WORD3 = crate::Reg<lockbit_word3::LOCKBIT_WORD3_SPEC>;
#[doc = "Lock Bits Word 3"]
pub mod lockbit_word3;

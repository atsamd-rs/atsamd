#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub aes_cr: crate::Reg<aes_cr::AES_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub aes_mr: crate::Reg<aes_mr::AES_MR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub aes_ier: crate::Reg<aes_ier::AES_IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub aes_idr: crate::Reg<aes_idr::AES_IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub aes_imr: crate::Reg<aes_imr::AES_IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status Register"]
    pub aes_isr: crate::Reg<aes_isr::AES_ISR_SPEC>,
    #[doc = "0x20..0x40 - Key Word Register"]
    pub aes_keywr: [crate::Reg<aes_keywr::AES_KEYWR_SPEC>; 8],
    #[doc = "0x40..0x50 - Input Data Register"]
    pub aes_idatar: [crate::Reg<aes_idatar::AES_IDATAR_SPEC>; 4],
    #[doc = "0x50..0x60 - Output Data Register"]
    pub aes_odatar: [crate::Reg<aes_odatar::AES_ODATAR_SPEC>; 4],
    #[doc = "0x60..0x70 - Initialization Vector Register"]
    pub aes_ivr: [crate::Reg<aes_ivr::AES_IVR_SPEC>; 4],
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    pub aes_aadlenr: crate::Reg<aes_aadlenr::AES_AADLENR_SPEC>,
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    pub aes_clenr: crate::Reg<aes_clenr::AES_CLENR_SPEC>,
    #[doc = "0x78..0x88 - GCM Intermediate Hash Word Register"]
    pub aes_ghashr: [crate::Reg<aes_ghashr::AES_GHASHR_SPEC>; 4],
    #[doc = "0x88..0x98 - GCM Authentication Tag Word Register"]
    pub aes_tagr: [crate::Reg<aes_tagr::AES_TAGR_SPEC>; 4],
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    pub aes_ctrr: crate::Reg<aes_ctrr::AES_CTRR_SPEC>,
    #[doc = "0x9c..0xac - GCM H Word Register"]
    pub aes_gcmhr: [crate::Reg<aes_gcmhr::AES_GCMHR_SPEC>; 4],
}
#[doc = "AES_CR register accessor: an alias for `Reg<AES_CR_SPEC>`"]
pub type AES_CR = crate::Reg<aes_cr::AES_CR_SPEC>;
#[doc = "Control Register"]
pub mod aes_cr;
#[doc = "AES_MR register accessor: an alias for `Reg<AES_MR_SPEC>`"]
pub type AES_MR = crate::Reg<aes_mr::AES_MR_SPEC>;
#[doc = "Mode Register"]
pub mod aes_mr;
#[doc = "AES_IER register accessor: an alias for `Reg<AES_IER_SPEC>`"]
pub type AES_IER = crate::Reg<aes_ier::AES_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod aes_ier;
#[doc = "AES_IDR register accessor: an alias for `Reg<AES_IDR_SPEC>`"]
pub type AES_IDR = crate::Reg<aes_idr::AES_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod aes_idr;
#[doc = "AES_IMR register accessor: an alias for `Reg<AES_IMR_SPEC>`"]
pub type AES_IMR = crate::Reg<aes_imr::AES_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod aes_imr;
#[doc = "AES_ISR register accessor: an alias for `Reg<AES_ISR_SPEC>`"]
pub type AES_ISR = crate::Reg<aes_isr::AES_ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod aes_isr;
#[doc = "AES_KEYWR register accessor: an alias for `Reg<AES_KEYWR_SPEC>`"]
pub type AES_KEYWR = crate::Reg<aes_keywr::AES_KEYWR_SPEC>;
#[doc = "Key Word Register"]
pub mod aes_keywr;
#[doc = "AES_IDATAR register accessor: an alias for `Reg<AES_IDATAR_SPEC>`"]
pub type AES_IDATAR = crate::Reg<aes_idatar::AES_IDATAR_SPEC>;
#[doc = "Input Data Register"]
pub mod aes_idatar;
#[doc = "AES_ODATAR register accessor: an alias for `Reg<AES_ODATAR_SPEC>`"]
pub type AES_ODATAR = crate::Reg<aes_odatar::AES_ODATAR_SPEC>;
#[doc = "Output Data Register"]
pub mod aes_odatar;
#[doc = "AES_IVR register accessor: an alias for `Reg<AES_IVR_SPEC>`"]
pub type AES_IVR = crate::Reg<aes_ivr::AES_IVR_SPEC>;
#[doc = "Initialization Vector Register"]
pub mod aes_ivr;
#[doc = "AES_AADLENR register accessor: an alias for `Reg<AES_AADLENR_SPEC>`"]
pub type AES_AADLENR = crate::Reg<aes_aadlenr::AES_AADLENR_SPEC>;
#[doc = "Additional Authenticated Data Length Register"]
pub mod aes_aadlenr;
#[doc = "AES_CLENR register accessor: an alias for `Reg<AES_CLENR_SPEC>`"]
pub type AES_CLENR = crate::Reg<aes_clenr::AES_CLENR_SPEC>;
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod aes_clenr;
#[doc = "AES_GHASHR register accessor: an alias for `Reg<AES_GHASHR_SPEC>`"]
pub type AES_GHASHR = crate::Reg<aes_ghashr::AES_GHASHR_SPEC>;
#[doc = "GCM Intermediate Hash Word Register"]
pub mod aes_ghashr;
#[doc = "AES_TAGR register accessor: an alias for `Reg<AES_TAGR_SPEC>`"]
pub type AES_TAGR = crate::Reg<aes_tagr::AES_TAGR_SPEC>;
#[doc = "GCM Authentication Tag Word Register"]
pub mod aes_tagr;
#[doc = "AES_CTRR register accessor: an alias for `Reg<AES_CTRR_SPEC>`"]
pub type AES_CTRR = crate::Reg<aes_ctrr::AES_CTRR_SPEC>;
#[doc = "GCM Encryption Counter Value Register"]
pub mod aes_ctrr;
#[doc = "AES_GCMHR register accessor: an alias for `Reg<AES_GCMHR_SPEC>`"]
pub type AES_GCMHR = crate::Reg<aes_gcmhr::AES_GCMHR_SPEC>;
#[doc = "GCM H Word Register"]
pub mod aes_gcmhr;

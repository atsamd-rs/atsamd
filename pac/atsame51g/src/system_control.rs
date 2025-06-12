#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ictr: Ictr,
    actlr: Actlr,
    _reserved2: [u8; 0x0cf4],
    cpuid: Cpuid,
    icsr: Icsr,
    vtor: Vtor,
    aircr: Aircr,
    scr: Scr,
    ccr: Ccr,
    shpr1: Shpr1,
    shpr2: Shpr2,
    shpr3: Shpr3,
    shcsr: Shcsr,
    cfsr: Cfsr,
    hfsr: Hfsr,
    dfsr: Dfsr,
    mmfar: Mmfar,
    bfar: Bfar,
    afsr: Afsr,
    pfr: [Pfr; 2],
    dfr: Dfr,
    adr: Adr,
    mmfr: [Mmfr; 4],
    isar: [Isar; 5],
    _reserved23: [u8; 0x14],
    cpacr: Cpacr,
}
impl RegisterBlock {
    #[doc = "0x04 - Interrupt Controller Type Register"]
    #[inline(always)]
    pub const fn ictr(&self) -> &Ictr {
        &self.ictr
    }
    #[doc = "0x08 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn actlr(&self) -> &Actlr {
        &self.actlr
    }
    #[doc = "0xd00 - CPUID Base Register"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
    #[doc = "0xd04 - Interrupt Control and State Register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &Icsr {
        &self.icsr
    }
    #[doc = "0xd08 - Vector Table Offset Register"]
    #[inline(always)]
    pub const fn vtor(&self) -> &Vtor {
        &self.vtor
    }
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    #[inline(always)]
    pub const fn aircr(&self) -> &Aircr {
        &self.aircr
    }
    #[doc = "0xd10 - System Control Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0xd14 - Configuration and Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0xd18 - System Handler Priority Register 1"]
    #[inline(always)]
    pub const fn shpr1(&self) -> &Shpr1 {
        &self.shpr1
    }
    #[doc = "0xd1c - System Handler Priority Register 2"]
    #[inline(always)]
    pub const fn shpr2(&self) -> &Shpr2 {
        &self.shpr2
    }
    #[doc = "0xd20 - System Handler Priority Register 3"]
    #[inline(always)]
    pub const fn shpr3(&self) -> &Shpr3 {
        &self.shpr3
    }
    #[doc = "0xd24 - System Handler Control and State Register"]
    #[inline(always)]
    pub const fn shcsr(&self) -> &Shcsr {
        &self.shcsr
    }
    #[doc = "0xd28 - Configurable Fault Status Register"]
    #[inline(always)]
    pub const fn cfsr(&self) -> &Cfsr {
        &self.cfsr
    }
    #[doc = "0xd2c - HardFault Status Register"]
    #[inline(always)]
    pub const fn hfsr(&self) -> &Hfsr {
        &self.hfsr
    }
    #[doc = "0xd30 - Debug Fault Status Register"]
    #[inline(always)]
    pub const fn dfsr(&self) -> &Dfsr {
        &self.dfsr
    }
    #[doc = "0xd34 - MemManage Fault Address Register"]
    #[inline(always)]
    pub const fn mmfar(&self) -> &Mmfar {
        &self.mmfar
    }
    #[doc = "0xd38 - BusFault Address Register"]
    #[inline(always)]
    pub const fn bfar(&self) -> &Bfar {
        &self.bfar
    }
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    #[inline(always)]
    pub const fn afsr(&self) -> &Afsr {
        &self.afsr
    }
    #[doc = "0xd40..0xd48 - Processor Feature Register"]
    #[inline(always)]
    pub const fn pfr(&self, n: usize) -> &Pfr {
        &self.pfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd40..0xd48 - Processor Feature Register"]
    #[inline(always)]
    pub fn pfr_iter(&self) -> impl Iterator<Item = &Pfr> {
        self.pfr.iter()
    }
    #[doc = "0xd48 - Debug Feature Register"]
    #[inline(always)]
    pub const fn dfr(&self) -> &Dfr {
        &self.dfr
    }
    #[doc = "0xd4c - Auxiliary Feature Register"]
    #[inline(always)]
    pub const fn adr(&self) -> &Adr {
        &self.adr
    }
    #[doc = "0xd50..0xd60 - Memory Model Feature Register"]
    #[inline(always)]
    pub const fn mmfr(&self, n: usize) -> &Mmfr {
        &self.mmfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd50..0xd60 - Memory Model Feature Register"]
    #[inline(always)]
    pub fn mmfr_iter(&self) -> impl Iterator<Item = &Mmfr> {
        self.mmfr.iter()
    }
    #[doc = "0xd60..0xd74 - Instruction Set Attributes Register"]
    #[inline(always)]
    pub const fn isar(&self, n: usize) -> &Isar {
        &self.isar[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd60..0xd74 - Instruction Set Attributes Register"]
    #[inline(always)]
    pub fn isar_iter(&self) -> impl Iterator<Item = &Isar> {
        self.isar.iter()
    }
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    #[inline(always)]
    pub const fn cpacr(&self) -> &Cpacr {
        &self.cpacr
    }
}
#[doc = "ICTR (r) register accessor: Interrupt Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ictr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictr`] module"]
#[doc(alias = "ICTR")]
pub type Ictr = crate::Reg<ictr::IctrSpec>;
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`] module"]
#[doc(alias = "ACTLR")]
pub type Actlr = crate::Reg<actlr::ActlrSpec>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
#[doc = "CPUID (r) register accessor: CPUID Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`] module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: Interrupt Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`] module"]
#[doc(alias = "ICSR")]
pub type Icsr = crate::Reg<icsr::IcsrSpec>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: Vector Table Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtor`] module"]
#[doc(alias = "VTOR")]
pub type Vtor = crate::Reg<vtor::VtorSpec>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: Application Interrupt and Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aircr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aircr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aircr`] module"]
#[doc(alias = "AIRCR")]
pub type Aircr = crate::Reg<aircr::AircrSpec>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR (rw) register accessor: System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR (rw) register accessor: Configuration and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: System Handler Priority Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr1`] module"]
#[doc(alias = "SHPR1")]
pub type Shpr1 = crate::Reg<shpr1::Shpr1Spec>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: System Handler Priority Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr2`] module"]
#[doc(alias = "SHPR2")]
pub type Shpr2 = crate::Reg<shpr2::Shpr2Spec>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: System Handler Priority Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpr3`] module"]
#[doc(alias = "SHPR3")]
pub type Shpr3 = crate::Reg<shpr3::Shpr3Spec>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: System Handler Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcsr`] module"]
#[doc(alias = "SHCSR")]
pub type Shcsr = crate::Reg<shcsr::ShcsrSpec>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: Configurable Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfsr`] module"]
#[doc(alias = "CFSR")]
pub type Cfsr = crate::Reg<cfsr::CfsrSpec>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: HardFault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfsr`] module"]
#[doc(alias = "HFSR")]
pub type Hfsr = crate::Reg<hfsr::HfsrSpec>;
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: Debug Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsr`] module"]
#[doc(alias = "DFSR")]
pub type Dfsr = crate::Reg<dfsr::DfsrSpec>;
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MMFAR (rw) register accessor: MemManage Fault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmfar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmfar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmfar`] module"]
#[doc(alias = "MMFAR")]
pub type Mmfar = crate::Reg<mmfar::MmfarSpec>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: BusFault Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfar`] module"]
#[doc(alias = "BFAR")]
pub type Bfar = crate::Reg<bfar::BfarSpec>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR (rw) register accessor: Auxiliary Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsr`] module"]
#[doc(alias = "AFSR")]
pub type Afsr = crate::Reg<afsr::AfsrSpec>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "PFR (rw) register accessor: Processor Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`] module"]
#[doc(alias = "PFR")]
pub type Pfr = crate::Reg<pfr::PfrSpec>;
#[doc = "Processor Feature Register"]
pub mod pfr;
#[doc = "DFR (r) register accessor: Debug Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfr`] module"]
#[doc(alias = "DFR")]
pub type Dfr = crate::Reg<dfr::DfrSpec>;
#[doc = "Debug Feature Register"]
pub mod dfr;
#[doc = "ADR (r) register accessor: Auxiliary Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adr`] module"]
#[doc(alias = "ADR")]
pub type Adr = crate::Reg<adr::AdrSpec>;
#[doc = "Auxiliary Feature Register"]
pub mod adr;
#[doc = "MMFR (r) register accessor: Memory Model Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmfr`] module"]
#[doc(alias = "MMFR")]
pub type Mmfr = crate::Reg<mmfr::MmfrSpec>;
#[doc = "Memory Model Feature Register"]
pub mod mmfr;
#[doc = "ISAR (r) register accessor: Instruction Set Attributes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isar`] module"]
#[doc(alias = "ISAR")]
pub type Isar = crate::Reg<isar::IsarSpec>;
#[doc = "Instruction Set Attributes Register"]
pub mod isar;
#[doc = "CPACR (rw) register accessor: Coprocessor Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpacr`] module"]
#[doc(alias = "CPACR")]
pub type Cpacr = crate::Reg<cpacr::CpacrSpec>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;

#[repr(C)]
#[doc = "SA\\[%s\\]"]
#[doc(alias = "SA")]
pub struct Sa {
    sab: Sab,
    sat: Sat,
}
impl Sa {
    #[doc = "0x00 - Specific Address Bottom \\[31:0\\]
Register"]
    #[inline(always)]
    pub const fn sab(&self) -> &Sab {
        &self.sab
    }
    #[doc = "0x04 - Specific Address Top \\[47:32\\]
Register"]
    #[inline(always)]
    pub const fn sat(&self) -> &Sat {
        &self.sat
    }
}
#[doc = "SAB (rw) register accessor: Specific Address Bottom \\[31:0\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sab`]
module"]
#[doc(alias = "SAB")]
pub type Sab = crate::Reg<sab::SabSpec>;
#[doc = "Specific Address Bottom \\[31:0\\]
Register"]
pub mod sab;
#[doc = "SAT (rw) register accessor: Specific Address Top \\[47:32\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sat`]
module"]
#[doc(alias = "SAT")]
pub type Sat = crate::Reg<sat::SatSpec>;
#[doc = "Specific Address Top \\[47:32\\]
Register"]
pub mod sat;

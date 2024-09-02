#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dir: (),
    _reserved1: [u8; 0x04],
    dirclr: (),
    _reserved2: [u8; 0x04],
    dirset: (),
    _reserved3: [u8; 0x04],
    dirtgl: (),
    _reserved4: [u8; 0x04],
    out: (),
    _reserved5: [u8; 0x04],
    outclr: (),
    _reserved6: [u8; 0x04],
    outset: (),
    _reserved7: [u8; 0x04],
    outtgl: (),
    _reserved8: [u8; 0x04],
    in_: (),
    _reserved9: [u8; 0x04],
    ctrl: (),
    _reserved10: [u8; 0x04],
    wrconfig: (),
    _reserved11: [u8; 0x08],
    pmux0_: [Pmux0_; 16],
    pincfg0_: [Pincfg0_; 32],
    _reserved13: [u8; 0x50],
    pmux1_0: [Pmux1_; 16],
    pincfg1_0: [Pincfg1_; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Data Direction"]
    #[inline(always)]
    pub const fn dir(&self, n: usize) -> &Dir {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Data Direction"]
    #[inline(always)]
    pub fn dir_iter(&self) -> impl Iterator<Item = &Dir> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x04..0x0c - Data Direction Clear"]
    #[inline(always)]
    pub const fn dirclr(&self, n: usize) -> &Dirclr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr_iter(&self) -> impl Iterator<Item = &Dirclr> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x10 - Data Direction Set"]
    #[inline(always)]
    pub const fn dirset(&self, n: usize) -> &Dirset {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - Data Direction Set"]
    #[inline(always)]
    pub fn dirset_iter(&self) -> impl Iterator<Item = &Dirset> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x14 - Data Direction Toggle"]
    #[inline(always)]
    pub const fn dirtgl(&self, n: usize) -> &Dirtgl {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl_iter(&self) -> impl Iterator<Item = &Dirtgl> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x10..0x18 - Data Output Value"]
    #[inline(always)]
    pub const fn out(&self, n: usize) -> &Out {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Data Output Value"]
    #[inline(always)]
    pub fn out_iter(&self) -> impl Iterator<Item = &Out> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x14..0x1c - Data Output Value Clear"]
    #[inline(always)]
    pub const fn outclr(&self, n: usize) -> &Outclr {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Data Output Value Clear"]
    #[inline(always)]
    pub fn outclr_iter(&self) -> impl Iterator<Item = &Outclr> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x18..0x20 - Data Output Value Set"]
    #[inline(always)]
    pub const fn outset(&self, n: usize) -> &Outset {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Data Output Value Set"]
    #[inline(always)]
    pub fn outset_iter(&self) -> impl Iterator<Item = &Outset> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x1c..0x24 - Data Output Value Toggle"]
    #[inline(always)]
    pub const fn outtgl(&self, n: usize) -> &Outtgl {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl_iter(&self) -> impl Iterator<Item = &Outtgl> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x20..0x28 - Data Input Value"]
    #[inline(always)]
    pub const fn in_(&self, n: usize) -> &In {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - Data Input Value"]
    #[inline(always)]
    pub fn in__iter(&self) -> impl Iterator<Item = &In> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x24..0x2c - Control"]
    #[inline(always)]
    pub const fn ctrl(&self, n: usize) -> &Ctrl {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x2c - Control"]
    #[inline(always)]
    pub fn ctrl_iter(&self) -> impl Iterator<Item = &Ctrl> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x28..0x30 - Write Configuration"]
    #[inline(always)]
    pub const fn wrconfig(&self, n: usize) -> &Wrconfig {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x30 - Write Configuration"]
    #[inline(always)]
    pub fn wrconfig_iter(&self) -> impl Iterator<Item = &Wrconfig> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x30..0x40 - Peripheral Multiplexing n - Group 0"]
    #[inline(always)]
    pub const fn pmux0_(&self, n: usize) -> &Pmux0_ {
        &self.pmux0_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Peripheral Multiplexing n - Group 0"]
    #[inline(always)]
    pub fn pmux0__iter(&self) -> impl Iterator<Item = &Pmux0_> {
        self.pmux0_.iter()
    }
    #[doc = "0x40..0x60 - Pin Configuration n - Group 0"]
    #[inline(always)]
    pub const fn pincfg0_(&self, n: usize) -> &Pincfg0_ {
        &self.pincfg0_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Pin Configuration n - Group 0"]
    #[inline(always)]
    pub fn pincfg0__iter(&self) -> impl Iterator<Item = &Pincfg0_> {
        self.pincfg0_.iter()
    }
    #[doc = "0xb0..0xc0 - Peripheral Multiplexing n - Group 1"]
    #[inline(always)]
    pub const fn pmux1_0(&self, n: usize) -> &Pmux1_ {
        &self.pmux1_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xc0 - Peripheral Multiplexing n - Group 1"]
    #[inline(always)]
    pub fn pmux1_0_iter(&self) -> impl Iterator<Item = &Pmux1_> {
        self.pmux1_0.iter()
    }
    #[doc = "0xc0..0xe0 - Pin Configuration n - Group 1"]
    #[inline(always)]
    pub const fn pincfg1_0(&self, n: usize) -> &Pincfg1_ {
        &self.pincfg1_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xe0 - Pin Configuration n - Group 1"]
    #[inline(always)]
    pub fn pincfg1_0_iter(&self) -> impl Iterator<Item = &Pincfg1_> {
        self.pincfg1_0.iter()
    }
}
#[doc = "DIR (rw) register accessor: Data Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR (rw) register accessor: Data Direction Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`]
module"]
#[doc(alias = "DIRCLR")]
pub type Dirclr = crate::Reg<dirclr::DirclrSpec>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET (rw) register accessor: Data Direction Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dirset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`]
module"]
#[doc(alias = "DIRSET")]
pub type Dirset = crate::Reg<dirset::DirsetSpec>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL (rw) register accessor: Data Direction Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtgl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtgl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtgl`]
module"]
#[doc(alias = "DIRTGL")]
pub type Dirtgl = crate::Reg<dirtgl::DirtglSpec>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "OUT (rw) register accessor: Data Output Value\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Data Output Value"]
pub mod out;
#[doc = "OUTCLR (rw) register accessor: Data Output Value Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`outclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outclr`]
module"]
#[doc(alias = "OUTCLR")]
pub type Outclr = crate::Reg<outclr::OutclrSpec>;
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET (rw) register accessor: Data Output Value Set\n\nYou can [`read`](crate::Reg::read) this register and get [`outset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outset`]
module"]
#[doc(alias = "OUTSET")]
pub type Outset = crate::Reg<outset::OutsetSpec>;
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "OUTTGL (rw) register accessor: Data Output Value Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`outtgl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outtgl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outtgl`]
module"]
#[doc(alias = "OUTTGL")]
pub type Outtgl = crate::Reg<outtgl::OuttglSpec>;
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "IN (r) register accessor: Data Input Value\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "WRCONFIG (w) register accessor: Write Configuration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrconfig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrconfig`]
module"]
#[doc(alias = "WRCONFIG")]
pub type Wrconfig = crate::Reg<wrconfig::WrconfigSpec>;
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "PMUX0_ (rw) register accessor: Peripheral Multiplexing n - Group 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux0_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux0_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmux0_`]
module"]
#[doc(alias = "PMUX0_")]
pub type Pmux0_ = crate::Reg<pmux0_::Pmux0_Spec>;
#[doc = "Peripheral Multiplexing n - Group 0"]
pub mod pmux0_;
pub use pmux0_ as pmux1_;
pub use Pmux0_ as Pmux1_;
#[doc = "PINCFG0_ (rw) register accessor: Pin Configuration n - Group 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg0_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg0_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg0_`]
module"]
#[doc(alias = "PINCFG0_")]
pub type Pincfg0_ = crate::Reg<pincfg0_::Pincfg0_Spec>;
#[doc = "Pin Configuration n - Group 0"]
pub mod pincfg0_;
pub use pincfg0_ as pincfg1_;
pub use Pincfg0_ as Pincfg1_;

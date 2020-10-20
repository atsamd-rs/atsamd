#[doc = "Specific Address Bottom \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sab](sab) module"]
pub type SAB = crate::Reg<u32, _SAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAB;
#[doc = "`read()` method returns [sab::R](sab::R) reader structure"]
impl crate::Readable for SAB {}
#[doc = "`write(|w| ..)` method takes [sab::W](sab::W) writer structure"]
impl crate::Writable for SAB {}
#[doc = "Specific Address Bottom \\[31:0\\]
Register"]
pub mod sab;
#[doc = "Specific Address Top \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sat](sat) module"]
pub type SAT = crate::Reg<u32, _SAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAT;
#[doc = "`read()` method returns [sat::R](sat::R) reader structure"]
impl crate::Readable for SAT {}
#[doc = "`write(|w| ..)` method takes [sat::W](sat::W) writer structure"]
impl crate::Writable for SAT {}
#[doc = "Specific Address Top \\[47:32\\]
Register"]
pub mod sat;

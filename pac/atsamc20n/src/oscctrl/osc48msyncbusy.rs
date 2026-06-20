#[doc = "Register `OSC48MSYNCBUSY` reader"]
pub struct R(crate::R<OSC48MSYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC48MSYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC48MSYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC48MSYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSC48MDIV` reader - OSC48MDIV Synchronization Status"]
pub struct OSC48MDIV_R(crate::FieldReader<bool, bool>);
impl OSC48MDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC48MDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC48MDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - OSC48MDIV Synchronization Status"]
    #[inline(always)]
    pub fn osc48mdiv(&self) -> OSC48MDIV_R {
        OSC48MDIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "OSC48M Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc48msyncbusy](index.html) module"]
pub struct OSC48MSYNCBUSY_SPEC;
impl crate::RegisterSpec for OSC48MSYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc48msyncbusy::R](R) reader structure"]
impl crate::Readable for OSC48MSYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC48MSYNCBUSY to value 0"]
impl crate::Resettable for OSC48MSYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

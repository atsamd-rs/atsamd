#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPDCONF` reader - Speed Configuration for Host"]
pub type SPDCONF_R = crate::FieldReader<SPDCONFSELECT_A>;
#[doc = "Speed Configuration for Host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDCONFSELECT_A {
    #[doc = "0: Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "3: Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    FS = 3,
}
impl From<SPDCONFSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONFSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPDCONFSELECT_A {
    type Ux = u8;
}
impl SPDCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPDCONFSELECT_A> {
        match self.bits {
            0 => Some(SPDCONFSELECT_A::NORMAL),
            3 => Some(SPDCONFSELECT_A::FS),
            _ => None,
        }
    }
    #[doc = "Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONFSELECT_A::NORMAL
    }
    #[doc = "Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == SPDCONFSELECT_A::FS
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration for Host"]
pub type SPDCONF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SPDCONFSELECT_A>;
impl<'a, REG, const O: u8> SPDCONF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode:the host starts in full-speed mode and performs a high-speed reset to switch to the high speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::NORMAL)
    }
    #[doc = "Full-speed:the host remains in full-speed mode whatever is the peripheral speed capability. Relevant in UTMI mode only."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(SPDCONFSELECT_A::FS)
    }
}
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TSTJ_R = crate::BitReader;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TSTJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TSTK_R = crate::BitReader;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TSTK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SOFE_R = crate::BitReader;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SOFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSRESET` reader - Send USB Reset"]
pub type BUSRESET_R = crate::BitReader;
#[doc = "Field `BUSRESET` writer - Send USB Reset"]
pub type BUSRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSOK` reader - VBUS is OK"]
pub type VBUSOK_R = crate::BitReader;
#[doc = "Field `VBUSOK` writer - VBUS is OK"]
pub type VBUSOK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L1RESUME` reader - Send L1 Resume"]
pub type L1RESUME_R = crate::BitReader;
#[doc = "Field `L1RESUME` writer - Send L1 Resume"]
pub type L1RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn busreset(&self) -> BUSRESET_R {
        BUSRESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    pub fn vbusok(&self) -> VBUSOK_R {
        VBUSOK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Send USB Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<CTRLB_SPEC, 1> {
        RESUME_W::new(self)
    }
    #[doc = "Bits 2:3 - Speed Configuration for Host"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<CTRLB_SPEC, 2> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TSTJ_W<CTRLB_SPEC, 5> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TSTK_W<CTRLB_SPEC, 6> {
        TSTK_W::new(self)
    }
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<CTRLB_SPEC, 8> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn busreset(&mut self) -> BUSRESET_W<CTRLB_SPEC, 9> {
        BUSRESET_W::new(self)
    }
    #[doc = "Bit 10 - VBUS is OK"]
    #[inline(always)]
    #[must_use]
    pub fn vbusok(&mut self) -> VBUSOK_W<CTRLB_SPEC, 10> {
        VBUSOK_W::new(self)
    }
    #[doc = "Bit 11 - Send L1 Resume"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<CTRLB_SPEC, 11> {
        L1RESUME_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HOST Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LEFTADJ_R = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LEFTADJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FREERUN_R = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FREERUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub type CORREN_R = crate::BitReader;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub type CORREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESSEL` reader - Conversion Result Resolution"]
pub type RESSEL_R = crate::FieldReader<RESSELSELECT_A>;
#[doc = "Conversion Result Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESSELSELECT_A {
    #[doc = "0: 12-bit result"]
    _12BIT = 0,
    #[doc = "1: For averaging mode output"]
    _16BIT = 1,
    #[doc = "2: 10-bit result"]
    _10BIT = 2,
    #[doc = "3: 8-bit result"]
    _8BIT = 3,
}
impl From<RESSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RESSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESSELSELECT_A {
    type Ux = u8;
}
impl RESSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESSELSELECT_A {
        match self.bits {
            0 => RESSELSELECT_A::_12BIT,
            1 => RESSELSELECT_A::_16BIT,
            2 => RESSELSELECT_A::_10BIT,
            3 => RESSELSELECT_A::_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RESSELSELECT_A::_12BIT
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == RESSELSELECT_A::_16BIT
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RESSELSELECT_A::_10BIT
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RESSELSELECT_A::_8BIT
    }
}
#[doc = "Field `RESSEL` writer - Conversion Result Resolution"]
pub type RESSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RESSELSELECT_A>;
impl<'a, REG, const O: u8> RESSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELSELECT_A::_12BIT)
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELSELECT_A::_16BIT)
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELSELECT_A::_10BIT)
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESSELSELECT_A::_8BIT)
    }
}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WINMODE_R = crate::FieldReader<WINMODESELECT_A>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINMODESELECT_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: RESULT > WINLT"]
    MODE1 = 1,
    #[doc = "2: RESULT &lt; WINUT"]
    MODE2 = 2,
    #[doc = "3: WINLT &lt; RESULT &lt; WINUT"]
    MODE3 = 3,
    #[doc = "4: !(WINLT &lt; RESULT &lt; WINUT)"]
    MODE4 = 4,
}
impl From<WINMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINMODESELECT_A {
    type Ux = u8;
}
impl WINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WINMODESELECT_A> {
        match self.bits {
            0 => Some(WINMODESELECT_A::DISABLE),
            1 => Some(WINMODESELECT_A::MODE1),
            2 => Some(WINMODESELECT_A::MODE2),
            3 => Some(WINMODESELECT_A::MODE3),
            4 => Some(WINMODESELECT_A::MODE4),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WINMODESELECT_A::DISABLE
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODESELECT_A::MODE1
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODESELECT_A::MODE2
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODESELECT_A::MODE3
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODESELECT_A::MODE4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WINMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WINMODESELECT_A>;
impl<'a, REG, const O: u8> WINMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::DISABLE)
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE1)
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE2)
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE3)
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE4)
    }
}
#[doc = "Field `WINSS` reader - Window Single Sample"]
pub type WINSS_R = crate::BitReader;
#[doc = "Field `WINSS` writer - Window Single Sample"]
pub type WINSS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CORREN_R {
        CORREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&self) -> WINSS_R {
        WINSS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<CTRLB_SPEC, 0> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<CTRLB_SPEC, 1> {
        FREERUN_W::new(self)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    #[must_use]
    pub fn corren(&mut self) -> CORREN_W<CTRLB_SPEC, 2> {
        CORREN_W::new(self)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> RESSEL_W<CTRLB_SPEC, 3> {
        RESSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<CTRLB_SPEC, 8> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    #[must_use]
    pub fn winss(&mut self) -> WINSS_W<CTRLB_SPEC, 11> {
        WINSS_W::new(self)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

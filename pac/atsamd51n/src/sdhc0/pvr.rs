#[doc = "Register `PVR[%s]` reader"]
pub type R = crate::R<PVR_SPEC>;
#[doc = "Register `PVR[%s]` writer"]
pub type W = crate::W<PVR_SPEC>;
#[doc = "Field `SDCLKFSEL` reader - SDCLK Frequency Select Value for Initialization"]
pub type SDCLKFSEL_R = crate::FieldReader<u16>;
#[doc = "Field `SDCLKFSEL` writer - SDCLK Frequency Select Value for Initialization"]
pub type SDCLKFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `CLKGSEL` reader - Clock Generator Select Value for Initialization"]
pub type CLKGSEL_R = crate::BitReader<CLKGSELSELECT_A>;
#[doc = "Clock Generator Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKGSELSELECT_A {
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    DIV = 0,
    #[doc = "1: Programmable Clock Generator"]
    PROG = 1,
}
impl From<CLKGSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKGSELSELECT_A {
        match self.bits {
            false => CLKGSELSELECT_A::DIV,
            true => CLKGSELSELECT_A::PROG,
        }
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == CLKGSELSELECT_A::DIV
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_prog(&self) -> bool {
        *self == CLKGSELSELECT_A::PROG
    }
}
#[doc = "Field `CLKGSEL` writer - Clock Generator Select Value for Initialization"]
pub type CLKGSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLKGSELSELECT_A>;
impl<'a, REG, const O: u8> CLKGSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator (Divider)"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(CLKGSELSELECT_A::DIV)
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn prog(self) -> &'a mut crate::W<REG> {
        self.variant(CLKGSELSELECT_A::PROG)
    }
}
#[doc = "Field `DRVSEL` reader - Driver Strength Select Value for Initialization"]
pub type DRVSEL_R = crate::FieldReader<DRVSELSELECT_A>;
#[doc = "Driver Strength Select Value for Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRVSELSELECT_A {
    #[doc = "0: Driver Type B is Selected"]
    B = 0,
    #[doc = "1: Driver Type A is Selected"]
    A = 1,
    #[doc = "2: Driver Type C is Selected"]
    C = 2,
    #[doc = "3: Driver Type D is Selected"]
    D = 3,
}
impl From<DRVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRVSELSELECT_A {
    type Ux = u8;
}
impl DRVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRVSELSELECT_A {
        match self.bits {
            0 => DRVSELSELECT_A::B,
            1 => DRVSELSELECT_A::A,
            2 => DRVSELSELECT_A::C,
            3 => DRVSELSELECT_A::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == DRVSELSELECT_A::B
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == DRVSELSELECT_A::A
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == DRVSELSELECT_A::C
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == DRVSELSELECT_A::D
    }
}
#[doc = "Field `DRVSEL` writer - Driver Strength Select Value for Initialization"]
pub type DRVSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DRVSELSELECT_A>;
impl<'a, REG, const O: u8> DRVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSELSELECT_A::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSELSELECT_A::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSELSELECT_A::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSELSELECT_A::D)
    }
}
impl R {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    pub fn sdclkfsel(&self) -> SDCLKFSEL_R {
        SDCLKFSEL_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    pub fn clkgsel(&self) -> CLKGSEL_R {
        CLKGSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    pub fn drvsel(&self) -> DRVSEL_R {
        DRVSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value for Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfsel(&mut self) -> SDCLKFSEL_W<PVR_SPEC, 0> {
        SDCLKFSEL_W::new(self)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn clkgsel(&mut self) -> CLKGSEL_W<PVR_SPEC, 10> {
        CLKGSEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn drvsel(&mut self) -> DRVSEL_W<PVR_SPEC, 14> {
        DRVSEL_W::new(self)
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
#[doc = "Preset Value n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PVR_SPEC;
impl crate::RegisterSpec for PVR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pvr::R`](R) reader structure"]
impl crate::Readable for PVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pvr::W`](W) writer structure"]
impl crate::Writable for PVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PVR[%s]
to value 0"]
impl crate::Resettable for PVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

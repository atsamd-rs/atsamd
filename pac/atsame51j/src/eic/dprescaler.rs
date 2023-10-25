#[doc = "Register `DPRESCALER` reader"]
pub type R = crate::R<DPRESCALER_SPEC>;
#[doc = "Register `DPRESCALER` writer"]
pub type W = crate::W<DPRESCALER_SPEC>;
#[doc = "Field `PRESCALER0` reader - Debouncer Prescaler"]
pub type PRESCALER0_R = crate::FieldReader<PRESCALER0SELECT_A>;
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER0SELECT_A {
    #[doc = "0: EIC clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER0SELECT_A {
    type Ux = u8;
}
impl PRESCALER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESCALER0SELECT_A {
        match self.bits {
            0 => PRESCALER0SELECT_A::DIV2,
            1 => PRESCALER0SELECT_A::DIV4,
            2 => PRESCALER0SELECT_A::DIV8,
            3 => PRESCALER0SELECT_A::DIV16,
            4 => PRESCALER0SELECT_A::DIV32,
            5 => PRESCALER0SELECT_A::DIV64,
            6 => PRESCALER0SELECT_A::DIV128,
            7 => PRESCALER0SELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV2
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV4
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV8
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV16
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV32
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV64
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV128
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER0SELECT_A::DIV256
    }
}
#[doc = "Field `PRESCALER0` writer - Debouncer Prescaler"]
pub type PRESCALER0_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, PRESCALER0SELECT_A>;
impl<'a, REG, const O: u8> PRESCALER0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER0SELECT_A::DIV256)
    }
}
#[doc = "Field `STATES0` reader - Debouncer number of states"]
pub type STATES0_R = crate::BitReader<STATES0SELECT_A>;
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATES0SELECT_A {
    #[doc = "0: 3 low frequency samples"]
    LFREQ3 = 0,
    #[doc = "1: 7 low frequency samples"]
    LFREQ7 = 1,
}
impl From<STATES0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STATES0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STATES0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STATES0SELECT_A {
        match self.bits {
            false => STATES0SELECT_A::LFREQ3,
            true => STATES0SELECT_A::LFREQ7,
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES0SELECT_A::LFREQ3
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES0SELECT_A::LFREQ7
    }
}
#[doc = "Field `STATES0` writer - Debouncer number of states"]
pub type STATES0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STATES0SELECT_A>;
impl<'a, REG, const O: u8> STATES0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut crate::W<REG> {
        self.variant(STATES0SELECT_A::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut crate::W<REG> {
        self.variant(STATES0SELECT_A::LFREQ7)
    }
}
#[doc = "Field `PRESCALER1` reader - Debouncer Prescaler"]
pub type PRESCALER1_R = crate::FieldReader<PRESCALER1SELECT_A>;
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER1SELECT_A {
    #[doc = "0: EIC clock divided by 2"]
    DIV2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    DIV4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    DIV8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    DIV16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    DIV32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    DIV64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    DIV128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    DIV256 = 7,
}
impl From<PRESCALER1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER1SELECT_A {
    type Ux = u8;
}
impl PRESCALER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESCALER1SELECT_A {
        match self.bits {
            0 => PRESCALER1SELECT_A::DIV2,
            1 => PRESCALER1SELECT_A::DIV4,
            2 => PRESCALER1SELECT_A::DIV8,
            3 => PRESCALER1SELECT_A::DIV16,
            4 => PRESCALER1SELECT_A::DIV32,
            5 => PRESCALER1SELECT_A::DIV64,
            6 => PRESCALER1SELECT_A::DIV128,
            7 => PRESCALER1SELECT_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV2
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV4
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV8
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV16
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV32
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV64
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV128
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER1SELECT_A::DIV256
    }
}
#[doc = "Field `PRESCALER1` writer - Debouncer Prescaler"]
pub type PRESCALER1_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, PRESCALER1SELECT_A>;
impl<'a, REG, const O: u8> PRESCALER1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER1SELECT_A::DIV256)
    }
}
#[doc = "Field `STATES1` reader - Debouncer number of states"]
pub type STATES1_R = crate::BitReader<STATES1SELECT_A>;
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATES1SELECT_A {
    #[doc = "0: 3 low frequency samples"]
    LFREQ3 = 0,
    #[doc = "1: 7 low frequency samples"]
    LFREQ7 = 1,
}
impl From<STATES1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STATES1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STATES1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STATES1SELECT_A {
        match self.bits {
            false => STATES1SELECT_A::LFREQ3,
            true => STATES1SELECT_A::LFREQ7,
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == STATES1SELECT_A::LFREQ3
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == STATES1SELECT_A::LFREQ7
    }
}
#[doc = "Field `STATES1` writer - Debouncer number of states"]
pub type STATES1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STATES1SELECT_A>;
impl<'a, REG, const O: u8> STATES1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut crate::W<REG> {
        self.variant(STATES1SELECT_A::LFREQ3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut crate::W<REG> {
        self.variant(STATES1SELECT_A::LFREQ7)
    }
}
#[doc = "Field `TICKON` reader - Pin Sampler frequency selection"]
pub type TICKON_R = crate::BitReader<TICKONSELECT_A>;
#[doc = "Pin Sampler frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICKONSELECT_A {
    #[doc = "0: Clocked by GCLK"]
    CLK_GCLK_EIC = 0,
    #[doc = "1: Clocked by Low Frequency Clock"]
    CLK_LFREQ = 1,
}
impl From<TICKONSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKONSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TICKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TICKONSELECT_A {
        match self.bits {
            false => TICKONSELECT_A::CLK_GCLK_EIC,
            true => TICKONSELECT_A::CLK_LFREQ,
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn is_clk_gclk_eic(&self) -> bool {
        *self == TICKONSELECT_A::CLK_GCLK_EIC
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline(always)]
    pub fn is_clk_lfreq(&self) -> bool {
        *self == TICKONSELECT_A::CLK_LFREQ
    }
}
#[doc = "Field `TICKON` writer - Pin Sampler frequency selection"]
pub type TICKON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TICKONSELECT_A>;
impl<'a, REG, const O: u8> TICKON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn clk_gclk_eic(self) -> &'a mut crate::W<REG> {
        self.variant(TICKONSELECT_A::CLK_GCLK_EIC)
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline(always)]
    pub fn clk_lfreq(self) -> &'a mut crate::W<REG> {
        self.variant(TICKONSELECT_A::CLK_LFREQ)
    }
}
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> PRESCALER0_R {
        PRESCALER0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> STATES0_R {
        STATES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&self) -> PRESCALER1_R {
        PRESCALER1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&self) -> STATES1_R {
        STATES1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TICKON_R {
        TICKON_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler0(&mut self) -> PRESCALER0_W<DPRESCALER_SPEC, 0> {
        PRESCALER0_W::new(self)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    #[must_use]
    pub fn states0(&mut self) -> STATES0_W<DPRESCALER_SPEC, 3> {
        STATES0_W::new(self)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler1(&mut self) -> PRESCALER1_W<DPRESCALER_SPEC, 4> {
        PRESCALER1_W::new(self)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    #[must_use]
    pub fn states1(&mut self) -> STATES1_W<DPRESCALER_SPEC, 7> {
        STATES1_W::new(self)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn tickon(&mut self) -> TICKON_W<DPRESCALER_SPEC, 16> {
        TICKON_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Debouncer Prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dprescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dprescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPRESCALER_SPEC;
impl crate::RegisterSpec for DPRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dprescaler::R`](R) reader structure"]
impl crate::Readable for DPRESCALER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dprescaler::W`](W) writer structure"]
impl crate::Writable for DPRESCALER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPRESCALER to value 0"]
impl crate::Resettable for DPRESCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

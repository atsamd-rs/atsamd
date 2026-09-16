#[doc = "Register `DPRESCALER` reader"]
pub type R = crate::R<DprescalerSpec>;
#[doc = "Register `DPRESCALER` writer"]
pub type W = crate::W<DprescalerSpec>;
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler0select {
    #[doc = "0: EIC clock divided by 2"]
    Div2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    Div4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    Div8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    Div16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    Div32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    Div64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    Div128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    Div256 = 7,
}
impl From<Prescaler0select> for u8 {
    #[inline(always)]
    fn from(variant: Prescaler0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescaler0select {
    type Ux = u8;
}
impl crate::IsEnum for Prescaler0select {}
#[doc = "Field `PRESCALER0` reader - Debouncer Prescaler"]
pub type Prescaler0R = crate::FieldReader<Prescaler0select>;
impl Prescaler0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescaler0select {
        match self.bits {
            0 => Prescaler0select::Div2,
            1 => Prescaler0select::Div4,
            2 => Prescaler0select::Div8,
            3 => Prescaler0select::Div16,
            4 => Prescaler0select::Div32,
            5 => Prescaler0select::Div64,
            6 => Prescaler0select::Div128,
            7 => Prescaler0select::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescaler0select::Div2
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescaler0select::Div4
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescaler0select::Div8
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescaler0select::Div16
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescaler0select::Div32
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescaler0select::Div64
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescaler0select::Div128
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescaler0select::Div256
    }
}
#[doc = "Field `PRESCALER0` writer - Debouncer Prescaler"]
pub type Prescaler0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescaler0select, crate::Safe>;
impl<'a, REG> Prescaler0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler0select::Div256)
    }
}
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum States0select {
    #[doc = "0: 3 low frequency samples"]
    Lfreq3 = 0,
    #[doc = "1: 7 low frequency samples"]
    Lfreq7 = 1,
}
impl From<States0select> for bool {
    #[inline(always)]
    fn from(variant: States0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATES0` reader - Debouncer number of states"]
pub type States0R = crate::BitReader<States0select>;
impl States0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> States0select {
        match self.bits {
            false => States0select::Lfreq3,
            true => States0select::Lfreq7,
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == States0select::Lfreq3
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == States0select::Lfreq7
    }
}
#[doc = "Field `STATES0` writer - Debouncer number of states"]
pub type States0W<'a, REG> = crate::BitWriter<'a, REG, States0select>;
impl<'a, REG> States0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut crate::W<REG> {
        self.variant(States0select::Lfreq3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut crate::W<REG> {
        self.variant(States0select::Lfreq7)
    }
}
#[doc = "Debouncer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler1select {
    #[doc = "0: EIC clock divided by 2"]
    Div2 = 0,
    #[doc = "1: EIC clock divided by 4"]
    Div4 = 1,
    #[doc = "2: EIC clock divided by 8"]
    Div8 = 2,
    #[doc = "3: EIC clock divided by 16"]
    Div16 = 3,
    #[doc = "4: EIC clock divided by 32"]
    Div32 = 4,
    #[doc = "5: EIC clock divided by 64"]
    Div64 = 5,
    #[doc = "6: EIC clock divided by 128"]
    Div128 = 6,
    #[doc = "7: EIC clock divided by 256"]
    Div256 = 7,
}
impl From<Prescaler1select> for u8 {
    #[inline(always)]
    fn from(variant: Prescaler1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescaler1select {
    type Ux = u8;
}
impl crate::IsEnum for Prescaler1select {}
#[doc = "Field `PRESCALER1` reader - Debouncer Prescaler"]
pub type Prescaler1R = crate::FieldReader<Prescaler1select>;
impl Prescaler1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescaler1select {
        match self.bits {
            0 => Prescaler1select::Div2,
            1 => Prescaler1select::Div4,
            2 => Prescaler1select::Div8,
            3 => Prescaler1select::Div16,
            4 => Prescaler1select::Div32,
            5 => Prescaler1select::Div64,
            6 => Prescaler1select::Div128,
            7 => Prescaler1select::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescaler1select::Div2
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescaler1select::Div4
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescaler1select::Div8
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescaler1select::Div16
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescaler1select::Div32
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescaler1select::Div64
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescaler1select::Div128
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescaler1select::Div256
    }
}
#[doc = "Field `PRESCALER1` writer - Debouncer Prescaler"]
pub type Prescaler1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescaler1select, crate::Safe>;
impl<'a, REG> Prescaler1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EIC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div2)
    }
    #[doc = "EIC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div4)
    }
    #[doc = "EIC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div8)
    }
    #[doc = "EIC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div16)
    }
    #[doc = "EIC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div32)
    }
    #[doc = "EIC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div64)
    }
    #[doc = "EIC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div128)
    }
    #[doc = "EIC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler1select::Div256)
    }
}
#[doc = "Debouncer number of states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum States1select {
    #[doc = "0: 3 low frequency samples"]
    Lfreq3 = 0,
    #[doc = "1: 7 low frequency samples"]
    Lfreq7 = 1,
}
impl From<States1select> for bool {
    #[inline(always)]
    fn from(variant: States1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATES1` reader - Debouncer number of states"]
pub type States1R = crate::BitReader<States1select>;
impl States1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> States1select {
        match self.bits {
            false => States1select::Lfreq3,
            true => States1select::Lfreq7,
        }
    }
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq3(&self) -> bool {
        *self == States1select::Lfreq3
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn is_lfreq7(&self) -> bool {
        *self == States1select::Lfreq7
    }
}
#[doc = "Field `STATES1` writer - Debouncer number of states"]
pub type States1W<'a, REG> = crate::BitWriter<'a, REG, States1select>;
impl<'a, REG> States1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "3 low frequency samples"]
    #[inline(always)]
    pub fn lfreq3(self) -> &'a mut crate::W<REG> {
        self.variant(States1select::Lfreq3)
    }
    #[doc = "7 low frequency samples"]
    #[inline(always)]
    pub fn lfreq7(self) -> &'a mut crate::W<REG> {
        self.variant(States1select::Lfreq7)
    }
}
#[doc = "Pin Sampler frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tickonselect {
    #[doc = "0: Clocked by GCLK"]
    ClkGclkEic = 0,
    #[doc = "1: Clocked by Low Frequency Clock"]
    ClkLfreq = 1,
}
impl From<Tickonselect> for bool {
    #[inline(always)]
    fn from(variant: Tickonselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICKON` reader - Pin Sampler frequency selection"]
pub type TickonR = crate::BitReader<Tickonselect>;
impl TickonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tickonselect {
        match self.bits {
            false => Tickonselect::ClkGclkEic,
            true => Tickonselect::ClkLfreq,
        }
    }
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn is_clk_gclk_eic(&self) -> bool {
        *self == Tickonselect::ClkGclkEic
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline(always)]
    pub fn is_clk_lfreq(&self) -> bool {
        *self == Tickonselect::ClkLfreq
    }
}
#[doc = "Field `TICKON` writer - Pin Sampler frequency selection"]
pub type TickonW<'a, REG> = crate::BitWriter<'a, REG, Tickonselect>;
impl<'a, REG> TickonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clocked by GCLK"]
    #[inline(always)]
    pub fn clk_gclk_eic(self) -> &'a mut crate::W<REG> {
        self.variant(Tickonselect::ClkGclkEic)
    }
    #[doc = "Clocked by Low Frequency Clock"]
    #[inline(always)]
    pub fn clk_lfreq(self) -> &'a mut crate::W<REG> {
        self.variant(Tickonselect::ClkLfreq)
    }
}
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> Prescaler0R {
        Prescaler0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> States0R {
        States0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler1(&self) -> Prescaler1R {
        Prescaler1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    pub fn states1(&self) -> States1R {
        States1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TickonR {
        TickonR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler0(&mut self) -> Prescaler0W<DprescalerSpec> {
        Prescaler0W::new(self, 0)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    #[must_use]
    pub fn states0(&mut self) -> States0W<DprescalerSpec> {
        States0W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Debouncer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler1(&mut self) -> Prescaler1W<DprescalerSpec> {
        Prescaler1W::new(self, 4)
    }
    #[doc = "Bit 7 - Debouncer number of states"]
    #[inline(always)]
    #[must_use]
    pub fn states1(&mut self) -> States1W<DprescalerSpec> {
        States1W::new(self, 7)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn tickon(&mut self) -> TickonW<DprescalerSpec> {
        TickonW::new(self, 16)
    }
}
#[doc = "Debouncer Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dprescaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dprescaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DprescalerSpec;
impl crate::RegisterSpec for DprescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dprescaler::R`](R) reader structure"]
impl crate::Readable for DprescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`dprescaler::W`](W) writer structure"]
impl crate::Writable for DprescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPRESCALER to value 0"]
impl crate::Resettable for DprescalerSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WinctrlSpec>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WinctrlSpec>;
#[doc = "Field `WEN0` reader - Window 0 Mode Enable"]
pub type Wen0R = crate::BitReader;
#[doc = "Field `WEN0` writer - Window 0 Mode Enable"]
pub type Wen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Window 0 Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wintsel0select {
    #[doc = "0: Interrupt on signal above window"]
    Above = 0,
    #[doc = "1: Interrupt on signal inside window"]
    Inside = 1,
    #[doc = "2: Interrupt on signal below window"]
    Below = 2,
    #[doc = "3: Interrupt on signal outside window"]
    Outside = 3,
}
impl From<Wintsel0select> for u8 {
    #[inline(always)]
    fn from(variant: Wintsel0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wintsel0select {
    type Ux = u8;
}
impl crate::IsEnum for Wintsel0select {}
#[doc = "Field `WINTSEL0` reader - Window 0 Interrupt Selection"]
pub type Wintsel0R = crate::FieldReader<Wintsel0select>;
impl Wintsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wintsel0select {
        match self.bits {
            0 => Wintsel0select::Above,
            1 => Wintsel0select::Inside,
            2 => Wintsel0select::Below,
            3 => Wintsel0select::Outside,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Wintsel0select::Above
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Wintsel0select::Inside
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Wintsel0select::Below
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == Wintsel0select::Outside
    }
}
#[doc = "Field `WINTSEL0` writer - Window 0 Interrupt Selection"]
pub type Wintsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wintsel0select, crate::Safe>;
impl<'a, REG> Wintsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(Wintsel0select::Above)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(Wintsel0select::Inside)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(Wintsel0select::Below)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(Wintsel0select::Outside)
    }
}
impl R {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&self) -> Wen0R {
        Wen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&self) -> Wintsel0R {
        Wintsel0R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen0(&mut self) -> Wen0W<WinctrlSpec> {
        Wen0W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wintsel0(&mut self) -> Wintsel0W<WinctrlSpec> {
        Wintsel0W::new(self, 1)
    }
}
#[doc = "Window Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinctrlSpec;
impl crate::RegisterSpec for WinctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`winctrl::R`](R) reader structure"]
impl crate::Readable for WinctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`winctrl::W`](W) writer structure"]
impl crate::Writable for WinctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WinctrlSpec {
    const RESET_VALUE: u8 = 0;
}

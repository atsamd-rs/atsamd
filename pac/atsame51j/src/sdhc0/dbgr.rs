#[doc = "Register `DBGR` reader"]
pub type R = crate::R<DbgrSpec>;
#[doc = "Register `DBGR` writer"]
pub type W = crate::W<DbgrSpec>;
#[doc = "Non-intrusive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nidbgselect {
    #[doc = "0: Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    Idbg = 0,
    #[doc = "1: Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    Nidbg = 1,
}
impl From<Nidbgselect> for bool {
    #[inline(always)]
    fn from(variant: Nidbgselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDBG` reader - Non-intrusive debug enable"]
pub type NidbgR = crate::BitReader<Nidbgselect>;
impl NidbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nidbgselect {
        match self.bits {
            false => Nidbgselect::Idbg,
            true => Nidbgselect::Nidbg,
        }
    }
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn is_idbg(&self) -> bool {
        *self == Nidbgselect::Idbg
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn is_nidbg(&self) -> bool {
        *self == Nidbgselect::Nidbg
    }
}
#[doc = "Field `NIDBG` writer - Non-intrusive debug enable"]
pub type NidbgW<'a, REG> = crate::BitWriter<'a, REG, Nidbgselect>;
impl<'a, REG> NidbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn idbg(self) -> &'a mut crate::W<REG> {
        self.variant(Nidbgselect::Idbg)
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn nidbg(self) -> &'a mut crate::W<REG> {
        self.variant(Nidbgselect::Nidbg)
    }
}
impl R {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&self) -> NidbgR {
        NidbgR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&mut self) -> NidbgW<DbgrSpec> {
        NidbgW::new(self, 0)
    }
}
#[doc = "Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgrSpec;
impl crate::RegisterSpec for DbgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgr::R`](R) reader structure"]
impl crate::Readable for DbgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgr::W`](W) writer structure"]
impl crate::Writable for DbgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGR to value 0"]
impl crate::Resettable for DbgrSpec {}

#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "Field `DBGSTOP` reader - Debug Mode"]
pub type DbgstopR = crate::BitReader;
#[doc = "Field `DBGSTOP` writer - Debug Mode"]
pub type DbgstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Mode"]
    #[inline(always)]
    pub fn dbgstop(&self) -> DbgstopR {
        DbgstopR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop(&mut self) -> DbgstopW<DbgctrlSpec> {
        DbgstopW::new(self, 0)
    }
}
#[doc = "USART_INT Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DbgctrlSpec {
    const RESET_VALUE: u8 = 0;
}

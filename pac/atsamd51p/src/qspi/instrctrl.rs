#[doc = "Register `INSTRCTRL` reader"]
pub type R = crate::R<InstrctrlSpec>;
#[doc = "Register `INSTRCTRL` writer"]
pub type W = crate::W<InstrctrlSpec>;
#[doc = "Field `INSTR` reader - Instruction Code"]
pub type InstrR = crate::FieldReader;
#[doc = "Field `INSTR` writer - Instruction Code"]
pub type InstrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OPTCODE` reader - Option Code"]
pub type OptcodeR = crate::FieldReader;
#[doc = "Field `OPTCODE` writer - Option Code"]
pub type OptcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn instr(&self) -> InstrR {
        InstrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn optcode(&self) -> OptcodeR {
        OptcodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn instr(&mut self) -> InstrW<InstrctrlSpec> {
        InstrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn optcode(&mut self) -> OptcodeW<InstrctrlSpec> {
        OptcodeW::new(self, 16)
    }
}
#[doc = "Instruction Code\n\nYou can [`read`](crate::Reg::read) this register and get [`instrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrctrlSpec;
impl crate::RegisterSpec for InstrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instrctrl::R`](R) reader structure"]
impl crate::Readable for InstrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`instrctrl::W`](W) writer structure"]
impl crate::Writable for InstrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSTRCTRL to value 0"]
impl crate::Resettable for InstrctrlSpec {}

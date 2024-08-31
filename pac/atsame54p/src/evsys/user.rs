#[doc = "Register `USER[%s]` reader"]
pub type R = crate::R<UserSpec>;
#[doc = "Register `USER[%s]` writer"]
pub type W = crate::W<UserSpec>;
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub type ChannelR = crate::FieldReader;
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> ChannelW<UserSpec> {
        ChannelW::new(self, 0)
    }
}
#[doc = "User Multiplexer n\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserSpec;
impl crate::RegisterSpec for UserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for UserSpec {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for UserSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER[%s]
to value 0"]
impl crate::Resettable for UserSpec {
    const RESET_VALUE: u32 = 0;
}

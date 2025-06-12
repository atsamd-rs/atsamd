#[doc = "Register `USER` reader"]
pub type R = crate::R<UserSpec>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<UserSpec>;
#[doc = "Field `USER` reader - User Multiplexer Selection"]
pub type UserR = crate::FieldReader;
#[doc = "Field `USER` writer - User Multiplexer Selection"]
pub type UserW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Channel Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Channelselect {
    #[doc = "0: No Channel Output Selected"]
    _0 = 0,
}
impl From<Channelselect> for u8 {
    #[inline(always)]
    fn from(variant: Channelselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Channelselect {
    type Ux = u8;
}
impl crate::IsEnum for Channelselect {}
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub type ChannelR = crate::FieldReader<Channelselect>;
impl ChannelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Channelselect> {
        match self.bits {
            0 => Some(Channelselect::_0),
            _ => None,
        }
    }
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Channelselect::_0
    }
}
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Channelselect>;
impl<'a, REG> ChannelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Channelselect::_0)
    }
}
impl R {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&mut self) -> UserW<UserSpec> {
        UserW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&mut self) -> ChannelW<UserSpec> {
        ChannelW::new(self, 8)
    }
}
#[doc = "User Multiplexer\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserSpec;
impl crate::RegisterSpec for UserSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for UserSpec {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for UserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for UserSpec {}

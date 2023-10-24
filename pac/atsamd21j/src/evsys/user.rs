#[doc = "Register `USER` reader"]
pub type R = crate::R<USER_SPEC>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<USER_SPEC>;
#[doc = "Field `USER` reader - User Multiplexer Selection"]
pub type USER_R = crate::FieldReader;
#[doc = "Field `USER` writer - User Multiplexer Selection"]
pub type USER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CHANNEL` reader - Channel Event Selection"]
pub type CHANNEL_R = crate::FieldReader<CHANNELSELECT_A>;
#[doc = "Channel Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNELSELECT_A {
    #[doc = "0: No Channel Output Selected"]
    _0 = 0,
}
impl From<CHANNELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHANNELSELECT_A {
    type Ux = u8;
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHANNELSELECT_A> {
        match self.bits {
            0 => Some(CHANNELSELECT_A::_0),
            _ => None,
        }
    }
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHANNELSELECT_A::_0
    }
}
#[doc = "Field `CHANNEL` writer - Channel Event Selection"]
pub type CHANNEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, CHANNELSELECT_A>;
impl<'a, REG, const O: u8> CHANNEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Channel Output Selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNELSELECT_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - User Multiplexer Selection"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<USER_SPEC, 0> {
        USER_W::new(self)
    }
    #[doc = "Bits 8:12 - Channel Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<USER_SPEC, 8> {
        CHANNEL_W::new(self)
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
#[doc = "User Multiplexer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for USER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

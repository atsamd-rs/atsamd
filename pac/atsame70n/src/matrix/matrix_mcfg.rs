#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MATRIX_MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Undefined Length Burst Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ULBT_A {
    #[doc = "0: Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    UNLTD_LENGTH = 0,
    #[doc = "1: Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    SINGLE_ACCESS = 1,
    #[doc = "2: 4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    _4BEAT_BURST = 2,
    #[doc = "3: 8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    _8BEAT_BURST = 3,
    #[doc = "4: 16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    _16BEAT_BURST = 4,
    #[doc = "5: 32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    _32BEAT_BURST = 5,
    #[doc = "6: 64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    _64BEAT_BURST = 6,
    #[doc = "7: 128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    _128BEAT_BURST = 7,
}
impl From<ULBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub struct ULBT_R(crate::FieldReader<u8, ULBT_A>);
impl ULBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ULBT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULBT_A {
        match self.bits {
            0 => ULBT_A::UNLTD_LENGTH,
            1 => ULBT_A::SINGLE_ACCESS,
            2 => ULBT_A::_4BEAT_BURST,
            3 => ULBT_A::_8BEAT_BURST,
            4 => ULBT_A::_16BEAT_BURST,
            5 => ULBT_A::_32BEAT_BURST,
            6 => ULBT_A::_64BEAT_BURST,
            7 => ULBT_A::_128BEAT_BURST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNLTD_LENGTH`"]
    #[inline(always)]
    pub fn is_unltd_length(&self) -> bool {
        **self == ULBT_A::UNLTD_LENGTH
    }
    #[doc = "Checks if the value of the field is `SINGLE_ACCESS`"]
    #[inline(always)]
    pub fn is_single_access(&self) -> bool {
        **self == ULBT_A::SINGLE_ACCESS
    }
    #[doc = "Checks if the value of the field is `_4BEAT_BURST`"]
    #[inline(always)]
    pub fn is_4beat_burst(&self) -> bool {
        **self == ULBT_A::_4BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_8BEAT_BURST`"]
    #[inline(always)]
    pub fn is_8beat_burst(&self) -> bool {
        **self == ULBT_A::_8BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_16BEAT_BURST`"]
    #[inline(always)]
    pub fn is_16beat_burst(&self) -> bool {
        **self == ULBT_A::_16BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_32BEAT_BURST`"]
    #[inline(always)]
    pub fn is_32beat_burst(&self) -> bool {
        **self == ULBT_A::_32BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_64BEAT_BURST`"]
    #[inline(always)]
    pub fn is_64beat_burst(&self) -> bool {
        **self == ULBT_A::_64BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_128BEAT_BURST`"]
    #[inline(always)]
    pub fn is_128beat_burst(&self) -> bool {
        **self == ULBT_A::_128BEAT_BURST
    }
}
impl core::ops::Deref for ULBT_R {
    type Target = crate::FieldReader<u8, ULBT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULBT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn unltd_length(self) -> &'a mut W {
        self.variant(ULBT_A::UNLTD_LENGTH)
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn single_access(self) -> &'a mut W {
        self.variant(ULBT_A::SINGLE_ACCESS)
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn _4beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_4BEAT_BURST)
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn _8beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_8BEAT_BURST)
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn _16beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_16BEAT_BURST)
    }
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn _32beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_32BEAT_BURST)
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn _64beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_64BEAT_BURST)
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    #[inline(always)]
    pub fn _128beat_burst(self) -> &'a mut W {
        self.variant(ULBT_A::_128BEAT_BURST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](index.html) module"]
pub struct MATRIX_MCFG_SPEC;
impl crate::RegisterSpec for MATRIX_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mcfg::R](R) reader structure"]
impl crate::Readable for MATRIX_MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](W) writer structure"]
impl crate::Writable for MATRIX_MCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_MCFG[%s]
to value 0"]
impl crate::Resettable for MATRIX_MCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

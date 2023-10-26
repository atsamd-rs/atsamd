#[doc = "Register `PRICTRL0` reader"]
pub type R = crate::R<PRICTRL0_SPEC>;
#[doc = "Register `PRICTRL0` writer"]
pub type W = crate::W<PRICTRL0_SPEC>;
#[doc = "Field `LVLPRI0` reader - Level 0 Channel Priority Number"]
pub type LVLPRI0_R = crate::FieldReader;
#[doc = "Field `LVLPRI0` writer - Level 0 Channel Priority Number"]
pub type LVLPRI0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `QOS0` reader - Level 0 Quality of Service"]
pub type QOS0_R = crate::FieldReader<QOS0SELECT_A>;
#[doc = "Level 0 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOS0SELECT_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QOS0SELECT_A {
    type Ux = u8;
}
impl QOS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QOS0SELECT_A {
        match self.bits {
            0 => QOS0SELECT_A::REGULAR,
            1 => QOS0SELECT_A::SHORTAGE,
            2 => QOS0SELECT_A::SENSITIVE,
            3 => QOS0SELECT_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == QOS0SELECT_A::REGULAR
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS0SELECT_A::SHORTAGE
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS0SELECT_A::SENSITIVE
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS0SELECT_A::CRITICAL
    }
}
#[doc = "Field `QOS0` writer - Level 0 Quality of Service"]
pub type QOS0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, QOS0SELECT_A>;
impl<'a, REG, const O: u8> QOS0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(QOS0SELECT_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(QOS0SELECT_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(QOS0SELECT_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(QOS0SELECT_A::CRITICAL)
    }
}
#[doc = "Field `RRLVLEN0` reader - Level 0 Round-Robin Scheduling Enable"]
pub type RRLVLEN0_R = crate::BitReader;
#[doc = "Field `RRLVLEN0` writer - Level 0 Round-Robin Scheduling Enable"]
pub type RRLVLEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVLPRI1` reader - Level 1 Channel Priority Number"]
pub type LVLPRI1_R = crate::FieldReader;
#[doc = "Field `LVLPRI1` writer - Level 1 Channel Priority Number"]
pub type LVLPRI1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `QOS1` reader - Level 1 Quality of Service"]
pub type QOS1_R = crate::FieldReader<QOS1SELECT_A>;
#[doc = "Level 1 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOS1SELECT_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QOS1SELECT_A {
    type Ux = u8;
}
impl QOS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QOS1SELECT_A {
        match self.bits {
            0 => QOS1SELECT_A::REGULAR,
            1 => QOS1SELECT_A::SHORTAGE,
            2 => QOS1SELECT_A::SENSITIVE,
            3 => QOS1SELECT_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == QOS1SELECT_A::REGULAR
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS1SELECT_A::SHORTAGE
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS1SELECT_A::SENSITIVE
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS1SELECT_A::CRITICAL
    }
}
#[doc = "Field `QOS1` writer - Level 1 Quality of Service"]
pub type QOS1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, QOS1SELECT_A>;
impl<'a, REG, const O: u8> QOS1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(QOS1SELECT_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(QOS1SELECT_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(QOS1SELECT_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(QOS1SELECT_A::CRITICAL)
    }
}
#[doc = "Field `RRLVLEN1` reader - Level 1 Round-Robin Scheduling Enable"]
pub type RRLVLEN1_R = crate::BitReader;
#[doc = "Field `RRLVLEN1` writer - Level 1 Round-Robin Scheduling Enable"]
pub type RRLVLEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVLPRI2` reader - Level 2 Channel Priority Number"]
pub type LVLPRI2_R = crate::FieldReader;
#[doc = "Field `LVLPRI2` writer - Level 2 Channel Priority Number"]
pub type LVLPRI2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `QOS2` reader - Level 2 Quality of Service"]
pub type QOS2_R = crate::FieldReader<QOS2SELECT_A>;
#[doc = "Level 2 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOS2SELECT_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS2SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS2SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QOS2SELECT_A {
    type Ux = u8;
}
impl QOS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QOS2SELECT_A {
        match self.bits {
            0 => QOS2SELECT_A::REGULAR,
            1 => QOS2SELECT_A::SHORTAGE,
            2 => QOS2SELECT_A::SENSITIVE,
            3 => QOS2SELECT_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == QOS2SELECT_A::REGULAR
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS2SELECT_A::SHORTAGE
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS2SELECT_A::SENSITIVE
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS2SELECT_A::CRITICAL
    }
}
#[doc = "Field `QOS2` writer - Level 2 Quality of Service"]
pub type QOS2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, QOS2SELECT_A>;
impl<'a, REG, const O: u8> QOS2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(QOS2SELECT_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(QOS2SELECT_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(QOS2SELECT_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(QOS2SELECT_A::CRITICAL)
    }
}
#[doc = "Field `RRLVLEN2` reader - Level 2 Round-Robin Scheduling Enable"]
pub type RRLVLEN2_R = crate::BitReader;
#[doc = "Field `RRLVLEN2` writer - Level 2 Round-Robin Scheduling Enable"]
pub type RRLVLEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LVLPRI3` reader - Level 3 Channel Priority Number"]
pub type LVLPRI3_R = crate::FieldReader;
#[doc = "Field `LVLPRI3` writer - Level 3 Channel Priority Number"]
pub type LVLPRI3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `QOS3` reader - Level 3 Quality of Service"]
pub type QOS3_R = crate::FieldReader<QOS3SELECT_A>;
#[doc = "Level 3 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QOS3SELECT_A {
    #[doc = "0: Regular delivery"]
    REGULAR = 0,
    #[doc = "1: Bandwidth shortage"]
    SHORTAGE = 1,
    #[doc = "2: Latency sensitive"]
    SENSITIVE = 2,
    #[doc = "3: Latency critical"]
    CRITICAL = 3,
}
impl From<QOS3SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: QOS3SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QOS3SELECT_A {
    type Ux = u8;
}
impl QOS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QOS3SELECT_A {
        match self.bits {
            0 => QOS3SELECT_A::REGULAR,
            1 => QOS3SELECT_A::SHORTAGE,
            2 => QOS3SELECT_A::SENSITIVE,
            3 => QOS3SELECT_A::CRITICAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == QOS3SELECT_A::REGULAR
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == QOS3SELECT_A::SHORTAGE
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == QOS3SELECT_A::SENSITIVE
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == QOS3SELECT_A::CRITICAL
    }
}
#[doc = "Field `QOS3` writer - Level 3 Quality of Service"]
pub type QOS3_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, QOS3SELECT_A>;
impl<'a, REG, const O: u8> QOS3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(QOS3SELECT_A::REGULAR)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(QOS3SELECT_A::SHORTAGE)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(QOS3SELECT_A::SENSITIVE)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(QOS3SELECT_A::CRITICAL)
    }
}
#[doc = "Field `RRLVLEN3` reader - Level 3 Round-Robin Scheduling Enable"]
pub type RRLVLEN3_R = crate::BitReader;
#[doc = "Field `RRLVLEN3` writer - Level 3 Round-Robin Scheduling Enable"]
pub type RRLVLEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&self) -> LVLPRI0_R {
        LVLPRI0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    pub fn qos0(&self) -> QOS0_R {
        QOS0_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&self) -> RRLVLEN0_R {
        RRLVLEN0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&self) -> LVLPRI1_R {
        LVLPRI1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    pub fn qos1(&self) -> QOS1_R {
        QOS1_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&self) -> RRLVLEN1_R {
        RRLVLEN1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&self) -> LVLPRI2_R {
        LVLPRI2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    pub fn qos2(&self) -> QOS2_R {
        QOS2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&self) -> RRLVLEN2_R {
        RRLVLEN2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&self) -> LVLPRI3_R {
        LVLPRI3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    pub fn qos3(&self) -> QOS3_R {
        QOS3_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&self) -> RRLVLEN3_R {
        RRLVLEN3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri0(&mut self) -> LVLPRI0_W<PRICTRL0_SPEC, 0> {
        LVLPRI0_W::new(self)
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos0(&mut self) -> QOS0_W<PRICTRL0_SPEC, 5> {
        QOS0_W::new(self)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen0(&mut self) -> RRLVLEN0_W<PRICTRL0_SPEC, 7> {
        RRLVLEN0_W::new(self)
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri1(&mut self) -> LVLPRI1_W<PRICTRL0_SPEC, 8> {
        LVLPRI1_W::new(self)
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos1(&mut self) -> QOS1_W<PRICTRL0_SPEC, 13> {
        QOS1_W::new(self)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen1(&mut self) -> RRLVLEN1_W<PRICTRL0_SPEC, 15> {
        RRLVLEN1_W::new(self)
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri2(&mut self) -> LVLPRI2_W<PRICTRL0_SPEC, 16> {
        LVLPRI2_W::new(self)
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos2(&mut self) -> QOS2_W<PRICTRL0_SPEC, 21> {
        QOS2_W::new(self)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen2(&mut self) -> RRLVLEN2_W<PRICTRL0_SPEC, 23> {
        RRLVLEN2_W::new(self)
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri3(&mut self) -> LVLPRI3_W<PRICTRL0_SPEC, 24> {
        LVLPRI3_W::new(self)
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    #[must_use]
    pub fn qos3(&mut self) -> QOS3_W<PRICTRL0_SPEC, 29> {
        QOS3_W::new(self)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen3(&mut self) -> RRLVLEN3_W<PRICTRL0_SPEC, 31> {
        RRLVLEN3_W::new(self)
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
#[doc = "Priority Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prictrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prictrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRICTRL0_SPEC;
impl crate::RegisterSpec for PRICTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prictrl0::R`](R) reader structure"]
impl crate::Readable for PRICTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prictrl0::W`](W) writer structure"]
impl crate::Writable for PRICTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRICTRL0 to value 0x4040_4040"]
impl crate::Resettable for PRICTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4040_4040;
}

#[doc = "Register `NISTR` reader"]
pub type R = crate::R<NISTR_SPEC>;
#[doc = "Register `NISTR` writer"]
pub type W = crate::W<NISTR_SPEC>;
#[doc = "Field `CMDC` reader - Command Complete"]
pub type CMDC_R = crate::BitReader<CMDCSELECT_A>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCSELECT_A {
    #[doc = "0: No command complete"]
    NO = 0,
    #[doc = "1: Command complete"]
    YES = 1,
}
impl From<CMDCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMDCSELECT_A {
        match self.bits {
            false => CMDCSELECT_A::NO,
            true => CMDCSELECT_A::YES,
        }
    }
    #[doc = "No command complete"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CMDCSELECT_A::NO
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CMDCSELECT_A::YES
    }
}
#[doc = "Field `CMDC` writer - Command Complete"]
pub type CMDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDCSELECT_A>;
impl<'a, REG, const O: u8> CMDC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No command complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCSELECT_A::NO)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCSELECT_A::YES)
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete"]
pub type TRFC_R = crate::BitReader<TRFCSELECT_A>;
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRFCSELECT_A {
    #[doc = "0: Not complete"]
    NO = 0,
    #[doc = "1: Command execution is completed"]
    YES = 1,
}
impl From<TRFCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRFCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRFCSELECT_A {
        match self.bits {
            false => TRFCSELECT_A::NO,
            true => TRFCSELECT_A::YES,
        }
    }
    #[doc = "Not complete"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TRFCSELECT_A::NO
    }
    #[doc = "Command execution is completed"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TRFCSELECT_A::YES
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete"]
pub type TRFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRFCSELECT_A>;
impl<'a, REG, const O: u8> TRFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(TRFCSELECT_A::NO)
    }
    #[doc = "Command execution is completed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(TRFCSELECT_A::YES)
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event"]
pub type BLKGE_R = crate::BitReader<BLKGESELECT_A>;
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLKGESELECT_A {
    #[doc = "0: No Block Gap Event"]
    NO = 0,
    #[doc = "1: Transaction stopped at block gap"]
    STOP = 1,
}
impl From<BLKGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BLKGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BLKGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLKGESELECT_A {
        match self.bits {
            false => BLKGESELECT_A::NO,
            true => BLKGESELECT_A::STOP,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BLKGESELECT_A::NO
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == BLKGESELECT_A::STOP
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event"]
pub type BLKGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BLKGESELECT_A>;
impl<'a, REG, const O: u8> BLKGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(BLKGESELECT_A::NO)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(BLKGESELECT_A::STOP)
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt"]
pub type DMAINT_R = crate::BitReader<DMAINTSELECT_A>;
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINTSELECT_A {
    #[doc = "0: No DMA Interrupt"]
    NO = 0,
    #[doc = "1: DMA Interrupt is generated"]
    YES = 1,
}
impl From<DMAINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAINTSELECT_A {
        match self.bits {
            false => DMAINTSELECT_A::NO,
            true => DMAINTSELECT_A::YES,
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == DMAINTSELECT_A::NO
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == DMAINTSELECT_A::YES
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt"]
pub type DMAINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAINTSELECT_A>;
impl<'a, REG, const O: u8> DMAINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINTSELECT_A::NO)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINTSELECT_A::YES)
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready"]
pub type BWRRDY_R = crate::BitReader<BWRRDYSELECT_A>;
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRRDYSELECT_A {
    #[doc = "0: Not ready to write buffer"]
    NO = 0,
    #[doc = "1: Ready to write buffer"]
    YES = 1,
}
impl From<BWRRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BWRRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWRRDYSELECT_A {
        match self.bits {
            false => BWRRDYSELECT_A::NO,
            true => BWRRDYSELECT_A::YES,
        }
    }
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BWRRDYSELECT_A::NO
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BWRRDYSELECT_A::YES
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready"]
pub type BWRRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BWRRDYSELECT_A>;
impl<'a, REG, const O: u8> BWRRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(BWRRDYSELECT_A::NO)
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(BWRRDYSELECT_A::YES)
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready"]
pub type BRDRDY_R = crate::BitReader<BRDRDYSELECT_A>;
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDRDYSELECT_A {
    #[doc = "0: Not ready to read buffer"]
    NO = 0,
    #[doc = "1: Ready to read buffer"]
    YES = 1,
}
impl From<BRDRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BRDRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRDRDYSELECT_A {
        match self.bits {
            false => BRDRDYSELECT_A::NO,
            true => BRDRDYSELECT_A::YES,
        }
    }
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BRDRDYSELECT_A::NO
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == BRDRDYSELECT_A::YES
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready"]
pub type BRDRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRDRDYSELECT_A>;
impl<'a, REG, const O: u8> BRDRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(BRDRDYSELECT_A::NO)
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(BRDRDYSELECT_A::YES)
    }
}
#[doc = "Field `CINS` reader - Card Insertion"]
pub type CINS_R = crate::BitReader<CINSSELECT_A>;
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSSELECT_A {
    #[doc = "0: Card state stable or Debouncing"]
    NO = 0,
    #[doc = "1: Card inserted"]
    YES = 1,
}
impl From<CINSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CINSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CINSSELECT_A {
        match self.bits {
            false => CINSSELECT_A::NO,
            true => CINSSELECT_A::YES,
        }
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CINSSELECT_A::NO
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CINSSELECT_A::YES
    }
}
#[doc = "Field `CINS` writer - Card Insertion"]
pub type CINS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CINSSELECT_A>;
impl<'a, REG, const O: u8> CINS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CINSSELECT_A::NO)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CINSSELECT_A::YES)
    }
}
#[doc = "Field `CREM` reader - Card Removal"]
pub type CREM_R = crate::BitReader<CREMSELECT_A>;
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREMSELECT_A {
    #[doc = "0: Card state stable or Debouncing"]
    NO = 0,
    #[doc = "1: Card Removed"]
    YES = 1,
}
impl From<CREMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CREMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CREM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CREMSELECT_A {
        match self.bits {
            false => CREMSELECT_A::NO,
            true => CREMSELECT_A::YES,
        }
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CREMSELECT_A::NO
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CREMSELECT_A::YES
    }
}
#[doc = "Field `CREM` writer - Card Removal"]
pub type CREM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CREMSELECT_A>;
impl<'a, REG, const O: u8> CREM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CREMSELECT_A::NO)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CREMSELECT_A::YES)
    }
}
#[doc = "Field `CINT` reader - Card Interrupt"]
pub type CINT_R = crate::BitReader<CINTSELECT_A>;
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTSELECT_A {
    #[doc = "0: No Card Interrupt"]
    NO = 0,
    #[doc = "1: Generate Card Interrupt"]
    YES = 1,
}
impl From<CINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CINTSELECT_A {
        match self.bits {
            false => CINTSELECT_A::NO,
            true => CINTSELECT_A::YES,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CINTSELECT_A::NO
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CINTSELECT_A::YES
    }
}
#[doc = "Field `CINT` writer - Card Interrupt"]
pub type CINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CINTSELECT_A>;
impl<'a, REG, const O: u8> CINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CINTSELECT_A::NO)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CINTSELECT_A::YES)
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ERRINT_R = crate::BitReader<ERRINTSELECT_A>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRINTSELECT_A {
    #[doc = "0: No Error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<ERRINTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRINTSELECT_A {
        match self.bits {
            false => ERRINTSELECT_A::NO,
            true => ERRINTSELECT_A::YES,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ERRINTSELECT_A::NO
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ERRINTSELECT_A::YES
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub type ERRINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRINTSELECT_A>;
impl<'a, REG, const O: u8> ERRINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(ERRINTSELECT_A::NO)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(ERRINTSELECT_A::YES)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdc(&self) -> CMDC_R {
        CMDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkge(&self) -> BLKGE_R {
        BLKGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BWRRDY_R {
        BWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BRDRDY_R {
        BRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crem(&self) -> CREM_R {
        CREM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmdc(&mut self) -> CMDC_W<NISTR_SPEC, 0> {
        CMDC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<NISTR_SPEC, 1> {
        TRFC_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BLKGE_W<NISTR_SPEC, 2> {
        BLKGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DMAINT_W<NISTR_SPEC, 3> {
        DMAINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BWRRDY_W<NISTR_SPEC, 4> {
        BWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BRDRDY_W<NISTR_SPEC, 5> {
        BRDRDY_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CINS_W<NISTR_SPEC, 6> {
        CINS_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn crem(&mut self) -> CREM_W<NISTR_SPEC, 7> {
        CREM_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<NISTR_SPEC, 8> {
        CINT_W::new(self)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn errint(&mut self) -> ERRINT_W<NISTR_SPEC, 15> {
        ERRINT_W::new(self)
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
#[doc = "Normal Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nistr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nistr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NISTR_SPEC;
impl crate::RegisterSpec for NISTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nistr::R`](R) reader structure"]
impl crate::Readable for NISTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nistr::W`](W) writer structure"]
impl crate::Writable for NISTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NISTR to value 0"]
impl crate::Resettable for NISTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `NISTER_EMMC_MODE` reader"]
pub type R = crate::R<NISTER_EMMC_MODE_SPEC>;
#[doc = "Register `NISTER_EMMC_MODE` writer"]
pub type W = crate::W<NISTER_EMMC_MODE_SPEC>;
#[doc = "Field `CMDC` reader - Command Complete Status Enable"]
pub type CMDC_R = crate::BitReader<CMDCSELECT_A>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => CMDCSELECT_A::MASKED,
            true => CMDCSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CMDCSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMDCSELECT_A::ENABLED
    }
}
#[doc = "Field `CMDC` writer - Command Complete Status Enable"]
pub type CMDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDCSELECT_A>;
impl<'a, REG, const O: u8> CMDC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCSELECT_A::ENABLED)
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete Status Enable"]
pub type TRFC_R = crate::BitReader<TRFCSELECT_A>;
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRFCSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => TRFCSELECT_A::MASKED,
            true => TRFCSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TRFCSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRFCSELECT_A::ENABLED
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete Status Enable"]
pub type TRFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRFCSELECT_A>;
impl<'a, REG, const O: u8> TRFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TRFCSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRFCSELECT_A::ENABLED)
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event Status Enable"]
pub type BLKGE_R = crate::BitReader<BLKGESELECT_A>;
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLKGESELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => BLKGESELECT_A::MASKED,
            true => BLKGESELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BLKGESELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLKGESELECT_A::ENABLED
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event Status Enable"]
pub type BLKGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BLKGESELECT_A>;
impl<'a, REG, const O: u8> BLKGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(BLKGESELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLKGESELECT_A::ENABLED)
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt Status Enable"]
pub type DMAINT_R = crate::BitReader<DMAINTSELECT_A>;
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAINTSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => DMAINTSELECT_A::MASKED,
            true => DMAINTSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DMAINTSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAINTSELECT_A::ENABLED
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt Status Enable"]
pub type DMAINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMAINTSELECT_A>;
impl<'a, REG, const O: u8> DMAINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINTSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAINTSELECT_A::ENABLED)
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready Status Enable"]
pub type BWRRDY_R = crate::BitReader<BWRRDYSELECT_A>;
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRRDYSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => BWRRDYSELECT_A::MASKED,
            true => BWRRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BWRRDYSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BWRRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready Status Enable"]
pub type BWRRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BWRRDYSELECT_A>;
impl<'a, REG, const O: u8> BWRRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(BWRRDYSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BWRRDYSELECT_A::ENABLED)
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready Status Enable"]
pub type BRDRDY_R = crate::BitReader<BRDRDYSELECT_A>;
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDRDYSELECT_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => BRDRDYSELECT_A::MASKED,
            true => BRDRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == BRDRDYSELECT_A::MASKED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRDRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready Status Enable"]
pub type BRDRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRDRDYSELECT_A>;
impl<'a, REG, const O: u8> BRDRDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(BRDRDYSELECT_A::MASKED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRDRDYSELECT_A::ENABLED)
    }
}
#[doc = "Field `BOOTAR` reader - Boot Acknowledge Received Status Enable"]
pub type BOOTAR_R = crate::BitReader;
#[doc = "Field `BOOTAR` writer - Boot Acknowledge Received Status Enable"]
pub type BOOTAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn cmdc(&self) -> CMDC_R {
        CMDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn blkge(&self) -> BLKGE_R {
        BLKGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BWRRDY_R {
        BWRRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BRDRDY_R {
        BRDRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Status Enable"]
    #[inline(always)]
    pub fn bootar(&self) -> BOOTAR_R {
        BOOTAR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdc(&mut self) -> CMDC_W<NISTER_EMMC_MODE_SPEC, 0> {
        CMDC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trfc(&mut self) -> TRFC_W<NISTER_EMMC_MODE_SPEC, 1> {
        TRFC_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkge(&mut self) -> BLKGE_W<NISTER_EMMC_MODE_SPEC, 2> {
        BLKGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaint(&mut self) -> DMAINT_W<NISTER_EMMC_MODE_SPEC, 3> {
        DMAINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrrdy(&mut self) -> BWRRDY_W<NISTER_EMMC_MODE_SPEC, 4> {
        BWRRDY_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brdrdy(&mut self) -> BRDRDY_W<NISTER_EMMC_MODE_SPEC, 5> {
        BRDRDY_W::new(self)
    }
    #[doc = "Bit 14 - Boot Acknowledge Received Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bootar(&mut self) -> BOOTAR_W<NISTER_EMMC_MODE_SPEC, 14> {
        BOOTAR_W::new(self)
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
#[doc = "Normal Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nister_emmc_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nister_emmc_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NISTER_EMMC_MODE_SPEC;
impl crate::RegisterSpec for NISTER_EMMC_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nister_emmc_mode::R`](R) reader structure"]
impl crate::Readable for NISTER_EMMC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nister_emmc_mode::W`](W) writer structure"]
impl crate::Writable for NISTER_EMMC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NISTER_EMMC_MODE to value 0"]
impl crate::Resettable for NISTER_EMMC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

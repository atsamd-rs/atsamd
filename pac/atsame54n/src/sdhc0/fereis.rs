#[doc = "Register `FEREIS` writer"]
pub type W = crate::W<FEREIS_SPEC>;
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDTEOSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDTEOSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDTEOSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTEO` writer - Force Event for Command Timeout Error"]
pub type CMDTEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDTEOSELECT_AW>;
impl<'a, REG, const O: u8> CMDTEO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTEOSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTEOSELECT_AW::YES)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDCRCSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDCRCSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDCRCSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRC` writer - Force Event for Command CRC Error"]
pub type CMDCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDCRCSELECT_AW>;
impl<'a, REG, const O: u8> CMDCRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCRCSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CMDCRCSELECT_AW::YES)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDENDSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDENDSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDENDSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDEND` writer - Force Event for Command End Bit Error"]
pub type CMDEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDENDSELECT_AW>;
impl<'a, REG, const O: u8> CMDEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CMDENDSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CMDENDSELECT_AW::YES)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDIDXSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDIDXSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDIDXSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDIDX` writer - Force Event for Command Index Error"]
pub type CMDIDX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMDIDXSELECT_AW>;
impl<'a, REG, const O: u8> CMDIDX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CMDIDXSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CMDIDXSELECT_AW::YES)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATTEOSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATTEOSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DATTEOSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTEO` writer - Force Event for Data Timeout Error"]
pub type DATTEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATTEOSELECT_AW>;
impl<'a, REG, const O: u8> DATTEO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DATTEOSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DATTEOSELECT_AW::YES)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATCRCSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATCRCSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DATCRCSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRC` writer - Force Event for Data CRC Error"]
pub type DATCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATCRCSELECT_AW>;
impl<'a, REG, const O: u8> DATCRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DATCRCSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DATCRCSELECT_AW::YES)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATENDSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATENDSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: DATENDSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATEND` writer - Force Event for Data End Bit Error"]
pub type DATEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DATENDSELECT_AW>;
impl<'a, REG, const O: u8> DATEND_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(DATENDSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(DATENDSELECT_AW::YES)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURLIMSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CURLIMSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CURLIMSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURLIM` writer - Force Event for Current Limit Error"]
pub type CURLIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CURLIMSELECT_AW>;
impl<'a, REG, const O: u8> CURLIM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(CURLIMSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(CURLIMSELECT_AW::YES)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDSELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMDSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMDSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD` writer - Force Event for Auto CMD Error"]
pub type ACMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ACMDSELECT_AW>;
impl<'a, REG, const O: u8> ACMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDSELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(ACMDSELECT_AW::YES)
    }
}
#[doc = "Force Event for ADMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMASELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ADMASELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ADMASELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA` writer - Force Event for ADMA Error"]
pub type ADMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADMASELECT_AW>;
impl<'a, REG, const O: u8> ADMA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(ADMASELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(ADMASELECT_AW::YES)
    }
}
#[doc = "Force Event for Boot Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTAESELECT_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<BOOTAESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: BOOTAESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTAE` writer - Force Event for Boot Acknowledge Error"]
pub type BOOTAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BOOTAESELECT_AW>;
impl<'a, REG, const O: u8> BOOTAE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTAESELECT_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTAESELECT_AW::YES)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdteo(&mut self) -> CMDTEO_W<FEREIS_SPEC, 0> {
        CMDTEO_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrc(&mut self) -> CMDCRC_W<FEREIS_SPEC, 1> {
        CMDCRC_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdend(&mut self) -> CMDEND_W<FEREIS_SPEC, 2> {
        CMDEND_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<FEREIS_SPEC, 3> {
        CMDIDX_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn datteo(&mut self) -> DATTEO_W<FEREIS_SPEC, 4> {
        DATTEO_W::new(self)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn datcrc(&mut self) -> DATCRC_W<FEREIS_SPEC, 5> {
        DATCRC_W::new(self)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn datend(&mut self) -> DATEND_W<FEREIS_SPEC, 6> {
        DATEND_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn curlim(&mut self) -> CURLIM_W<FEREIS_SPEC, 7> {
        CURLIM_W::new(self)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmd(&mut self) -> ACMD_W<FEREIS_SPEC, 8> {
        ACMD_W::new(self)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> ADMA_W<FEREIS_SPEC, 9> {
        ADMA_W::new(self)
    }
    #[doc = "Bit 12 - Force Event for Boot Acknowledge Error"]
    #[inline(always)]
    #[must_use]
    pub fn bootae(&mut self) -> BOOTAE_W<FEREIS_SPEC, 12> {
        BOOTAE_W::new(self)
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
#[doc = "Force Event for Error Interrupt Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fereis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEREIS_SPEC;
impl crate::RegisterSpec for FEREIS_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`fereis::W`](W) writer structure"]
impl crate::Writable for FEREIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEREIS to value 0"]
impl crate::Resettable for FEREIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

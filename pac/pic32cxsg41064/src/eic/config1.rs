#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<Config1Spec>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<Config1Spec>;
#[doc = "Input Sense Configuration 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense8select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense8select> for u8 {
    #[inline(always)]
    fn from(variant: Sense8select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense8select {
    type Ux = u8;
}
impl crate::IsEnum for Sense8select {}
#[doc = "Field `SENSE8` reader - Input Sense Configuration 8"]
pub type Sense8R = crate::FieldReader<Sense8select>;
impl Sense8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense8select> {
        match self.bits {
            0 => Some(Sense8select::None),
            1 => Some(Sense8select::Rise),
            2 => Some(Sense8select::Fall),
            3 => Some(Sense8select::Both),
            4 => Some(Sense8select::High),
            5 => Some(Sense8select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense8select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense8select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense8select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense8select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense8select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense8select::Low
    }
}
#[doc = "Field `SENSE8` writer - Input Sense Configuration 8"]
pub type Sense8W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense8select>;
impl<'a, REG> Sense8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense8select::Low)
    }
}
#[doc = "Field `FILTEN8` reader - Filter Enable 8"]
pub type Filten8R = crate::BitReader;
#[doc = "Field `FILTEN8` writer - Filter Enable 8"]
pub type Filten8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense9select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense9select> for u8 {
    #[inline(always)]
    fn from(variant: Sense9select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense9select {
    type Ux = u8;
}
impl crate::IsEnum for Sense9select {}
#[doc = "Field `SENSE9` reader - Input Sense Configuration 9"]
pub type Sense9R = crate::FieldReader<Sense9select>;
impl Sense9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense9select> {
        match self.bits {
            0 => Some(Sense9select::None),
            1 => Some(Sense9select::Rise),
            2 => Some(Sense9select::Fall),
            3 => Some(Sense9select::Both),
            4 => Some(Sense9select::High),
            5 => Some(Sense9select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense9select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense9select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense9select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense9select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense9select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense9select::Low
    }
}
#[doc = "Field `SENSE9` writer - Input Sense Configuration 9"]
pub type Sense9W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense9select>;
impl<'a, REG> Sense9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense9select::Low)
    }
}
#[doc = "Field `FILTEN9` reader - Filter Enable 9"]
pub type Filten9R = crate::BitReader;
#[doc = "Field `FILTEN9` writer - Filter Enable 9"]
pub type Filten9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense10select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense10select> for u8 {
    #[inline(always)]
    fn from(variant: Sense10select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense10select {
    type Ux = u8;
}
impl crate::IsEnum for Sense10select {}
#[doc = "Field `SENSE10` reader - Input Sense Configuration 10"]
pub type Sense10R = crate::FieldReader<Sense10select>;
impl Sense10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense10select> {
        match self.bits {
            0 => Some(Sense10select::None),
            1 => Some(Sense10select::Rise),
            2 => Some(Sense10select::Fall),
            3 => Some(Sense10select::Both),
            4 => Some(Sense10select::High),
            5 => Some(Sense10select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense10select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense10select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense10select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense10select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense10select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense10select::Low
    }
}
#[doc = "Field `SENSE10` writer - Input Sense Configuration 10"]
pub type Sense10W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense10select>;
impl<'a, REG> Sense10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense10select::Low)
    }
}
#[doc = "Field `FILTEN10` reader - Filter Enable 10"]
pub type Filten10R = crate::BitReader;
#[doc = "Field `FILTEN10` writer - Filter Enable 10"]
pub type Filten10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense11select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense11select> for u8 {
    #[inline(always)]
    fn from(variant: Sense11select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense11select {
    type Ux = u8;
}
impl crate::IsEnum for Sense11select {}
#[doc = "Field `SENSE11` reader - Input Sense Configuration 11"]
pub type Sense11R = crate::FieldReader<Sense11select>;
impl Sense11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense11select> {
        match self.bits {
            0 => Some(Sense11select::None),
            1 => Some(Sense11select::Rise),
            2 => Some(Sense11select::Fall),
            3 => Some(Sense11select::Both),
            4 => Some(Sense11select::High),
            5 => Some(Sense11select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense11select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense11select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense11select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense11select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense11select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense11select::Low
    }
}
#[doc = "Field `SENSE11` writer - Input Sense Configuration 11"]
pub type Sense11W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense11select>;
impl<'a, REG> Sense11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense11select::Low)
    }
}
#[doc = "Field `FILTEN11` reader - Filter Enable 11"]
pub type Filten11R = crate::BitReader;
#[doc = "Field `FILTEN11` writer - Filter Enable 11"]
pub type Filten11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense12select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense12select> for u8 {
    #[inline(always)]
    fn from(variant: Sense12select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense12select {
    type Ux = u8;
}
impl crate::IsEnum for Sense12select {}
#[doc = "Field `SENSE12` reader - Input Sense Configuration 12"]
pub type Sense12R = crate::FieldReader<Sense12select>;
impl Sense12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense12select> {
        match self.bits {
            0 => Some(Sense12select::None),
            1 => Some(Sense12select::Rise),
            2 => Some(Sense12select::Fall),
            3 => Some(Sense12select::Both),
            4 => Some(Sense12select::High),
            5 => Some(Sense12select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense12select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense12select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense12select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense12select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense12select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense12select::Low
    }
}
#[doc = "Field `SENSE12` writer - Input Sense Configuration 12"]
pub type Sense12W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense12select>;
impl<'a, REG> Sense12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense12select::Low)
    }
}
#[doc = "Field `FILTEN12` reader - Filter Enable 12"]
pub type Filten12R = crate::BitReader;
#[doc = "Field `FILTEN12` writer - Filter Enable 12"]
pub type Filten12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense13select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense13select> for u8 {
    #[inline(always)]
    fn from(variant: Sense13select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense13select {
    type Ux = u8;
}
impl crate::IsEnum for Sense13select {}
#[doc = "Field `SENSE13` reader - Input Sense Configuration 13"]
pub type Sense13R = crate::FieldReader<Sense13select>;
impl Sense13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense13select> {
        match self.bits {
            0 => Some(Sense13select::None),
            1 => Some(Sense13select::Rise),
            2 => Some(Sense13select::Fall),
            3 => Some(Sense13select::Both),
            4 => Some(Sense13select::High),
            5 => Some(Sense13select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense13select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense13select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense13select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense13select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense13select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense13select::Low
    }
}
#[doc = "Field `SENSE13` writer - Input Sense Configuration 13"]
pub type Sense13W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense13select>;
impl<'a, REG> Sense13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense13select::Low)
    }
}
#[doc = "Field `FILTEN13` reader - Filter Enable 13"]
pub type Filten13R = crate::BitReader;
#[doc = "Field `FILTEN13` writer - Filter Enable 13"]
pub type Filten13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense14select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense14select> for u8 {
    #[inline(always)]
    fn from(variant: Sense14select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense14select {
    type Ux = u8;
}
impl crate::IsEnum for Sense14select {}
#[doc = "Field `SENSE14` reader - Input Sense Configuration 14"]
pub type Sense14R = crate::FieldReader<Sense14select>;
impl Sense14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense14select> {
        match self.bits {
            0 => Some(Sense14select::None),
            1 => Some(Sense14select::Rise),
            2 => Some(Sense14select::Fall),
            3 => Some(Sense14select::Both),
            4 => Some(Sense14select::High),
            5 => Some(Sense14select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense14select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense14select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense14select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense14select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense14select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense14select::Low
    }
}
#[doc = "Field `SENSE14` writer - Input Sense Configuration 14"]
pub type Sense14W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense14select>;
impl<'a, REG> Sense14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense14select::Low)
    }
}
#[doc = "Field `FILTEN14` reader - Filter Enable 14"]
pub type Filten14R = crate::BitReader;
#[doc = "Field `FILTEN14` writer - Filter Enable 14"]
pub type Filten14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Sense Configuration 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sense15select {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising edge detection"]
    Rise = 1,
    #[doc = "2: Falling edge detection"]
    Fall = 2,
    #[doc = "3: Both edges detection"]
    Both = 3,
    #[doc = "4: High level detection"]
    High = 4,
    #[doc = "5: Low level detection"]
    Low = 5,
}
impl From<Sense15select> for u8 {
    #[inline(always)]
    fn from(variant: Sense15select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sense15select {
    type Ux = u8;
}
impl crate::IsEnum for Sense15select {}
#[doc = "Field `SENSE15` reader - Input Sense Configuration 15"]
pub type Sense15R = crate::FieldReader<Sense15select>;
impl Sense15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sense15select> {
        match self.bits {
            0 => Some(Sense15select::None),
            1 => Some(Sense15select::Rise),
            2 => Some(Sense15select::Fall),
            3 => Some(Sense15select::Both),
            4 => Some(Sense15select::High),
            5 => Some(Sense15select::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sense15select::None
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Sense15select::Rise
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Sense15select::Fall
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Sense15select::Both
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sense15select::High
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sense15select::Low
    }
}
#[doc = "Field `SENSE15` writer - Input Sense Configuration 15"]
pub type Sense15W<'a, REG> = crate::FieldWriter<'a, REG, 3, Sense15select>;
impl<'a, REG> Sense15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::None)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::Rise)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::Fall)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::Both)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::High)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sense15select::Low)
    }
}
#[doc = "Field `FILTEN15` reader - Filter Enable 15"]
pub type Filten15R = crate::BitReader;
#[doc = "Field `FILTEN15` writer - Filter Enable 15"]
pub type Filten15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Input Sense Configuration 8"]
    #[inline(always)]
    pub fn sense8(&self) -> Sense8R {
        Sense8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Filter Enable 8"]
    #[inline(always)]
    pub fn filten8(&self) -> Filten8R {
        Filten8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 9"]
    #[inline(always)]
    pub fn sense9(&self) -> Sense9R {
        Sense9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Filter Enable 9"]
    #[inline(always)]
    pub fn filten9(&self) -> Filten9R {
        Filten9R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 10"]
    #[inline(always)]
    pub fn sense10(&self) -> Sense10R {
        Sense10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Filter Enable 10"]
    #[inline(always)]
    pub fn filten10(&self) -> Filten10R {
        Filten10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 11"]
    #[inline(always)]
    pub fn sense11(&self) -> Sense11R {
        Sense11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Filter Enable 11"]
    #[inline(always)]
    pub fn filten11(&self) -> Filten11R {
        Filten11R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 12"]
    #[inline(always)]
    pub fn sense12(&self) -> Sense12R {
        Sense12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Filter Enable 12"]
    #[inline(always)]
    pub fn filten12(&self) -> Filten12R {
        Filten12R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 13"]
    #[inline(always)]
    pub fn sense13(&self) -> Sense13R {
        Sense13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Filter Enable 13"]
    #[inline(always)]
    pub fn filten13(&self) -> Filten13R {
        Filten13R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 14"]
    #[inline(always)]
    pub fn sense14(&self) -> Sense14R {
        Sense14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Filter Enable 14"]
    #[inline(always)]
    pub fn filten14(&self) -> Filten14R {
        Filten14R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 15"]
    #[inline(always)]
    pub fn sense15(&self) -> Sense15R {
        Sense15R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Filter Enable 15"]
    #[inline(always)]
    pub fn filten15(&self) -> Filten15R {
        Filten15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Sense Configuration 8"]
    #[inline(always)]
    #[must_use]
    pub fn sense8(&mut self) -> Sense8W<Config1Spec> {
        Sense8W::new(self, 0)
    }
    #[doc = "Bit 3 - Filter Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn filten8(&mut self) -> Filten8W<Config1Spec> {
        Filten8W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 9"]
    #[inline(always)]
    #[must_use]
    pub fn sense9(&mut self) -> Sense9W<Config1Spec> {
        Sense9W::new(self, 4)
    }
    #[doc = "Bit 7 - Filter Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn filten9(&mut self) -> Filten9W<Config1Spec> {
        Filten9W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 10"]
    #[inline(always)]
    #[must_use]
    pub fn sense10(&mut self) -> Sense10W<Config1Spec> {
        Sense10W::new(self, 8)
    }
    #[doc = "Bit 11 - Filter Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn filten10(&mut self) -> Filten10W<Config1Spec> {
        Filten10W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 11"]
    #[inline(always)]
    #[must_use]
    pub fn sense11(&mut self) -> Sense11W<Config1Spec> {
        Sense11W::new(self, 12)
    }
    #[doc = "Bit 15 - Filter Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn filten11(&mut self) -> Filten11W<Config1Spec> {
        Filten11W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 12"]
    #[inline(always)]
    #[must_use]
    pub fn sense12(&mut self) -> Sense12W<Config1Spec> {
        Sense12W::new(self, 16)
    }
    #[doc = "Bit 19 - Filter Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn filten12(&mut self) -> Filten12W<Config1Spec> {
        Filten12W::new(self, 19)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 13"]
    #[inline(always)]
    #[must_use]
    pub fn sense13(&mut self) -> Sense13W<Config1Spec> {
        Sense13W::new(self, 20)
    }
    #[doc = "Bit 23 - Filter Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn filten13(&mut self) -> Filten13W<Config1Spec> {
        Filten13W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 14"]
    #[inline(always)]
    #[must_use]
    pub fn sense14(&mut self) -> Sense14W<Config1Spec> {
        Sense14W::new(self, 24)
    }
    #[doc = "Bit 27 - Filter Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn filten14(&mut self) -> Filten14W<Config1Spec> {
        Filten14W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 15"]
    #[inline(always)]
    #[must_use]
    pub fn sense15(&mut self) -> Sense15W<Config1Spec> {
        Sense15W::new(self, 28)
    }
    #[doc = "Bit 31 - Filter Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn filten15(&mut self) -> Filten15W<Config1Spec> {
        Filten15W::new(self, 31)
    }
}
#[doc = "External Interrupt Sense Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config1Spec;
impl crate::RegisterSpec for Config1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for Config1Spec {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for Config1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for Config1Spec {
    const RESET_VALUE: u32 = 0;
}

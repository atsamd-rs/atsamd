#[doc = "Register `LUTCTRL[%s]` reader"]
pub type R = crate::R<LutctrlSpec>;
#[doc = "Register `LUTCTRL[%s]` writer"]
pub type W = crate::W<LutctrlSpec>;
#[doc = "LUT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableselect {
    #[doc = "0: LUT block is disabled"]
    Disable = 0,
    #[doc = "1: LUT block is enabled"]
    Enable = 1,
}
impl From<Enableselect> for bool {
    #[inline(always)]
    fn from(variant: Enableselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub type EnableR = crate::BitReader<Enableselect>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableselect {
        match self.bits {
            false => Enableselect::Disable,
            true => Enableselect::Enable,
        }
    }
    #[doc = "LUT block is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enableselect::Disable
    }
    #[doc = "LUT block is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enableselect::Enable
    }
}
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enableselect>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LUT block is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Disable)
    }
    #[doc = "LUT block is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enableselect::Enable)
    }
}
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filtselselect {
    #[doc = "0: Filter disabled"]
    Disable = 0,
    #[doc = "1: Synchronizer enabled"]
    Synch = 1,
    #[doc = "2: Filter enabled"]
    Filter = 2,
}
impl From<Filtselselect> for u8 {
    #[inline(always)]
    fn from(variant: Filtselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filtselselect {
    type Ux = u8;
}
impl crate::IsEnum for Filtselselect {}
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub type FiltselR = crate::FieldReader<Filtselselect>;
impl FiltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Filtselselect> {
        match self.bits {
            0 => Some(Filtselselect::Disable),
            1 => Some(Filtselselect::Synch),
            2 => Some(Filtselselect::Filter),
            _ => None,
        }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Filtselselect::Disable
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == Filtselselect::Synch
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == Filtselselect::Filter
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub type FiltselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filtselselect>;
impl<'a, REG> FiltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Disable)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Synch)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Filter)
    }
}
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edgeselselect {
    #[doc = "0: Edge detector is disabled"]
    Disable = 0,
    #[doc = "1: Edge detector is enabled"]
    Enable = 1,
}
impl From<Edgeselselect> for bool {
    #[inline(always)]
    fn from(variant: Edgeselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGESEL` reader - Edge Selection"]
pub type EdgeselR = crate::BitReader<Edgeselselect>;
impl EdgeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edgeselselect {
        match self.bits {
            false => Edgeselselect::Disable,
            true => Edgeselselect::Enable,
        }
    }
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Edgeselselect::Disable
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Edgeselselect::Enable
    }
}
#[doc = "Field `EDGESEL` writer - Edge Selection"]
pub type EdgeselW<'a, REG> = crate::BitWriter<'a, REG, Edgeselselect>;
impl<'a, REG> EdgeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detector is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Edgeselselect::Disable)
    }
    #[doc = "Edge detector is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Edgeselselect::Enable)
    }
}
#[doc = "Input Selection 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel0select {
    #[doc = "0: Masked input"]
    Mask = 0,
    #[doc = "1: Feedback input source"]
    Feedback = 1,
    #[doc = "2: Linked LUT input source"]
    Link = 2,
    #[doc = "3: Event input source"]
    Event = 3,
    #[doc = "4: I/O pin input source"]
    Io = 4,
    #[doc = "5: AC input source"]
    Ac = 5,
    #[doc = "6: TC input source"]
    Tc = 6,
    #[doc = "7: Alternate TC input source"]
    Alttc = 7,
    #[doc = "8: TCC input source"]
    Tcc = 8,
    #[doc = "9: SERCOM input source"]
    Sercom = 9,
}
impl From<Insel0select> for u8 {
    #[inline(always)]
    fn from(variant: Insel0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel0select {
    type Ux = u8;
}
impl crate::IsEnum for Insel0select {}
#[doc = "Field `INSEL0` reader - Input Selection 0"]
pub type Insel0R = crate::FieldReader<Insel0select>;
impl Insel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Insel0select> {
        match self.bits {
            0 => Some(Insel0select::Mask),
            1 => Some(Insel0select::Feedback),
            2 => Some(Insel0select::Link),
            3 => Some(Insel0select::Event),
            4 => Some(Insel0select::Io),
            5 => Some(Insel0select::Ac),
            6 => Some(Insel0select::Tc),
            7 => Some(Insel0select::Alttc),
            8 => Some(Insel0select::Tcc),
            9 => Some(Insel0select::Sercom),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Insel0select::Mask
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == Insel0select::Feedback
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == Insel0select::Link
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Insel0select::Event
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Insel0select::Io
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == Insel0select::Ac
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == Insel0select::Tc
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == Insel0select::Alttc
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == Insel0select::Tcc
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == Insel0select::Sercom
    }
}
#[doc = "Field `INSEL0` writer - Input Selection 0"]
pub type Insel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Insel0select>;
impl<'a, REG> Insel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Mask)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Feedback)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Link)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Event)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Io)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Ac)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Tc)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Alttc)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Tcc)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Sercom)
    }
}
#[doc = "Input Selection 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel1select {
    #[doc = "0: Masked input"]
    Mask = 0,
    #[doc = "1: Feedback input source"]
    Feedback = 1,
    #[doc = "2: Linked LUT input source"]
    Link = 2,
    #[doc = "3: Event input source"]
    Event = 3,
    #[doc = "4: I/O pin input source"]
    Io = 4,
    #[doc = "5: AC input source"]
    Ac = 5,
    #[doc = "6: TC input source"]
    Tc = 6,
    #[doc = "7: Alternate TC input source"]
    Alttc = 7,
    #[doc = "8: TCC input source"]
    Tcc = 8,
    #[doc = "9: SERCOM input source"]
    Sercom = 9,
}
impl From<Insel1select> for u8 {
    #[inline(always)]
    fn from(variant: Insel1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel1select {
    type Ux = u8;
}
impl crate::IsEnum for Insel1select {}
#[doc = "Field `INSEL1` reader - Input Selection 1"]
pub type Insel1R = crate::FieldReader<Insel1select>;
impl Insel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Insel1select> {
        match self.bits {
            0 => Some(Insel1select::Mask),
            1 => Some(Insel1select::Feedback),
            2 => Some(Insel1select::Link),
            3 => Some(Insel1select::Event),
            4 => Some(Insel1select::Io),
            5 => Some(Insel1select::Ac),
            6 => Some(Insel1select::Tc),
            7 => Some(Insel1select::Alttc),
            8 => Some(Insel1select::Tcc),
            9 => Some(Insel1select::Sercom),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Insel1select::Mask
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == Insel1select::Feedback
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == Insel1select::Link
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Insel1select::Event
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Insel1select::Io
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == Insel1select::Ac
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == Insel1select::Tc
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == Insel1select::Alttc
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == Insel1select::Tcc
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == Insel1select::Sercom
    }
}
#[doc = "Field `INSEL1` writer - Input Selection 1"]
pub type Insel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Insel1select>;
impl<'a, REG> Insel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Mask)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Feedback)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Link)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Event)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Io)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Ac)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Tc)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Alttc)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Tcc)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut crate::W<REG> {
        self.variant(Insel1select::Sercom)
    }
}
#[doc = "Input Selection 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel2select {
    #[doc = "0: Masked input"]
    Mask = 0,
    #[doc = "1: Feedback input source"]
    Feedback = 1,
    #[doc = "2: Linked LUT input source"]
    Link = 2,
    #[doc = "3: Event input source"]
    Event = 3,
    #[doc = "4: I/O pin input source"]
    Io = 4,
    #[doc = "5: AC input source"]
    Ac = 5,
    #[doc = "6: TC input source"]
    Tc = 6,
    #[doc = "7: Alternate TC input source"]
    Alttc = 7,
    #[doc = "8: TCC input source"]
    Tcc = 8,
    #[doc = "9: SERCOM input source"]
    Sercom = 9,
}
impl From<Insel2select> for u8 {
    #[inline(always)]
    fn from(variant: Insel2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel2select {
    type Ux = u8;
}
impl crate::IsEnum for Insel2select {}
#[doc = "Field `INSEL2` reader - Input Selection 2"]
pub type Insel2R = crate::FieldReader<Insel2select>;
impl Insel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Insel2select> {
        match self.bits {
            0 => Some(Insel2select::Mask),
            1 => Some(Insel2select::Feedback),
            2 => Some(Insel2select::Link),
            3 => Some(Insel2select::Event),
            4 => Some(Insel2select::Io),
            5 => Some(Insel2select::Ac),
            6 => Some(Insel2select::Tc),
            7 => Some(Insel2select::Alttc),
            8 => Some(Insel2select::Tcc),
            9 => Some(Insel2select::Sercom),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Insel2select::Mask
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == Insel2select::Feedback
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == Insel2select::Link
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Insel2select::Event
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Insel2select::Io
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == Insel2select::Ac
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == Insel2select::Tc
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == Insel2select::Alttc
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == Insel2select::Tcc
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == Insel2select::Sercom
    }
}
#[doc = "Field `INSEL2` writer - Input Selection 2"]
pub type Insel2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Insel2select>;
impl<'a, REG> Insel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Mask)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Feedback)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Link)
    }
    #[doc = "Event input source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Event)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Io)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Ac)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Tc)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Alttc)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Tcc)
    }
    #[doc = "SERCOM input source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut crate::W<REG> {
        self.variant(Insel2select::Sercom)
    }
}
#[doc = "Inverted Event Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inveiselect {
    #[doc = "0: Incoming event is not inverted"]
    Disable = 0,
    #[doc = "1: Incoming event is inverted"]
    Enable = 1,
}
impl From<Inveiselect> for bool {
    #[inline(always)]
    fn from(variant: Inveiselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVEI` reader - Inverted Event Input Enable"]
pub type InveiR = crate::BitReader<Inveiselect>;
impl InveiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inveiselect {
        match self.bits {
            false => Inveiselect::Disable,
            true => Inveiselect::Enable,
        }
    }
    #[doc = "Incoming event is not inverted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inveiselect::Disable
    }
    #[doc = "Incoming event is inverted"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inveiselect::Enable
    }
}
#[doc = "Field `INVEI` writer - Inverted Event Input Enable"]
pub type InveiW<'a, REG> = crate::BitWriter<'a, REG, Inveiselect>;
impl<'a, REG> InveiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Incoming event is not inverted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inveiselect::Disable)
    }
    #[doc = "Incoming event is inverted"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inveiselect::Enable)
    }
}
#[doc = "LUT Event Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Luteiselect {
    #[doc = "0: LUT incoming event is disabled"]
    Disable = 0,
    #[doc = "1: LUT incoming event is enabled"]
    Enable = 1,
}
impl From<Luteiselect> for bool {
    #[inline(always)]
    fn from(variant: Luteiselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUTEI` reader - LUT Event Input Enable"]
pub type LuteiR = crate::BitReader<Luteiselect>;
impl LuteiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Luteiselect {
        match self.bits {
            false => Luteiselect::Disable,
            true => Luteiselect::Enable,
        }
    }
    #[doc = "LUT incoming event is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Luteiselect::Disable
    }
    #[doc = "LUT incoming event is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Luteiselect::Enable
    }
}
#[doc = "Field `LUTEI` writer - LUT Event Input Enable"]
pub type LuteiW<'a, REG> = crate::BitWriter<'a, REG, Luteiselect>;
impl<'a, REG> LuteiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LUT incoming event is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Luteiselect::Disable)
    }
    #[doc = "LUT incoming event is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Luteiselect::Enable)
    }
}
#[doc = "LUT Event Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Luteoselect {
    #[doc = "0: LUT event output is disabled"]
    Disable = 0,
    #[doc = "1: LUT event output is enabled"]
    Enable = 1,
}
impl From<Luteoselect> for bool {
    #[inline(always)]
    fn from(variant: Luteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LUTEO` reader - LUT Event Output Enable"]
pub type LuteoR = crate::BitReader<Luteoselect>;
impl LuteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Luteoselect {
        match self.bits {
            false => Luteoselect::Disable,
            true => Luteoselect::Enable,
        }
    }
    #[doc = "LUT event output is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Luteoselect::Disable
    }
    #[doc = "LUT event output is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Luteoselect::Enable
    }
}
#[doc = "Field `LUTEO` writer - LUT Event Output Enable"]
pub type LuteoW<'a, REG> = crate::BitWriter<'a, REG, Luteoselect>;
impl<'a, REG> LuteoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LUT event output is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Luteoselect::Disable)
    }
    #[doc = "LUT event output is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Luteoselect::Enable)
    }
}
#[doc = "Field `TRUTH` reader - Truth Value"]
pub type TruthR = crate::FieldReader;
#[doc = "Field `TRUTH` writer - Truth Value"]
pub type TruthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FiltselR {
        FiltselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&self) -> EdgeselR {
        EdgeselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&self) -> Insel0R {
        Insel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&self) -> Insel1R {
        Insel1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&self) -> Insel2R {
        Insel2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn invei(&self) -> InveiR {
        InveiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&self) -> LuteiR {
        LuteiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&self) -> LuteoR {
        LuteoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&self) -> TruthR {
        TruthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<LutctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FiltselW<LutctrlSpec> {
        FiltselW::new(self, 4)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgesel(&mut self) -> EdgeselW<LutctrlSpec> {
        EdgeselW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> Insel0W<LutctrlSpec> {
        Insel0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> Insel1W<LutctrlSpec> {
        Insel1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> Insel2W<LutctrlSpec> {
        Insel2W::new(self, 16)
    }
    #[doc = "Bit 20 - Inverted Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei(&mut self) -> InveiW<LutctrlSpec> {
        InveiW::new(self, 20)
    }
    #[doc = "Bit 21 - LUT Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lutei(&mut self) -> LuteiW<LutctrlSpec> {
        LuteiW::new(self, 21)
    }
    #[doc = "Bit 22 - LUT Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn luteo(&mut self) -> LuteoW<LutctrlSpec> {
        LuteoW::new(self, 22)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    #[must_use]
    pub fn truth(&mut self) -> TruthW<LutctrlSpec> {
        TruthW::new(self, 24)
    }
}
#[doc = "LUT Control x\n\nYou can [`read`](crate::Reg::read) this register and get [`lutctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutctrlSpec;
impl crate::RegisterSpec for LutctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lutctrl::R`](R) reader structure"]
impl crate::Readable for LutctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lutctrl::W`](W) writer structure"]
impl crate::Writable for LutctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTCTRL[%s]
to value 0"]
impl crate::Resettable for LutctrlSpec {
    const RESET_VALUE: u32 = 0;
}

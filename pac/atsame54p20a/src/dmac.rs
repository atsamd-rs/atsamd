#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - CRC Control"]
    pub crcctrl: CRCCTRL,
    #[doc = "0x04 - CRC Data Input"]
    pub crcdatain: CRCDATAIN,
    #[doc = "0x08 - CRC Checksum"]
    pub crcchksum: CRCCHKSUM,
    #[doc = "0x0c - CRC Status"]
    pub crcstatus: CRCSTATUS,
    #[doc = "0x0d - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved6: [u8; 2usize],
    #[doc = "0x10 - Software Trigger Control"]
    pub swtrigctrl: SWTRIGCTRL,
    #[doc = "0x14 - Priority Control 0"]
    pub prictrl0: PRICTRL0,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Interrupt Pending"]
    pub intpend: INTPEND,
    _reserved9: [u8; 2usize],
    #[doc = "0x24 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x28 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x2c - Pending Channels"]
    pub pendch: PENDCH,
    #[doc = "0x30 - Active Channel and Levels"]
    pub active: ACTIVE,
    #[doc = "0x34 - Descriptor Memory Section Base Address"]
    pub baseaddr: BASEADDR,
    #[doc = "0x38 - Write-Back Memory Section Base Address"]
    pub wrbaddr: WRBADDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - Channel n Control A"]
    pub chctrla0: CHCTRLA,
    #[doc = "0x44 - Channel n Control B"]
    pub chctrlb0: CHCTRLB,
    #[doc = "0x45 - Channel n Priority Level"]
    pub chprilvl0: CHPRILVL,
    #[doc = "0x46 - Channel n Event Control"]
    pub chevctrl0: CHEVCTRL,
    _reserved19: [u8; 5usize],
    #[doc = "0x4c - Channel n Interrupt Enable Clear"]
    pub chintenclr0: CHINTENCLR,
    #[doc = "0x4d - Channel n Interrupt Enable Set"]
    pub chintenset0: CHINTENSET,
    #[doc = "0x4e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag0: CHINTFLAG,
    #[doc = "0x4f - Channel n Status"]
    pub chstatus0: CHSTATUS,
    #[doc = "0x50 - Channel n Control A"]
    pub chctrla1: CHCTRLA,
    #[doc = "0x54 - Channel n Control B"]
    pub chctrlb1: CHCTRLB,
    #[doc = "0x55 - Channel n Priority Level"]
    pub chprilvl1: CHPRILVL,
    #[doc = "0x56 - Channel n Event Control"]
    pub chevctrl1: CHEVCTRL,
    _reserved27: [u8; 5usize],
    #[doc = "0x5c - Channel n Interrupt Enable Clear"]
    pub chintenclr1: CHINTENCLR,
    #[doc = "0x5d - Channel n Interrupt Enable Set"]
    pub chintenset1: CHINTENSET,
    #[doc = "0x5e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag1: CHINTFLAG,
    #[doc = "0x5f - Channel n Status"]
    pub chstatus1: CHSTATUS,
    #[doc = "0x60 - Channel n Control A"]
    pub chctrla2: CHCTRLA,
    #[doc = "0x64 - Channel n Control B"]
    pub chctrlb2: CHCTRLB,
    #[doc = "0x65 - Channel n Priority Level"]
    pub chprilvl2: CHPRILVL,
    #[doc = "0x66 - Channel n Event Control"]
    pub chevctrl2: CHEVCTRL,
    _reserved35: [u8; 5usize],
    #[doc = "0x6c - Channel n Interrupt Enable Clear"]
    pub chintenclr2: CHINTENCLR,
    #[doc = "0x6d - Channel n Interrupt Enable Set"]
    pub chintenset2: CHINTENSET,
    #[doc = "0x6e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag2: CHINTFLAG,
    #[doc = "0x6f - Channel n Status"]
    pub chstatus2: CHSTATUS,
    #[doc = "0x70 - Channel n Control A"]
    pub chctrla3: CHCTRLA,
    #[doc = "0x74 - Channel n Control B"]
    pub chctrlb3: CHCTRLB,
    #[doc = "0x75 - Channel n Priority Level"]
    pub chprilvl3: CHPRILVL,
    #[doc = "0x76 - Channel n Event Control"]
    pub chevctrl3: CHEVCTRL,
    _reserved43: [u8; 5usize],
    #[doc = "0x7c - Channel n Interrupt Enable Clear"]
    pub chintenclr3: CHINTENCLR,
    #[doc = "0x7d - Channel n Interrupt Enable Set"]
    pub chintenset3: CHINTENSET,
    #[doc = "0x7e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag3: CHINTFLAG,
    #[doc = "0x7f - Channel n Status"]
    pub chstatus3: CHSTATUS,
    #[doc = "0x80 - Channel n Control A"]
    pub chctrla4: CHCTRLA,
    #[doc = "0x84 - Channel n Control B"]
    pub chctrlb4: CHCTRLB,
    #[doc = "0x85 - Channel n Priority Level"]
    pub chprilvl4: CHPRILVL,
    #[doc = "0x86 - Channel n Event Control"]
    pub chevctrl4: CHEVCTRL,
    _reserved51: [u8; 5usize],
    #[doc = "0x8c - Channel n Interrupt Enable Clear"]
    pub chintenclr4: CHINTENCLR,
    #[doc = "0x8d - Channel n Interrupt Enable Set"]
    pub chintenset4: CHINTENSET,
    #[doc = "0x8e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag4: CHINTFLAG,
    #[doc = "0x8f - Channel n Status"]
    pub chstatus4: CHSTATUS,
    #[doc = "0x90 - Channel n Control A"]
    pub chctrla5: CHCTRLA,
    #[doc = "0x94 - Channel n Control B"]
    pub chctrlb5: CHCTRLB,
    #[doc = "0x95 - Channel n Priority Level"]
    pub chprilvl5: CHPRILVL,
    #[doc = "0x96 - Channel n Event Control"]
    pub chevctrl5: CHEVCTRL,
    _reserved59: [u8; 5usize],
    #[doc = "0x9c - Channel n Interrupt Enable Clear"]
    pub chintenclr5: CHINTENCLR,
    #[doc = "0x9d - Channel n Interrupt Enable Set"]
    pub chintenset5: CHINTENSET,
    #[doc = "0x9e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag5: CHINTFLAG,
    #[doc = "0x9f - Channel n Status"]
    pub chstatus5: CHSTATUS,
    #[doc = "0xa0 - Channel n Control A"]
    pub chctrla6: CHCTRLA,
    #[doc = "0xa4 - Channel n Control B"]
    pub chctrlb6: CHCTRLB,
    #[doc = "0xa5 - Channel n Priority Level"]
    pub chprilvl6: CHPRILVL,
    #[doc = "0xa6 - Channel n Event Control"]
    pub chevctrl6: CHEVCTRL,
    _reserved67: [u8; 5usize],
    #[doc = "0xac - Channel n Interrupt Enable Clear"]
    pub chintenclr6: CHINTENCLR,
    #[doc = "0xad - Channel n Interrupt Enable Set"]
    pub chintenset6: CHINTENSET,
    #[doc = "0xae - Channel n Interrupt Flag Status and Clear"]
    pub chintflag6: CHINTFLAG,
    #[doc = "0xaf - Channel n Status"]
    pub chstatus6: CHSTATUS,
    #[doc = "0xb0 - Channel n Control A"]
    pub chctrla7: CHCTRLA,
    #[doc = "0xb4 - Channel n Control B"]
    pub chctrlb7: CHCTRLB,
    #[doc = "0xb5 - Channel n Priority Level"]
    pub chprilvl7: CHPRILVL,
    #[doc = "0xb6 - Channel n Event Control"]
    pub chevctrl7: CHEVCTRL,
    _reserved75: [u8; 5usize],
    #[doc = "0xbc - Channel n Interrupt Enable Clear"]
    pub chintenclr7: CHINTENCLR,
    #[doc = "0xbd - Channel n Interrupt Enable Set"]
    pub chintenset7: CHINTENSET,
    #[doc = "0xbe - Channel n Interrupt Flag Status and Clear"]
    pub chintflag7: CHINTFLAG,
    #[doc = "0xbf - Channel n Status"]
    pub chstatus7: CHSTATUS,
    #[doc = "0xc0 - Channel n Control A"]
    pub chctrla8: CHCTRLA,
    #[doc = "0xc4 - Channel n Control B"]
    pub chctrlb8: CHCTRLB,
    #[doc = "0xc5 - Channel n Priority Level"]
    pub chprilvl8: CHPRILVL,
    #[doc = "0xc6 - Channel n Event Control"]
    pub chevctrl8: CHEVCTRL,
    _reserved83: [u8; 5usize],
    #[doc = "0xcc - Channel n Interrupt Enable Clear"]
    pub chintenclr8: CHINTENCLR,
    #[doc = "0xcd - Channel n Interrupt Enable Set"]
    pub chintenset8: CHINTENSET,
    #[doc = "0xce - Channel n Interrupt Flag Status and Clear"]
    pub chintflag8: CHINTFLAG,
    #[doc = "0xcf - Channel n Status"]
    pub chstatus8: CHSTATUS,
    #[doc = "0xd0 - Channel n Control A"]
    pub chctrla9: CHCTRLA,
    #[doc = "0xd4 - Channel n Control B"]
    pub chctrlb9: CHCTRLB,
    #[doc = "0xd5 - Channel n Priority Level"]
    pub chprilvl9: CHPRILVL,
    #[doc = "0xd6 - Channel n Event Control"]
    pub chevctrl9: CHEVCTRL,
    _reserved91: [u8; 5usize],
    #[doc = "0xdc - Channel n Interrupt Enable Clear"]
    pub chintenclr9: CHINTENCLR,
    #[doc = "0xdd - Channel n Interrupt Enable Set"]
    pub chintenset9: CHINTENSET,
    #[doc = "0xde - Channel n Interrupt Flag Status and Clear"]
    pub chintflag9: CHINTFLAG,
    #[doc = "0xdf - Channel n Status"]
    pub chstatus9: CHSTATUS,
    #[doc = "0xe0 - Channel n Control A"]
    pub chctrla10: CHCTRLA,
    #[doc = "0xe4 - Channel n Control B"]
    pub chctrlb10: CHCTRLB,
    #[doc = "0xe5 - Channel n Priority Level"]
    pub chprilvl10: CHPRILVL,
    #[doc = "0xe6 - Channel n Event Control"]
    pub chevctrl10: CHEVCTRL,
    _reserved99: [u8; 5usize],
    #[doc = "0xec - Channel n Interrupt Enable Clear"]
    pub chintenclr10: CHINTENCLR,
    #[doc = "0xed - Channel n Interrupt Enable Set"]
    pub chintenset10: CHINTENSET,
    #[doc = "0xee - Channel n Interrupt Flag Status and Clear"]
    pub chintflag10: CHINTFLAG,
    #[doc = "0xef - Channel n Status"]
    pub chstatus10: CHSTATUS,
    #[doc = "0xf0 - Channel n Control A"]
    pub chctrla11: CHCTRLA,
    #[doc = "0xf4 - Channel n Control B"]
    pub chctrlb11: CHCTRLB,
    #[doc = "0xf5 - Channel n Priority Level"]
    pub chprilvl11: CHPRILVL,
    #[doc = "0xf6 - Channel n Event Control"]
    pub chevctrl11: CHEVCTRL,
    _reserved107: [u8; 5usize],
    #[doc = "0xfc - Channel n Interrupt Enable Clear"]
    pub chintenclr11: CHINTENCLR,
    #[doc = "0xfd - Channel n Interrupt Enable Set"]
    pub chintenset11: CHINTENSET,
    #[doc = "0xfe - Channel n Interrupt Flag Status and Clear"]
    pub chintflag11: CHINTFLAG,
    #[doc = "0xff - Channel n Status"]
    pub chstatus11: CHSTATUS,
    #[doc = "0x100 - Channel n Control A"]
    pub chctrla12: CHCTRLA,
    #[doc = "0x104 - Channel n Control B"]
    pub chctrlb12: CHCTRLB,
    #[doc = "0x105 - Channel n Priority Level"]
    pub chprilvl12: CHPRILVL,
    #[doc = "0x106 - Channel n Event Control"]
    pub chevctrl12: CHEVCTRL,
    _reserved115: [u8; 5usize],
    #[doc = "0x10c - Channel n Interrupt Enable Clear"]
    pub chintenclr12: CHINTENCLR,
    #[doc = "0x10d - Channel n Interrupt Enable Set"]
    pub chintenset12: CHINTENSET,
    #[doc = "0x10e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag12: CHINTFLAG,
    #[doc = "0x10f - Channel n Status"]
    pub chstatus12: CHSTATUS,
    #[doc = "0x110 - Channel n Control A"]
    pub chctrla13: CHCTRLA,
    #[doc = "0x114 - Channel n Control B"]
    pub chctrlb13: CHCTRLB,
    #[doc = "0x115 - Channel n Priority Level"]
    pub chprilvl13: CHPRILVL,
    #[doc = "0x116 - Channel n Event Control"]
    pub chevctrl13: CHEVCTRL,
    _reserved123: [u8; 5usize],
    #[doc = "0x11c - Channel n Interrupt Enable Clear"]
    pub chintenclr13: CHINTENCLR,
    #[doc = "0x11d - Channel n Interrupt Enable Set"]
    pub chintenset13: CHINTENSET,
    #[doc = "0x11e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag13: CHINTFLAG,
    #[doc = "0x11f - Channel n Status"]
    pub chstatus13: CHSTATUS,
    #[doc = "0x120 - Channel n Control A"]
    pub chctrla14: CHCTRLA,
    #[doc = "0x124 - Channel n Control B"]
    pub chctrlb14: CHCTRLB,
    #[doc = "0x125 - Channel n Priority Level"]
    pub chprilvl14: CHPRILVL,
    #[doc = "0x126 - Channel n Event Control"]
    pub chevctrl14: CHEVCTRL,
    _reserved131: [u8; 5usize],
    #[doc = "0x12c - Channel n Interrupt Enable Clear"]
    pub chintenclr14: CHINTENCLR,
    #[doc = "0x12d - Channel n Interrupt Enable Set"]
    pub chintenset14: CHINTENSET,
    #[doc = "0x12e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag14: CHINTFLAG,
    #[doc = "0x12f - Channel n Status"]
    pub chstatus14: CHSTATUS,
    #[doc = "0x130 - Channel n Control A"]
    pub chctrla15: CHCTRLA,
    #[doc = "0x134 - Channel n Control B"]
    pub chctrlb15: CHCTRLB,
    #[doc = "0x135 - Channel n Priority Level"]
    pub chprilvl15: CHPRILVL,
    #[doc = "0x136 - Channel n Event Control"]
    pub chevctrl15: CHEVCTRL,
    _reserved139: [u8; 5usize],
    #[doc = "0x13c - Channel n Interrupt Enable Clear"]
    pub chintenclr15: CHINTENCLR,
    #[doc = "0x13d - Channel n Interrupt Enable Set"]
    pub chintenset15: CHINTENSET,
    #[doc = "0x13e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag15: CHINTFLAG,
    #[doc = "0x13f - Channel n Status"]
    pub chstatus15: CHSTATUS,
    #[doc = "0x140 - Channel n Control A"]
    pub chctrla16: CHCTRLA,
    #[doc = "0x144 - Channel n Control B"]
    pub chctrlb16: CHCTRLB,
    #[doc = "0x145 - Channel n Priority Level"]
    pub chprilvl16: CHPRILVL,
    #[doc = "0x146 - Channel n Event Control"]
    pub chevctrl16: CHEVCTRL,
    _reserved147: [u8; 5usize],
    #[doc = "0x14c - Channel n Interrupt Enable Clear"]
    pub chintenclr16: CHINTENCLR,
    #[doc = "0x14d - Channel n Interrupt Enable Set"]
    pub chintenset16: CHINTENSET,
    #[doc = "0x14e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag16: CHINTFLAG,
    #[doc = "0x14f - Channel n Status"]
    pub chstatus16: CHSTATUS,
    #[doc = "0x150 - Channel n Control A"]
    pub chctrla17: CHCTRLA,
    #[doc = "0x154 - Channel n Control B"]
    pub chctrlb17: CHCTRLB,
    #[doc = "0x155 - Channel n Priority Level"]
    pub chprilvl17: CHPRILVL,
    #[doc = "0x156 - Channel n Event Control"]
    pub chevctrl17: CHEVCTRL,
    _reserved155: [u8; 5usize],
    #[doc = "0x15c - Channel n Interrupt Enable Clear"]
    pub chintenclr17: CHINTENCLR,
    #[doc = "0x15d - Channel n Interrupt Enable Set"]
    pub chintenset17: CHINTENSET,
    #[doc = "0x15e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag17: CHINTFLAG,
    #[doc = "0x15f - Channel n Status"]
    pub chstatus17: CHSTATUS,
    #[doc = "0x160 - Channel n Control A"]
    pub chctrla18: CHCTRLA,
    #[doc = "0x164 - Channel n Control B"]
    pub chctrlb18: CHCTRLB,
    #[doc = "0x165 - Channel n Priority Level"]
    pub chprilvl18: CHPRILVL,
    #[doc = "0x166 - Channel n Event Control"]
    pub chevctrl18: CHEVCTRL,
    _reserved163: [u8; 5usize],
    #[doc = "0x16c - Channel n Interrupt Enable Clear"]
    pub chintenclr18: CHINTENCLR,
    #[doc = "0x16d - Channel n Interrupt Enable Set"]
    pub chintenset18: CHINTENSET,
    #[doc = "0x16e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag18: CHINTFLAG,
    #[doc = "0x16f - Channel n Status"]
    pub chstatus18: CHSTATUS,
    #[doc = "0x170 - Channel n Control A"]
    pub chctrla19: CHCTRLA,
    #[doc = "0x174 - Channel n Control B"]
    pub chctrlb19: CHCTRLB,
    #[doc = "0x175 - Channel n Priority Level"]
    pub chprilvl19: CHPRILVL,
    #[doc = "0x176 - Channel n Event Control"]
    pub chevctrl19: CHEVCTRL,
    _reserved171: [u8; 5usize],
    #[doc = "0x17c - Channel n Interrupt Enable Clear"]
    pub chintenclr19: CHINTENCLR,
    #[doc = "0x17d - Channel n Interrupt Enable Set"]
    pub chintenset19: CHINTENSET,
    #[doc = "0x17e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag19: CHINTFLAG,
    #[doc = "0x17f - Channel n Status"]
    pub chstatus19: CHSTATUS,
    #[doc = "0x180 - Channel n Control A"]
    pub chctrla20: CHCTRLA,
    #[doc = "0x184 - Channel n Control B"]
    pub chctrlb20: CHCTRLB,
    #[doc = "0x185 - Channel n Priority Level"]
    pub chprilvl20: CHPRILVL,
    #[doc = "0x186 - Channel n Event Control"]
    pub chevctrl20: CHEVCTRL,
    _reserved179: [u8; 5usize],
    #[doc = "0x18c - Channel n Interrupt Enable Clear"]
    pub chintenclr20: CHINTENCLR,
    #[doc = "0x18d - Channel n Interrupt Enable Set"]
    pub chintenset20: CHINTENSET,
    #[doc = "0x18e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag20: CHINTFLAG,
    #[doc = "0x18f - Channel n Status"]
    pub chstatus20: CHSTATUS,
    #[doc = "0x190 - Channel n Control A"]
    pub chctrla21: CHCTRLA,
    #[doc = "0x194 - Channel n Control B"]
    pub chctrlb21: CHCTRLB,
    #[doc = "0x195 - Channel n Priority Level"]
    pub chprilvl21: CHPRILVL,
    #[doc = "0x196 - Channel n Event Control"]
    pub chevctrl21: CHEVCTRL,
    _reserved187: [u8; 5usize],
    #[doc = "0x19c - Channel n Interrupt Enable Clear"]
    pub chintenclr21: CHINTENCLR,
    #[doc = "0x19d - Channel n Interrupt Enable Set"]
    pub chintenset21: CHINTENSET,
    #[doc = "0x19e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag21: CHINTFLAG,
    #[doc = "0x19f - Channel n Status"]
    pub chstatus21: CHSTATUS,
    #[doc = "0x1a0 - Channel n Control A"]
    pub chctrla22: CHCTRLA,
    #[doc = "0x1a4 - Channel n Control B"]
    pub chctrlb22: CHCTRLB,
    #[doc = "0x1a5 - Channel n Priority Level"]
    pub chprilvl22: CHPRILVL,
    #[doc = "0x1a6 - Channel n Event Control"]
    pub chevctrl22: CHEVCTRL,
    _reserved195: [u8; 5usize],
    #[doc = "0x1ac - Channel n Interrupt Enable Clear"]
    pub chintenclr22: CHINTENCLR,
    #[doc = "0x1ad - Channel n Interrupt Enable Set"]
    pub chintenset22: CHINTENSET,
    #[doc = "0x1ae - Channel n Interrupt Flag Status and Clear"]
    pub chintflag22: CHINTFLAG,
    #[doc = "0x1af - Channel n Status"]
    pub chstatus22: CHSTATUS,
    #[doc = "0x1b0 - Channel n Control A"]
    pub chctrla23: CHCTRLA,
    #[doc = "0x1b4 - Channel n Control B"]
    pub chctrlb23: CHCTRLB,
    #[doc = "0x1b5 - Channel n Priority Level"]
    pub chprilvl23: CHPRILVL,
    #[doc = "0x1b6 - Channel n Event Control"]
    pub chevctrl23: CHEVCTRL,
    _reserved203: [u8; 5usize],
    #[doc = "0x1bc - Channel n Interrupt Enable Clear"]
    pub chintenclr23: CHINTENCLR,
    #[doc = "0x1bd - Channel n Interrupt Enable Set"]
    pub chintenset23: CHINTENSET,
    #[doc = "0x1be - Channel n Interrupt Flag Status and Clear"]
    pub chintflag23: CHINTFLAG,
    #[doc = "0x1bf - Channel n Status"]
    pub chstatus23: CHSTATUS,
    #[doc = "0x1c0 - Channel n Control A"]
    pub chctrla24: CHCTRLA,
    #[doc = "0x1c4 - Channel n Control B"]
    pub chctrlb24: CHCTRLB,
    #[doc = "0x1c5 - Channel n Priority Level"]
    pub chprilvl24: CHPRILVL,
    #[doc = "0x1c6 - Channel n Event Control"]
    pub chevctrl24: CHEVCTRL,
    _reserved211: [u8; 5usize],
    #[doc = "0x1cc - Channel n Interrupt Enable Clear"]
    pub chintenclr24: CHINTENCLR,
    #[doc = "0x1cd - Channel n Interrupt Enable Set"]
    pub chintenset24: CHINTENSET,
    #[doc = "0x1ce - Channel n Interrupt Flag Status and Clear"]
    pub chintflag24: CHINTFLAG,
    #[doc = "0x1cf - Channel n Status"]
    pub chstatus24: CHSTATUS,
    #[doc = "0x1d0 - Channel n Control A"]
    pub chctrla25: CHCTRLA,
    #[doc = "0x1d4 - Channel n Control B"]
    pub chctrlb25: CHCTRLB,
    #[doc = "0x1d5 - Channel n Priority Level"]
    pub chprilvl25: CHPRILVL,
    #[doc = "0x1d6 - Channel n Event Control"]
    pub chevctrl25: CHEVCTRL,
    _reserved219: [u8; 5usize],
    #[doc = "0x1dc - Channel n Interrupt Enable Clear"]
    pub chintenclr25: CHINTENCLR,
    #[doc = "0x1dd - Channel n Interrupt Enable Set"]
    pub chintenset25: CHINTENSET,
    #[doc = "0x1de - Channel n Interrupt Flag Status and Clear"]
    pub chintflag25: CHINTFLAG,
    #[doc = "0x1df - Channel n Status"]
    pub chstatus25: CHSTATUS,
    #[doc = "0x1e0 - Channel n Control A"]
    pub chctrla26: CHCTRLA,
    #[doc = "0x1e4 - Channel n Control B"]
    pub chctrlb26: CHCTRLB,
    #[doc = "0x1e5 - Channel n Priority Level"]
    pub chprilvl26: CHPRILVL,
    #[doc = "0x1e6 - Channel n Event Control"]
    pub chevctrl26: CHEVCTRL,
    _reserved227: [u8; 5usize],
    #[doc = "0x1ec - Channel n Interrupt Enable Clear"]
    pub chintenclr26: CHINTENCLR,
    #[doc = "0x1ed - Channel n Interrupt Enable Set"]
    pub chintenset26: CHINTENSET,
    #[doc = "0x1ee - Channel n Interrupt Flag Status and Clear"]
    pub chintflag26: CHINTFLAG,
    #[doc = "0x1ef - Channel n Status"]
    pub chstatus26: CHSTATUS,
    #[doc = "0x1f0 - Channel n Control A"]
    pub chctrla27: CHCTRLA,
    #[doc = "0x1f4 - Channel n Control B"]
    pub chctrlb27: CHCTRLB,
    #[doc = "0x1f5 - Channel n Priority Level"]
    pub chprilvl27: CHPRILVL,
    #[doc = "0x1f6 - Channel n Event Control"]
    pub chevctrl27: CHEVCTRL,
    _reserved235: [u8; 5usize],
    #[doc = "0x1fc - Channel n Interrupt Enable Clear"]
    pub chintenclr27: CHINTENCLR,
    #[doc = "0x1fd - Channel n Interrupt Enable Set"]
    pub chintenset27: CHINTENSET,
    #[doc = "0x1fe - Channel n Interrupt Flag Status and Clear"]
    pub chintflag27: CHINTFLAG,
    #[doc = "0x1ff - Channel n Status"]
    pub chstatus27: CHSTATUS,
    #[doc = "0x200 - Channel n Control A"]
    pub chctrla28: CHCTRLA,
    #[doc = "0x204 - Channel n Control B"]
    pub chctrlb28: CHCTRLB,
    #[doc = "0x205 - Channel n Priority Level"]
    pub chprilvl28: CHPRILVL,
    #[doc = "0x206 - Channel n Event Control"]
    pub chevctrl28: CHEVCTRL,
    _reserved243: [u8; 5usize],
    #[doc = "0x20c - Channel n Interrupt Enable Clear"]
    pub chintenclr28: CHINTENCLR,
    #[doc = "0x20d - Channel n Interrupt Enable Set"]
    pub chintenset28: CHINTENSET,
    #[doc = "0x20e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag28: CHINTFLAG,
    #[doc = "0x20f - Channel n Status"]
    pub chstatus28: CHSTATUS,
    #[doc = "0x210 - Channel n Control A"]
    pub chctrla29: CHCTRLA,
    #[doc = "0x214 - Channel n Control B"]
    pub chctrlb29: CHCTRLB,
    #[doc = "0x215 - Channel n Priority Level"]
    pub chprilvl29: CHPRILVL,
    #[doc = "0x216 - Channel n Event Control"]
    pub chevctrl29: CHEVCTRL,
    _reserved251: [u8; 5usize],
    #[doc = "0x21c - Channel n Interrupt Enable Clear"]
    pub chintenclr29: CHINTENCLR,
    #[doc = "0x21d - Channel n Interrupt Enable Set"]
    pub chintenset29: CHINTENSET,
    #[doc = "0x21e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag29: CHINTFLAG,
    #[doc = "0x21f - Channel n Status"]
    pub chstatus29: CHSTATUS,
    #[doc = "0x220 - Channel n Control A"]
    pub chctrla30: CHCTRLA,
    #[doc = "0x224 - Channel n Control B"]
    pub chctrlb30: CHCTRLB,
    #[doc = "0x225 - Channel n Priority Level"]
    pub chprilvl30: CHPRILVL,
    #[doc = "0x226 - Channel n Event Control"]
    pub chevctrl30: CHEVCTRL,
    _reserved259: [u8; 5usize],
    #[doc = "0x22c - Channel n Interrupt Enable Clear"]
    pub chintenclr30: CHINTENCLR,
    #[doc = "0x22d - Channel n Interrupt Enable Set"]
    pub chintenset30: CHINTENSET,
    #[doc = "0x22e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag30: CHINTFLAG,
    #[doc = "0x22f - Channel n Status"]
    pub chstatus30: CHSTATUS,
    #[doc = "0x230 - Channel n Control A"]
    pub chctrla31: CHCTRLA,
    #[doc = "0x234 - Channel n Control B"]
    pub chctrlb31: CHCTRLB,
    #[doc = "0x235 - Channel n Priority Level"]
    pub chprilvl31: CHPRILVL,
    #[doc = "0x236 - Channel n Event Control"]
    pub chevctrl31: CHEVCTRL,
    _reserved267: [u8; 5usize],
    #[doc = "0x23c - Channel n Interrupt Enable Clear"]
    pub chintenclr31: CHINTENCLR,
    #[doc = "0x23d - Channel n Interrupt Enable Set"]
    pub chintenset31: CHINTENSET,
    #[doc = "0x23e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag31: CHINTFLAG,
    #[doc = "0x23f - Channel n Status"]
    pub chstatus31: CHSTATUS,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcctrl](crcctrl) module"]
pub type CRCCTRL = crate::Reg<u16, _CRCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCTRL;
#[doc = "`read()` method returns [crcctrl::R](crcctrl::R) reader structure"]
impl crate::Readable for CRCCTRL {}
#[doc = "`write(|w| ..)` method takes [crcctrl::W](crcctrl::W) writer structure"]
impl crate::Writable for CRCCTRL {}
#[doc = "CRC Control"]
pub mod crcctrl;
#[doc = "CRC Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatain](crcdatain) module"]
pub type CRCDATAIN = crate::Reg<u32, _CRCDATAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDATAIN;
#[doc = "`read()` method returns [crcdatain::R](crcdatain::R) reader structure"]
impl crate::Readable for CRCDATAIN {}
#[doc = "`write(|w| ..)` method takes [crcdatain::W](crcdatain::W) writer structure"]
impl crate::Writable for CRCDATAIN {}
#[doc = "CRC Data Input"]
pub mod crcdatain;
#[doc = "CRC Checksum\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcchksum](crcchksum) module"]
pub type CRCCHKSUM = crate::Reg<u32, _CRCCHKSUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCCHKSUM;
#[doc = "`read()` method returns [crcchksum::R](crcchksum::R) reader structure"]
impl crate::Readable for CRCCHKSUM {}
#[doc = "`write(|w| ..)` method takes [crcchksum::W](crcchksum::W) writer structure"]
impl crate::Writable for CRCCHKSUM {}
#[doc = "CRC Checksum"]
pub mod crcchksum;
#[doc = "CRC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](crcstatus) module"]
pub type CRCSTATUS = crate::Reg<u8, _CRCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCSTATUS;
#[doc = "`read()` method returns [crcstatus::R](crcstatus::R) reader structure"]
impl crate::Readable for CRCSTATUS {}
#[doc = "`write(|w| ..)` method takes [crcstatus::W](crcstatus::W) writer structure"]
impl crate::Writable for CRCSTATUS {}
#[doc = "CRC Status"]
pub mod crcstatus;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Software Trigger Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigctrl](swtrigctrl) module"]
pub type SWTRIGCTRL = crate::Reg<u32, _SWTRIGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIGCTRL;
#[doc = "`read()` method returns [swtrigctrl::R](swtrigctrl::R) reader structure"]
impl crate::Readable for SWTRIGCTRL {}
#[doc = "`write(|w| ..)` method takes [swtrigctrl::W](swtrigctrl::W) writer structure"]
impl crate::Writable for SWTRIGCTRL {}
#[doc = "Software Trigger Control"]
pub mod swtrigctrl;
#[doc = "Priority Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl0](prictrl0) module"]
pub type PRICTRL0 = crate::Reg<u32, _PRICTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRICTRL0;
#[doc = "`read()` method returns [prictrl0::R](prictrl0::R) reader structure"]
impl crate::Readable for PRICTRL0 {}
#[doc = "`write(|w| ..)` method takes [prictrl0::W](prictrl0::W) writer structure"]
impl crate::Writable for PRICTRL0 {}
#[doc = "Priority Control 0"]
pub mod prictrl0;
#[doc = "Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u16, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "`write(|w| ..)` method takes [intpend::W](intpend::W) writer structure"]
impl crate::Writable for INTPEND {}
#[doc = "Interrupt Pending"]
pub mod intpend;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busych](busych) module"]
pub type BUSYCH = crate::Reg<u32, _BUSYCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSYCH;
#[doc = "`read()` method returns [busych::R](busych::R) reader structure"]
impl crate::Readable for BUSYCH {}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Pending Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pendch](pendch) module"]
pub type PENDCH = crate::Reg<u32, _PENDCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDCH;
#[doc = "`read()` method returns [pendch::R](pendch::R) reader structure"]
impl crate::Readable for PENDCH {}
#[doc = "Pending Channels"]
pub mod pendch;
#[doc = "Active Channel and Levels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](active) module"]
pub type ACTIVE = crate::Reg<u32, _ACTIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTIVE;
#[doc = "`read()` method returns [active::R](active::R) reader structure"]
impl crate::Readable for ACTIVE {}
#[doc = "Active Channel and Levels"]
pub mod active;
#[doc = "Descriptor Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddr](baseaddr) module"]
pub type BASEADDR = crate::Reg<u32, _BASEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASEADDR;
#[doc = "`read()` method returns [baseaddr::R](baseaddr::R) reader structure"]
impl crate::Readable for BASEADDR {}
#[doc = "`write(|w| ..)` method takes [baseaddr::W](baseaddr::W) writer structure"]
impl crate::Writable for BASEADDR {}
#[doc = "Descriptor Memory Section Base Address"]
pub mod baseaddr;
#[doc = "Write-Back Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrbaddr](wrbaddr) module"]
pub type WRBADDR = crate::Reg<u32, _WRBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRBADDR;
#[doc = "`read()` method returns [wrbaddr::R](wrbaddr::R) reader structure"]
impl crate::Readable for WRBADDR {}
#[doc = "`write(|w| ..)` method takes [wrbaddr::W](wrbaddr::W) writer structure"]
impl crate::Writable for WRBADDR {}
#[doc = "Write-Back Memory Section Base Address"]
pub mod wrbaddr;
#[doc = "Channel n Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrla](chctrla) module"]
pub type CHCTRLA = crate::Reg<u32, _CHCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTRLA;
#[doc = "`read()` method returns [chctrla::R](chctrla::R) reader structure"]
impl crate::Readable for CHCTRLA {}
#[doc = "`write(|w| ..)` method takes [chctrla::W](chctrla::W) writer structure"]
impl crate::Writable for CHCTRLA {}
#[doc = "Channel n Control A"]
pub mod chctrla;
#[doc = "Channel n Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrlb](chctrlb) module"]
pub type CHCTRLB = crate::Reg<u8, _CHCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTRLB;
#[doc = "`read()` method returns [chctrlb::R](chctrlb::R) reader structure"]
impl crate::Readable for CHCTRLB {}
#[doc = "`write(|w| ..)` method takes [chctrlb::W](chctrlb::W) writer structure"]
impl crate::Writable for CHCTRLB {}
#[doc = "Channel n Control B"]
pub mod chctrlb;
#[doc = "Channel n Priority Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chprilvl](chprilvl) module"]
pub type CHPRILVL = crate::Reg<u8, _CHPRILVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPRILVL;
#[doc = "`read()` method returns [chprilvl::R](chprilvl::R) reader structure"]
impl crate::Readable for CHPRILVL {}
#[doc = "`write(|w| ..)` method takes [chprilvl::W](chprilvl::W) writer structure"]
impl crate::Writable for CHPRILVL {}
#[doc = "Channel n Priority Level"]
pub mod chprilvl;
#[doc = "Channel n Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chevctrl](chevctrl) module"]
pub type CHEVCTRL = crate::Reg<u8, _CHEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEVCTRL;
#[doc = "`read()` method returns [chevctrl::R](chevctrl::R) reader structure"]
impl crate::Readable for CHEVCTRL {}
#[doc = "`write(|w| ..)` method takes [chevctrl::W](chevctrl::W) writer structure"]
impl crate::Writable for CHEVCTRL {}
#[doc = "Channel n Event Control"]
pub mod chevctrl;
#[doc = "Channel n Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenclr](chintenclr) module"]
pub type CHINTENCLR = crate::Reg<u8, _CHINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTENCLR;
#[doc = "`read()` method returns [chintenclr::R](chintenclr::R) reader structure"]
impl crate::Readable for CHINTENCLR {}
#[doc = "`write(|w| ..)` method takes [chintenclr::W](chintenclr::W) writer structure"]
impl crate::Writable for CHINTENCLR {}
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "Channel n Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintenset](chintenset) module"]
pub type CHINTENSET = crate::Reg<u8, _CHINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTENSET;
#[doc = "`read()` method returns [chintenset::R](chintenset::R) reader structure"]
impl crate::Readable for CHINTENSET {}
#[doc = "`write(|w| ..)` method takes [chintenset::W](chintenset::W) writer structure"]
impl crate::Writable for CHINTENSET {}
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "Channel n Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chintflag](chintflag) module"]
pub type CHINTFLAG = crate::Reg<u8, _CHINTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHINTFLAG;
#[doc = "`read()` method returns [chintflag::R](chintflag::R) reader structure"]
impl crate::Readable for CHINTFLAG {}
#[doc = "`write(|w| ..)` method takes [chintflag::W](chintflag::W) writer structure"]
impl crate::Writable for CHINTFLAG {}
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "Channel n Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chstatus](chstatus) module"]
pub type CHSTATUS = crate::Reg<u8, _CHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSTATUS;
#[doc = "`read()` method returns [chstatus::R](chstatus::R) reader structure"]
impl crate::Readable for CHSTATUS {}
#[doc = "`write(|w| ..)` method takes [chstatus::W](chstatus::W) writer structure"]
impl crate::Writable for CHSTATUS {}
#[doc = "Channel n Status"]
pub mod chstatus;

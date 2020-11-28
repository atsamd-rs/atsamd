#[allow(dead_code)]
use super::erpc::{codec, id, Err};
use generic_array::{ArrayLength, GenericArray};
use heapless::{consts::U18, String};
use nom::{
    error::ParseError, lib::std::ops::RangeFrom, lib::std::ops::RangeTo, number::streaming,
    IResult, InputIter, InputLength, InputTake, Slice,
};

pub struct GetMacAddress {}

impl codec::RPC for GetMacAddress {
    type ReturnValue = String<U18>;
    type Error = i32;

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::Wifi,
            request: id::WifiRequest::GetMacAddress.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::Wifi
            || hdr.request != id::WifiRequest::GetMacAddress.into()
        {
            return Err(Err::NotOurs);
        }

        if data.input_len() < 18 {
            return Err(Err::RPCErr(-1));
        }
        let mut mac: String<U18> = String::new();
        for b in data.slice(RangeTo { end: 17 }).iter_elements() {
            mac.push(b as char).map_err(|_| Err::ResponseOverrun)?;
        }

        let (_, result) = streaming::le_u32(data.slice(RangeFrom { start: 18 }))?;
        if result != 0 {
            Err(Err::RPCErr(result as i32))
        } else {
            Ok(mac)
        }
    }
}

pub struct IsScanning {}

impl codec::RPC for IsScanning {
    type ReturnValue = bool;
    type Error = ();

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::Wifi,
            request: id::WifiRequest::IsScanning.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::Wifi
            || hdr.request != id::WifiRequest::IsScanning.into()
        {
            return Err(Err::NotOurs);
        }

        if data.input_len() < 1 {
            return Err(Err::RPCErr(()));
        }
        Ok(data.iter_elements().nth(0) != Some(0))
    }
}

const SSID_MAX_LENGTH: u8 = 32;
const AP_LIST_MAX_LENGTH: usize = 60;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum BssType {
    Infra = 0,
    Adhoc = 1,
    Any = 2,
    Unknown = core::u32::MAX,
}

const WEP_ENABLED: u32 = 1;
const TKIP_ENABLED: u32 = 2;
const AES_ENABLED: u32 = 4;
const AES_CMAC_ENABLED: u32 = 0x10;
const SHARED_ENABLED: u32 = 0x00008000;
const WPA_SECURITY: u32 = 0x00200000;
const WPA2_SECURITY: u32 = 0x00400000;
const WPA3_SECURITY: u32 = 0x00800000;
const WPS_ENABLED: u32 = 0x10000000;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Security {
    Open = 0,
    WepPsk = WEP_ENABLED,
    WepShared = WEP_ENABLED | SHARED_ENABLED,
    WpaTkipPsk = WPA_SECURITY | TKIP_ENABLED,
    WpaAesPsk = WPA_SECURITY | AES_ENABLED,
    Wpa2AesPsk = WPA2_SECURITY | AES_ENABLED,
    Wpa2TkipPsk = WPA2_SECURITY | TKIP_ENABLED,
    Wpa2MixedPsk = WPA2_SECURITY | AES_ENABLED | TKIP_ENABLED,
    WpaWpa2Mixed = WPA_SECURITY | WPA2_SECURITY,
    Wpa2AesCmac = WPA2_SECURITY | AES_CMAC_ENABLED,
    WpsOpen = WPS_ENABLED,
    WpsSecure = WPS_ENABLED | AES_ENABLED,
    Wps3AesPsk = WPA3_SECURITY | AES_ENABLED,
    Unknown = core::u32::MAX,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WPS {
    Default = 0x0000,
    UserSpecifed = 0x0001,
    MachineSpecified = 0x0002,
    Rekey = 0x0003,
    Pushbutton = 0x0004,
    RegistrarSpecified = 0x0005,
    None = 0x0006,
    Wsc = 0x0007,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Band {
    _5Ghz = 0,
    _24Ghz = 1,
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct BSSID(pub [u8; 6]);

impl core::fmt::Debug for BSSID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let table = b"0123456789abcdef";

        let mut out = [0u8; 12 + 6];
        for i in 0..(12 + 6) {
            let b = self.0[i / 3];
            out[i] = match (i + 1) % 3 {
                0 => ':' as u8,
                1 => table[(b >> 4) as usize],
                2 => table[(b & 0xf) as usize],
                _ => '?' as u8,
            }
        }

        f.write_str(core::str::from_utf8(&out).unwrap())
    }
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct SSID {
    len: u8,
    value: [u8; 33],
}

impl core::fmt::Debug for SSID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.value[..self.len as usize]).unwrap())
    }
}

impl SSID {
    fn parse<I, E: ParseError<I>>(i: I) -> IResult<I, Self, E>
    where
        I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength + InputTake,
    {
        let (data, length) = streaming::le_u8(i)?;
        let mut out = Self {
            len: length,
            value: [0u8; 33],
        };
        if data.input_len() < length as usize {
            return Err(nom::Err::Incomplete(nom::Needed::new(
                length as usize - data.input_len(),
            )));
        };
        for (i, b) in data.take(length as usize).iter_elements().enumerate() {
            out.value[i] = b;
        }

        Ok((data.slice(length as usize..), out))
    }
}

impl<N> Into<String<N>> for SSID
where
    N: heapless::ArrayLength<u8>,
{
    fn into(self) -> String<N> {
        let mut out = String::new();
        for i in 0..self.len as usize {
            out.push(self.value[i] as char);
        }
        out
    }
}

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct ScanResult {
    /// Service Set Identification (i.e. Name of Access Point)
    pub ssid: SSID,
    /// Basic Service Set Identification (i.e. MAC address of Access Point)
    pub bssid: BSSID,
    /// Receive Signal Strength Indication in dBm. <-90=poor, >-30=Excellent
    pub rssi: i16,
    /// Network type
    pub bss_type: BssType,
    /// Security type
    pub security: Security,
    /// WPS type
    pub wps: WPS,
    /// Channel
    pub chan: u32,
    /// Radio channel that the AP beacon was received on
    pub band: Band,
}

impl core::fmt::Debug for ScanResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.ssid.len > 0 {
            f.debug_struct("ScanResult")
                .field("ssid", &self.ssid)
                .field("bssid", &self.bssid)
                .field("rssi", &self.rssi)
                .field("type", &self.bss_type)
                .field("security", &self.security)
                .field("wps", &self.wps)
                .field("channel", &self.chan)
                .field("band", &self.band)
                .finish()
        } else {
            f.debug_struct("ScanResult")
                .field("bssid", &self.bssid)
                .field("rssi", &self.rssi)
                .field("type", &self.bss_type)
                .field("security", &self.security)
                .field("wps", &self.wps)
                .field("channel", &self.chan)
                .field("band", &self.band)
                .finish()
        }
    }
}

impl Default for ScanResult {
    fn default() -> Self {
        Self {
            ssid: SSID {
                len: 0,
                value: [0u8; 33],
            },
            bssid: BSSID([0u8; 6]),
            rssi: 0,
            bss_type: BssType::Any,
            security: Security::Open,
            wps: WPS::Default,
            chan: 0,
            band: Band::_24Ghz,
        }
    }
}

pub struct ScanGetAP<N: ArrayLength<ScanResult>> {
    m: core::marker::PhantomData<N>,
}

impl<N: ArrayLength<ScanResult>> ScanGetAP<N> {
    pub fn new() -> Self {
        Self {
            m: core::marker::PhantomData,
        }
    }
}

impl<N: ArrayLength<ScanResult>> codec::RPC for ScanGetAP<N> {
    type ReturnValue = (GenericArray<ScanResult, N>, i32);
    type Error = usize;

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::Wifi,
            request: id::WifiRequest::ScanGetAP.into(),
        }
    }

    fn args(&self, buff: &mut heapless::Vec<u8, heapless::consts::U64>) {
        let num = N::to_u16().to_le_bytes();
        buff.extend_from_slice(&num).ok();
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::Wifi
            || hdr.request != id::WifiRequest::ScanGetAP.into()
        {
            return Err(Err::NotOurs);
        }

        let (data, l) = streaming::le_u32(data)?; // Binary len - returning 62 bytes per result
        if l as usize != (core::mem::size_of::<ScanResult>() * N::to_usize()) {
            return Err(Err::ResponseOverrun);
        }

        let mut res = GenericArray::<ScanResult, N>::default();

        //let (data, ssid) = SSID::parse(data)?;
        for i in 0..N::to_usize() {
            res[i] = unsafe { *((data.as_ptr() as *const ScanResult).offset(i as isize)) };
        }

        //let s: ScanResult = unsafe { core::mem::transmute::<[u8;
        // core::mem::size_of::<ScanResult>()], ScanResult>(b) };

        // let out = ScanResult{
        //     ssid: ssid,
        //     bssid: [0u8; 6],
        //     signal_strength: 0,
        //     bss_type: BssType::Infra,
        //     security: Security::Open,
        //     wps_type: WPS::Default,
        //     channel: 0,
        //     band: Band::_5Ghz,
        // };

        let (_, ret_val) = streaming::le_i32(data.slice(core::mem::size_of::<ScanResult>()..))?;
        Ok((res, ret_val))
    }
}

pub struct ScanGetNumAPs {}

impl codec::RPC for ScanGetNumAPs {
    type ReturnValue = u16;
    type Error = ();

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::Wifi,
            request: id::WifiRequest::ScanGetNumAPs.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::Wifi
            || hdr.request != id::WifiRequest::ScanGetNumAPs.into()
        {
            return Err(Err::NotOurs);
        }

        if data.input_len() < 2 {
            return Err(Err::RPCErr(()));
        }
        let (_, num) = streaming::le_u16(data)?;
        Ok(num)
    }
}

pub struct ScanStart {}

impl codec::RPC for ScanStart {
    type ReturnValue = i32;
    type Error = ();

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::Wifi,
            request: id::WifiRequest::ScanStart.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::Wifi
            || hdr.request != id::WifiRequest::ScanStart.into()
        {
            return Err(Err::NotOurs);
        }

        let (_, num) = streaming::le_i32(data)?;
        Ok(num)
    }
}

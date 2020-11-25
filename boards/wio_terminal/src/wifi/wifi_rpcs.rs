use super::erpc::{codec, id, Err};
use heapless::{consts::U18, String};
use nom::number::streaming;

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
        let (data, _) = codec::Header::parse::<()>(data)?; // TODO: Check RPC header values
        if data.len() < 18 {
            return Err(Err::RPCErr(-1));
        }
        let mut mac: String<U18> = String::new();
        for b in &data[..17] {
            mac.push(*b as char);
        }

        let (_, result) = streaming::le_u32::<()>(&data[18..])?;
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
            request: id::WifiRequest::GetMacAddress.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, Err<Self::Error>> {
        let (data, _) = codec::Header::parse::<()>(data)?; // TODO: Check RPC header values
        if data.len() < 1 {
            return Err(Err::RPCErr(()));
        }
        Ok(data[0] != 0)
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
        let (data, _) = codec::Header::parse::<()>(data)?; // TODO: Check RPC header values
        if data.len() < 2 {
            return Err(Err::RPCErr(()));
        }
        let (_, num) = streaming::le_u16(data)?;
        Ok(num)
    }
}

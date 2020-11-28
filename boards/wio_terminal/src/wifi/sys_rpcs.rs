use super::erpc::{codec, id, Err};
use heapless::{consts::U16, String};
use nom::{
    lib::std::ops::RangeFrom, lib::std::ops::RangeTo, number::streaming, InputIter, InputLength,
    InputTake, Slice,
};

pub struct GetVersion {}

impl codec::RPC for GetVersion {
    type ReturnValue = String<U16>;
    type Error = ();

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::System,
            request: id::SystemRequest::VersionID.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<String<U16>, Err<()>> {
        let (data, hdr) = codec::Header::parse(data)?;
        if hdr.msg_type != id::MsgType::Reply
            || hdr.service != id::Service::System
            || hdr.request != id::SystemRequest::VersionID.into()
        {
            return Err(Err::NotOurs);
        }

        let (data, length) = streaming::le_u32(data)?;
        if length > 16 {
            return Err(Err::ResponseOverrun);
        }

        let mut out: Self::ReturnValue = String::new();
        for b in data.iter_elements() {
            out.push(b as char).ok();
        }
        Ok(out)
    }
}

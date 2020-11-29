pub mod id {
    /// eRPC request type
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[allow(unused)]
    pub enum MsgType {
        Invocation = 0,
        Oneway = 1,
        Reply = 2,
        Notification = 3,
        Unknown = 255,
    }

    impl From<u8> for MsgType {
        fn from(mt: u8) -> MsgType {
            match mt {
                0 => MsgType::Invocation,
                1 => MsgType::Oneway,
                2 => MsgType::Reply,
                3 => MsgType::Notification,
                _ => MsgType::Unknown,
            }
        }
    }

    /// Wio Terminal services
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[allow(unused)]
    pub enum Service {
        System = 1,
        BLEHost = 2,
        BLEGap = 3,
        BLEGapBone = 4,
        BLECallback = 13,
        Wifi = 14,
        WifiCallback = 18,
        Unknown = 255,
    }

    impl From<u8> for Service {
        fn from(mt: u8) -> Service {
            match mt {
                1 => Service::System,
                2 => Service::BLEHost,
                3 => Service::BLEGap,
                4 => Service::BLEGapBone,
                13 => Service::BLECallback,
                14 => Service::Wifi,
                18 => Service::WifiCallback,
                _ => Service::Unknown,
            }
        }
    }

    /// Wio Terminal request IDs for the System service
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[allow(unused)]
    pub enum SystemRequest {
        VersionID = 1,
        AckID = 2,
    }

    impl From<SystemRequest> for u8 {
        fn from(r: SystemRequest) -> u8 {
            r as u8
        }
    }

    /// Wio Terminal request IDs for the Wifi service
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[allow(unused)]
    pub enum WifiRequest {
        GetMacAddress = 8,
        ScanStart = 64,
        IsScanning = 65,
        ScanGetAP = 66,
        ScanGetNumAPs = 67,
    }

    impl From<WifiRequest> for u8 {
        fn from(r: WifiRequest) -> u8 {
            r as u8
        }
    }
}

pub mod codec {
    use super::id::*;
    use nom::{
        error::ParseError, lib::std::ops::RangeFrom, lib::std::ops::RangeTo, number::streaming,
        IResult, InputIter, InputLength, InputTake, Slice,
    };

    const BASIC_CODEC_VERSION: u8 = 1;

    /// RPC header
    #[derive(Clone, Debug)]
    pub struct Header {
        pub service: Service,
        pub request: u8,
        pub msg_type: MsgType,
        pub sequence: u32, // incrementing number.
    }

    impl Header {
        pub fn as_bytes(&self) -> [u8; 8] {
            let header: u32 = (BASIC_CODEC_VERSION as u32) << 24
                | ((self.service as u32) << 16)
                | ((self.request as u32) << 8)
                | (self.msg_type as u32);
            let header = header.to_le_bytes();

            let seq = self.sequence.to_le_bytes();

            [
                header[0], header[1], header[2], header[3], seq[0], seq[1], seq[2], seq[3],
            ]
        }

        pub fn parse<I, E: ParseError<I>>(i: I) -> IResult<I, Self, E>
        where
            I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
        {
            let (i, header) = streaming::le_u32(i)?;
            let (i, sequence) = streaming::le_u32(i)?;
            Ok((
                i,
                Self {
                    service: (((header >> 16) & 0xff) as u8).into(),
                    request: ((header >> 8) & 0xff) as u8,
                    msg_type: ((header & 0xff) as u8).into(),
                    sequence,
                },
            ))
        }
    }

    /// Frame preamble
    #[derive(Clone, Debug)]
    pub struct FramePreamble {
        pub msg_length: u16,
        pub crc16: u16,
    }

    impl FramePreamble {
        pub fn new_from_msg(msg: &[u8]) -> Self {
            Self {
                msg_length: msg.len() as u16,
                crc16: crc16(msg),
            }
        }

        pub fn as_bytes(&self) -> [u8; 4] {
            let (l, c) = (self.msg_length.to_le_bytes(), self.crc16.to_le_bytes());
            [l[0], l[1], c[0], c[1]]
        }

        pub fn parse<I, E: ParseError<I>>(i: I) -> IResult<I, Self, E>
        where
            I: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
        {
            let (i, msg_length) = streaming::le_u16(i)?;
            let (i, crc16) = streaming::le_u16(i)?;
            Ok((i, Self { msg_length, crc16 }))
        }
    }

    /// computes the CRC value used in the Wio Terminal eRPC codec
    pub(crate) fn crc16<I>(data: I) -> u16
    where
        I: InputIter<Item = u8>,
    {
        let mut crc: u32 = 0xEF4A;

        for b in data.iter_elements() {
            crc ^= (b as u32) << 8;
            for _ in 0..8 {
                let mut temp: u32 = crc << 1;
                if (crc & 0x8000) != 0 {
                    temp ^= 0x1021;
                }
                crc = temp;
            }
        }

        crc as u16
    }

    pub trait RPC {
        type ReturnValue;
        type Error;

        fn header(&self, seq: u32) -> Header;
        fn args(&self, _buff: &mut heapless::Vec<u8, heapless::consts::U64>) {}

        fn parse(&mut self, data: &[u8]) -> Result<Self::ReturnValue, super::Err<Self::Error>>;
    }
}

/// Encapsulates errors that might occur when issuing or processing eRPCs.
#[derive(Debug, Clone, PartialEq)]
pub enum Err<E> {
    /// Parsing via the nom crate indicated an error
    Parsing(nom::Err<()>),
    /// The CRC was wrong
    CRCMismatch,
    /// There was an issue while transmitting
    TXErr,
    /// The response we were given to parse was for a different (callback,
    /// probably) RPC.
    NotOurs,
    /// There was an RPC-specific error.
    RPCErr(E),
    /// Too much data was present in the response
    ResponseOverrun,
    Unknown,
}

impl<E> From<nom::Err<()>> for Err<E> {
    fn from(nom_err: nom::Err<()>) -> Self {
        Err::Parsing::<E>(nom_err)
    }
}
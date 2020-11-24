pub mod id {
    /// eRPC request type
    #[derive(Debug, Copy, Clone)]
    #[allow(unused)]
    #[repr(u8)]
    pub enum MsgType {
        Invocation = 0,
        Oneway = 1,
        Reply = 2,
        Notification = 3,
    }

    /// Wio Terminal services
    #[derive(Debug, Copy, Clone)]
    #[allow(unused)]
    #[repr(u8)]
    pub enum Service {
        System = 1,
    }

    /// Wio Terminal request IDs for the System service
    #[derive(Debug, Copy, Clone)]
    #[allow(unused)]
    #[repr(u8)]
    pub enum SystemRequest {
        VersionID = 1,
        AckID = 2,
    }

    impl From<SystemRequest> for u8 {
        fn from(r: SystemRequest) -> u8 {
            r as u8
        }
    }
}

pub mod codec {
    use super::id::*;
    const BASIC_CODEC_VERSION: u8 = 1;

    /// RPC header
    pub struct Header {
        pub service: Service,
        pub request: u8,
        pub msg_type: MsgType,
        pub sequence: u32, // incrementing number.
    }

    impl Header {
        pub fn as_bytes(&self) -> [u8; 8] {
            let header: u32 = (((BASIC_CODEC_VERSION as u32) << 24)
                | ((self.service as u32) << 16)
                | ((self.request as u32) << 8)
                | (self.msg_type as u32));
            let header = header.to_le_bytes();

            let seq = self.sequence.to_le_bytes();

            [
                header[0], header[1], header[2], header[3], seq[0], seq[1], seq[2], seq[3],
            ]
        }
    }

    /// computes the CRC value used in the Wio Terminal eRPC codec
    pub fn crc16(data: &[u8]) -> u16 {
        let mut crc: u32 = 0xEF4A;

        for (j, b) in data.iter().enumerate() {
            crc ^= (*b as u32) << 8;
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
}

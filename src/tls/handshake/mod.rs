use super::util::*;
use super::tls::TLSFlagment;
mod clienthello;
mod ciphersuite;
#[repr(u8)]
enum HandshakeType {
    HelloRequest = 0x00,
    ClientHello = 0x01,
    ServerHello = 0x02,
    Certificate = 0x0b,
    ServerKeyExchange = 0x0c,
    CertificateRequest = 0x0d,
    ServerHelloDone = 0x0e,
    CerficiateVerify = 0x0f,
    ClientKeyExchange = 0x10,
    Finished = 0x14,
}
#[derive(Copy,Clone, Debug)]
pub enum HandshakeData {
    UnHandled,
    ClientHello{gmt_unix_time : u32, random_bytes : [u8; 28]},
}
impl From<HandshakeData> for u8 {
    fn from(x: HandshakeData) -> u8 {
        match x {
            HandshakeData::ClientHello{..} => HandshakeType::ClientHello as u8,
            _ => 0xff,
        }
    }
}
impl Into<Vec<u8>> for HandshakeData {
    fn into(self) -> Vec<u8> {
        match self {
            HandshakeData::ClientHello{gmt_unix_time,random_bytes} => {
                let mut res = u32_to_bytearray(gmt_unix_time);
                res.extend_from_slice(&random_bytes);
                res
            },
            _ => vec![]
        }
    }
}
#[repr(packed)]
#[derive(Debug)]
pub struct HandshakeChunk {
    //0 type
    //1~3 length (high, middle, low)
    //4~ body
    msg_type : u8,
    length : u24,
    data : HandshakeData
}
impl HandshakeChunk{
    pub fn new(data : HandshakeData) -> HandshakeChunk{
        HandshakeChunk{
            msg_type: data.into(),
            length: 32u32.into(),
            data: data,
        }
    }
    #[cfg(test)]
    pub fn gettype(self) -> u8 {
        self.msg_type
    }
}
impl TLSFlagment for HandshakeChunk {
    const TYPE: u8 = 0x16;
    fn get(self) -> Vec<u8> {
        let mut res :Vec<u8> = vec![];
        let msgtype : u8 = self.msg_type;
        res.push(msgtype);
        let length : Vec<u8> = self.length.into();
        res.extend(length);
        let data : Vec<u8> = self.data.into();
        res.extend(data);
        res
    }
}
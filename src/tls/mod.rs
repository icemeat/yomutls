
#[cfg(test)]
mod test;
mod common;
mod util;
mod handshake;
mod tls {
    use super::util::u24;

    pub trait TLSFlagment {
        const TYPE: u8;
        fn get(self) -> Vec<u8>;
    }
    struct ChangeCipherSpecChunk {
        //0 type
        msg_type : u8,
    }
    impl TLSFlagment for ChangeCipherSpecChunk {
        const TYPE: u8 = 0x14;
        fn get(self) -> Vec<u8> {
            vec![]
        }
    }
    #[derive(Debug)]
    enum AlertLevel {
        Warning = 0x01,
        Fatal = 0x02,
    }
    #[derive(Debug)]
    enum AlertDescription {
        CloseNotify = 0x00,
        UnexpectedMessage = 0x0a,
        BadrecordMAC = 0x14,
        DescryptionFailed = 0x15,
        RecordOverflow = 0x16,
        DecompressionFailure = 0x1e,
        HandshakeFailure = 0x28,
        BadCertificate = 0x2a,
        UnsupportedCertificate = 0x2b,
        CertificateRevoked = 0x2c,
        CertificateExpired = 0x2d,
        CertificateUnknown = 0x2e,
        IllegalParameter = 0x2f,
        UnknownCA = 0x30,
        AccessDenied = 0x31,
        DecodeError = 0x32,
        DecryptError = 0x33,
        ExportRestriction = 0x3c,
        ProtocolVersion = 0x46,
        InsufficientSecurity = 0x47,
        InternalErro = 0x50,
        UserCanceled = 0x5a,
        NoRenegotiation = 0x64,
    }
    struct AlertChunk{
        //0 level
        //1 description
        alert_level : AlertLevel,
        alert_desc : AlertDescription,
    }
    impl TLSFlagment for AlertChunk {
        const TYPE: u8 = 0x15;
        fn get(self) -> Vec<u8> {
            vec![]
        }
    }

    struct TLSPlaintext<Chunk>
    where Chunk: TLSFlagment{
        //0 type
        //1~2 version (major, minor)
        //3~4 length (high, low)
        //5~ fragment
        chunktype : u8,
        version: [u8; 2],
        length : [u8; 3],
        chunk: Chunk,
    }
    #[derive(Debug)]
    enum TLSCompressType {
        None
    }
    struct TLSCompressed{
        //0 type
        //1~2 version (major, minor)
        //3~4 length (high, low)
        //5~ fragment
        comptype : TLSCompressType ,
        data : Vec<u8>,
    }
    impl TLSCompressed{
        fn new<Chunk>(comptype :TLSCompressType, _plane_data : &Chunk)->TLSCompressed
        where Chunk: TLSFlagment{
            //TODO: 圧縮
            //
            TLSCompressed{comptype: comptype, data: vec![]}
        }
    }
    struct TLS_MAC{

    }
    struct TLSCiphertext<Chunk>
    where Chunk: TLSFlagment{
        data : Chunk,
        padding : Vec<u8>,
        mac : TLS_MAC,
    }
}
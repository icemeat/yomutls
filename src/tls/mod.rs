mod tls {
    trait TLSFlagment {
        const TYPE: u8;
        fn get(&self) -> Vec<u8>;
    }
    const hello_request: u8 = 0x00;
    const client_hello: u8 = 0x01;
    const server_hello: u8 = 0x02;
    const certificate: u8 = 0x0b;
    const server_key_exchange: u8 = 0x0c;
    const certificate_request: u8 = 0x0d;
    const server_hello_done: u8 = 0x0e;
    const certificate_verify: u8 = 0x0f;
    const client_key_exchange: u8 = 0x10;
    const finished: u8 = 0x14;

    #[repr(packed)]
    struct HandshakeChunk {
        //0 type
        //1~3 length (high, middle, low)
        //4~ body
        msg_type : u8,
        length : [u8; 3],
        body : Vec<u8>,
    }
    impl TLSFlagment for HandshakeChunk {
        const TYPE: u8 = 0x16;
        fn get(&self) -> Vec<u8> {
            vec![]
        }
    }

    const change_chipher_spec: u8= 0x01;
    struct ChangeCipherSpecChunk {
        //0 type
        msg_type : u8,
    }
    impl TLSFlagment for ChangeCipherSpecChunk {
        const TYPE: u8 = 0x14;
        fn get(&self) -> Vec<u8> {
            vec![]
        }
    }
    #[derive(Debug)]
    enum AlertLevel {
        warning = 0x01,
        fatal = 0x02,
    }
    #[derive(Debug)]
    enum AlertDescription {
        close_notify = 0x00,
        unexpected_message = 0x0a,
        bad_record_mac = 0x14,
        descryption_failed = 0x15,
        record_overflow = 0x16,
        decompression_failure = 0x1e,
        handshake_failure = 0x28,
        bad_certificate = 0x2a,
        unsupported_certificate = 0x2b,
        certificate_revoked = 0x2c,
        certificate_expired = 0x2d,
        certificate_unknown = 0x2e,
        illedgal_parameter = 0x2f,
        unknown_ca = 0x30,
        access_denied = 0x31,
        decode_error = 0x32,
        decrypt_error = 0x33,
        export_restriction = 0x3c,
        protocol_version = 0x46,
        insufficient_security = 0x47,
        internal_error = 0x50,
        user_canceled = 0x5a,
        no_renegotiation = 0x64,
    }
    struct AlertChunk{
        //0 level
        //1 description
        alert_level : AlertLevel,
        alert_desc : AlertDescription,
    }
    impl TLSFlagment for AlertChunk {
        const TYPE: u8 = 0x15;
        fn get(&self) -> Vec<u8> {
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
        fn new<Chunk>(comptype :TLSCompressType, plane_data : &Chunk)->TLSCompressed
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
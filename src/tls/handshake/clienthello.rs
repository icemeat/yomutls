mod clienthello{
    use tls::handshake::ciphersuite::CipherSuite;
    use tls::common::ProtocolVersion;
    enum CompressionMethod {
        null=0,
        _255=255,
    }
    struct Random{
        gmt_unix_time : u32,
        random_bytes : [u8; 28],
    }
    struct ClientHello{
        version : ProtocolVersion,
        random : Random,
        sessionID : Vec<u8>,
        cipherSuite : Vec<CipherSuite>,
        extention : Vec<u8>,
    }
}
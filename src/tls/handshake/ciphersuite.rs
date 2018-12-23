pub type CipherSuite = [u8;2];

const TLS_NULL_WITH_NULL_NULL :CipherSuite= [0x00, 0x00];
const TLS_RSA_WITH_NULL_SHA :CipherSuite= [0x00, 0x02];
const TLS_RSA_WITH_AES_128_CBC_SHA :CipherSuite = [0x00, 0x2F];
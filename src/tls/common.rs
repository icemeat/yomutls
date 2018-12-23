
enum ContentType {
    ChangeChipherSpec=20,
    Alert=21,
    Handshake=22,
    ApplicationData=23,
    _255=255,
}
pub struct ProtocolVersion {
    major: u8,
    minor: u8
}
const TLS_1_2 :ProtocolVersion = ProtocolVersion{major: 3, minor:3};



#[derive(Debug)]
#[repr(packed)]
#[allow(non_camel_case_types)]
pub struct u24 {
    data : [u8; 3]
}
impl From<u32> for u24 {
    fn from(x: u32) -> u24 {
        u24{data:[
            ((x>>16)&0xff) as u8,
            ((x>>8)&0xff) as u8,
            (x&0xff) as u8
            ]}
    }
}
impl Into<u32> for u24 {
    fn into(self) -> u32 {
        ((self.data[0] as u32)<<16 | (self.data[1] as u32)<<8 | (self.data[0] as u32)) as u32
    }
}
impl Into<[u8; 3]> for u24 {
    fn into(self) -> [u8; 3] {
        self.data
    }
}

impl Into<Vec<u8>> for u24 {
    fn into(self) -> Vec<u8> {
        vec![self.data[0],self.data[1],self.data[2]]
    }
}
pub fn u32_to_bytearray(x: u32) -> Vec<u8>{
    vec![
        ((x>>24)&0xff) as u8,
        ((x>>16)&0xff) as u8,
        ((x>>08)&0xff) as u8,
        ((x>>00)&0xff) as u8,
    ]
}

#[cfg(test)]
mod test{
    use tls::util::u24;
    #[test]
    fn test_u32(){
        let testcase :u32 = 0xff00ff00;
        let testtarget = u24::from(0xff00ff00);
        let testres :u32 = testtarget.into();
        assert_eq!(testres,0x0000ff00);
        assert_ne!(testres,testcase);
    }
    #[test]
    fn test_u8_slice(){
        let testcase = [0x00u8, 0xffu8, 0x00u8];
        let testtarget = u24::from(0xff00ff00);
        let testres :[u8; 3] = testtarget.into();
        assert_eq!(testres,testcase);
    }
}

#[cfg(test)]
mod test{
    use tls::tls::TLSFlagment;
    use tls::util::u24;
    use tls::handshake;
    use tls::handshake::{HandshakeChunk,HandshakeData};
    #[test]
    fn test_handshake(){
        let testcase = HandshakeChunk::new(
                HandshakeData::ClientHello{
                    gmt_unix_time: 32u32,
                    random_bytes: [0; 28]
                }
            );
        println!("{:?}",testcase);
        assert_eq!(testcase.gettype(),handshake::CLIENT_HELLO);
    }
    
    use std::net::{ToSocketAddrs, SocketAddr,TcpStream};
    use std::time::{Duration,SystemTime};
    use std::io::prelude::*;
    use std::io;
    #[test]
    fn googleapi_test(){
        
        let testcase = HandshakeChunk::new(
                HandshakeData::ClientHello{
                    gmt_unix_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32,
                    random_bytes: [0; 28]
                }
            );
        println!("{:?}",&testcase);
        let mut url = "www.googleapis.com:443".to_socket_addrs().unwrap();
        let timeout = Duration::new(5u64,0u32);
        match TcpStream::connect_timeout(&(url.next().unwrap()), timeout){
            Ok(stream)=>{
                println!("ok! {:?}",stream);
                let mut wstream = io::BufWriter::new(&stream);
                let mut rstream = io::BufReader::new(&stream);
                //write_post(stream.get_mut());
                println!("writeSize : {:?}",wstream.write(&testcase.get()).unwrap());
                let mut buf :Vec<u8> = vec![];
                loop {
                    let cnt = rstream.read_to_end(&mut buf).unwrap();
                    if cnt>0 {
                        println!("cnt!!{}",cnt);
                        let strBuf = std::str::from_utf8(buf.as_slice()).unwrap();
                        println!("{}",strBuf);
                    }else {
                        panic!("cnt==0");
                    }
                }
                
            }
            Err(e)=>{
                panic!(e);
            }
        }
    }
}
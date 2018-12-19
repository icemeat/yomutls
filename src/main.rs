use std::net::{ToSocketAddrs, SocketAddr,TcpStream};
use std::time::Duration;
use std::io::prelude::*;
use std::io;
mod tls;
fn write_post(stream :&mut TcpStream){
    let post =
b"POST /upload/storage/v1/b/gahu-4a8eb.appspot.com/o?uploadType=media&name=abc HTTP/1.1
Host: www.googleapis.com
Content-length: 1
Content-type: text/plain
Authorization: Bearer ya29.Glt1BtIh6xX_PBVv7kh4W2u4kkpQSy5f5wKhu66NaesS_mlYey311xzaMEoVh9keaYix8iCR3iVD0ESKo7cmFnZH4COjE6PgYJ2-or8Nb4d8tG7qpSrCweocMVNZ

a";
    stream.write(
        post
    ).unwrap();
}
fn ping(stream :&mut TcpStream){
    let post =
"GET /Android/ HTTP/1.1
host: localhost
Content-Type: text/plain
Content-Length: 3
User-Agent: icemeat
Authorization: Bearer ya29.Glt1Bh2HKAWFN7Fu8dt6eQrF64TFAnzLm6pMtmjcmwg2lWi5w8j-TgGi0dROrXssN49tWdkU_qBJiMpEjLwJi8Uu0Z8RlyWqsaLFUDzQOc_JGzrUjdOMZ5rijBfG

qweqwe";
    stream.write(
        post.as_bytes()
    ).unwrap();
}
fn main() {
    //let mut url = "localhost:3594".to_socket_addrs().unwrap();
    let mut url = "www.googleapis.com:443".to_socket_addrs().unwrap();
    let timeout = Duration::new(5u64,0u32);
    match TcpStream::connect_timeout(&(url.next().unwrap()), timeout){
        Ok(stream)=>{
// POST /upload/storage/v1/b/gahu-4a8eb/o?uploadType=media&name=myObject HTTP/1.1
// Content-Type: image/jpeg
// Content-Length: [NUMBER_OF_BYTES_IN_FILE]
// Authorization: Bearer [YOUR_AUTH_TOKEN]
//
// [JPEG_DATA]
            println!("ok! {:?}",stream);
            let mut stream = io::BufReader::new(stream);
            //write_post(stream.get_mut());
            //write!(stream.get_mut(),"ClientHello");
            let mut buf :Vec<u8> = vec![];
            loop {
                let cnt = stream.read_to_end(&mut buf).unwrap();
                if cnt>0 {
                    println!("cnt!!{}",cnt);
                    let strBuf = std::str::from_utf8(buf.as_slice()).unwrap();
                    println!("{}",strBuf);
                }
            }
            
        }
        Err(e)=>{
            panic!(e);
        }
    }

}

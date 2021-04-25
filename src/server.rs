use std::{io, mem, net::SocketAddr, thread};

use httparse::{Request, Status};
use io::ErrorKind;
use tokio::net::{TcpListener, TcpStream};

use crate::utils::{RetUnit, MAX_HEADER_SIZE};

// parse_request -> process_request -> write_response
pub async fn handle_connection(stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
    // println!(
    //     "[{:?}] got connection from: {}",
    //     thread::current().id(),
    //     addr
    // );

    let mut total = 0usize;
    let mut data = Vec::new();
    let mut buff: [u8; 2048] = unsafe { mem::MaybeUninit::uninit().assume_init() };

    loop {
        stream.readable().await?;

        let n = match stream.try_read(&mut buff) {
            Ok(0) => break,
            Ok(n) => n,
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => continue,
            Err(err) => return Err(err),
        };

        total += n;

        data.extend_from_slice(unsafe { buff.get_unchecked(..n) });

        // println!(
        //     "[{:?}] read {} bytes from: {}",
        //     thread::current().id(),
        //     n,
        //     addr
        // );

        let mut headers: [httparse::Header; MAX_HEADER_SIZE] =
            unsafe { mem::MaybeUninit::uninit().assume_init() };

        let mut request = Request::new(&mut headers);
        match request.parse(&buff) {
            Ok(Status::Partial) => continue,
            Ok(Status::Complete(size)) => {
                // println!("parsed: {} bytes", size);
                // print_request(&request);
                write_ok(&stream).await?;
                break;
            }
            Err(err) => {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    format!("parse error: {}", err.to_string()),
                ))
            }
        }
    }

    Ok(())
}

pub fn print_request(req: &Request) {
    println!("{:?}", req);
}

pub async fn write_ok(stream: &TcpStream) -> io::Result<()> {
    const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n";
    stream.writable().await?;

    stream.try_write(OK_RESPONSE.as_bytes())?;

    Ok(())
}

pub async fn handle_accept_error(err: io::Error) {}

pub async fn start_server() -> io::Result<()> {
    let addr = "0.0.0.0:8000";
    let server = TcpListener::bind(addr)
        .await
        .expect(&format!("Can't bind server address to {}", addr));

    loop {
        match server.accept().await {
            Ok((stream, addr)) => tokio::spawn(handle_connection(stream, addr)).ret_unit(),
            Err(err) => tokio::spawn(handle_accept_error(err)).ret_unit(),
        }
    }
}

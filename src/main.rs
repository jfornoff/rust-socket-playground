extern crate nix;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use nix::sys::socket;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("NEW CONNECTION");
                output_info(stream)
            }
            Err(_e) => println!("We dun goofd :-("),
        }
    }
}

fn output_info(stream: TcpStream) {
    println!(
        "### Local address:\n {:?}",
        stream.local_addr().expect(
            "Didn't get the local address :-(",
        )
    );

    println!(
        "### Remote address:\n {:?}",
        stream.peer_addr().expect(
            "Didn't get the remote address :-(",
        )
    );

    println!("File descriptor {:?}", stream.as_raw_fd());
    println!(
        "### Send buffer size:\n {:?}",
        socket::getsockopt(stream.as_raw_fd(), socket::sockopt::SndBuf).unwrap()
    );

    println!(
        "### TCP Info:\n {:#?}",
        socket::getsockopt(stream.as_raw_fd(), socket::sockopt::TcpInfo).unwrap()
    );
}

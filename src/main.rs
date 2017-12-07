extern crate nix;
extern crate net2;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use nix::sys::socket;
use net2::TcpBuilder;

fn main() {
    let unbound_socket = TcpBuilder::new_v4().unwrap();
    println!(
        "### MPTCP Enabled:\n {:#?}",
        socket::getsockopt(unbound_socket.as_raw_fd(), socket::sockopt::MptcpEnabled).unwrap()
    );

    let result = socket::setsockopt(unbound_socket.as_raw_fd(), socket::sockopt::MptcpEnabled, &true);
    println!("### Setting: {:?}", result);

    println!(
        "### MPTCP Enabled:\n {:#?}",
        socket::getsockopt(unbound_socket.as_raw_fd(), socket::sockopt::MptcpEnabled).unwrap()
    );

    let listener = unbound_socket.bind("0.0.0.0:80").expect("Failed to bind").listen(5000).expect("Failed to start listening");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("NEW CONNECTION");
                output_info(stream)
            }
            Err(e) => println!("{:?}", e),
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

    println!(
        "### MPTCP Enabled:\n {:#?}",
        socket::getsockopt(stream.as_raw_fd(), socket::sockopt::MptcpEnabled).unwrap()
    );
}

use std::net::UdpSocket;
use std::str;
use std::thread;

fn send_str(msg: &str, address: &str) -> usize {
    let bind_result = UdpSocket::bind("127.0.0.1:51112"); // src
    let socket = match bind_result {
        Ok(v) => v,
        Err(e) => panic!("OS can't assign a port. Detail: {}", e),
    };
    let send_result = socket.send_to(msg.as_bytes(), address);
    return match send_result {
        Ok(v) => v,
        Err(_e) => 0,
    };
}

fn recv_str() -> String {
    let bind_result = UdpSocket::bind("127.0.0.1:51111"); // rcv
    let socket = match bind_result {
        Ok(v) => v,
        Err(e) => panic!("OS can't assign this port. Detail: {}", e),
    };
    let mut buf: Box<[u8; 100]> = Box::new([0; 100]);
    let recv_result = socket.recv_from(&mut buf[..]);
    return match recv_result {
        Ok(_v) => {
            let str_recv = match str::from_utf8(&buf[..]) {
                Ok(_vv) => String::from(_vv),
                Err(_ee) => String::from(""),
            };
            String::from(str_recv)
        }
        Err(_e) => String::from(""),
    };
}

fn main() {
    thread::spawn(|| {
        loop {
            let received = recv_str();
            println!("Re: {}", received);
        }
    });
    use std::io::stdin;
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        send_str(&s, "127.0.0.1:60582");
        s.clear();
    }
}

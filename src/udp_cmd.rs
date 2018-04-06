use std::thread;
mod udp_lib;

fn main() {
    thread::spawn(|| {
        loop {
            let received = udp_lib::recv_str();
            println!("Re: {}", received);
        }
    });
    use std::io::stdin;
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        udp_lib::send_str(&s, "127.0.0.1:59970");
        s.clear();
    }
}

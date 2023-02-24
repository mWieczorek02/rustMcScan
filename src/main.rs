#[path = "./mod/address_list.rs"]
mod address_list;

use ipnet::Ipv4Net;

// use std::io::prelude::*;

// use std::sync::mpsc::channel;

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use packed_struct::prelude::*;

use address_list::get_address_list;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);
fn main() {
    struct TestPack {
        protocol_version: i32,
        server_address: String,
        server_port: u8,
        NewtState: i32,
    }

    use std::time::Instant;
    let now = Instant::now();

    let mut stream = TcpStream::connect("206.189.10.214:25565").unwrap();

    println!("{:?}", stream);

    stream.write(&[1]);
    let data = stream.read(&mut [0; 128]).unwrap();

    println!("{:?}", data);

    // get_address_list().iter().for_each(|address| {
    //     let net4: Ipv4Net = address.parse().unwrap();
    //     net4.hosts().for_each(|host| {
    //         GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    //         thread::spawn(move || {
    //             let stream = TcpStream::connect_timeout(
    //                 &SocketAddr::new(IpAddr::V4(host), 25565),
    //                 Duration::new(10, 0),
    //             );

    //             // let stream = TcpStream::connect(host.to_string() + ":" + &25565.to_string());

    //             // stream.write(&[1]);
    //             // stream.read(&mut [0; 128]);
    //             match &stream {
    //                 Ok(ok) => println!("stream {:?} - host {:?}!!", ok, host),
    //                 Err(_) => print!(""),
    //             }
    //             GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
    //         });
    //     });
    // });

    // while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
    //     thread::sleep(Duration::from_millis(1));
    // }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
